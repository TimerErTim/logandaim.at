//! One case per acceptance criterion of SPEC-konto, driven over HTTP.
//!
//! The core suite (`logandaim-core/tests/cases/konto.rs`) already proves the *rules* against
//! both adapters. This suite proves the *edge*: the cookie attributes, the status codes, the
//! `Authorization` header, and — for AC-19 — a mail that really travelled over SMTP. A rule
//! that only holds because a use case was called directly does not reach a client.
//!
//! AC-7a, AC-17, and AC-18 only exist at this layer (a cookie header, a session row's public
//! key, a clock past `exp`), so they have no counterpart in the core cases.

use chrono::Duration;
use logandaim_core::domain::konto::ACCESS_TOKEN_LIFETIME;
use poem::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use super::{
    BESTÄTIGUNG_LINK, E2eHarness, FRONTEND_BASIS_URL, PASSWORT, WIEDERHERSTELLUNG_LINK,
    cookie_pair, mailpit,
};

/// The refresh cookie a client keeps.
fn refresh_cookie(set_cookie: &str) -> String {
    assert!(
        set_cookie.starts_with("__Host-logandaim_refresh="),
        "unerwartetes Cookie: {set_cookie}"
    );
    cookie_pair(set_cookie)
}

/// Register fully and return `(access_token, refresh_cookie, benutzer_id, email)`.
async fn registriere(harness: &E2eHarness, email: &str) -> (String, String, String, String) {
    let (status, _) = harness
        .client
        .post_json("/api/registrierung/start", json!({ "eMailAdresse": email }))
        .await;
    assert_eq!(status, StatusCode::OK);

    let token = harness.letztes_token();
    let (status, body, cookie) = harness
        .client
        .post_json_cookie(
            "/api/registrierung/abschluss",
            json!({ "token": token, "passwort": PASSWORT }),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "abschluss: {body}");

    let access = body["accessToken"]
        .as_str()
        .expect("accessToken im Body")
        .to_owned();
    let cookie = refresh_cookie(&cookie.expect("Set-Cookie beim Abschluss"));
    let (_, benutzer) = harness.client.get_json_auth("/api/benutzer", &access).await;
    let benutzer_id = benutzer["id"].as_str().expect("id im Body").to_owned();
    (access, cookie, benutzer_id, email.to_owned())
}

// ------------------------------------------------------------------- AC-1, AC-2, AC-20

#[test_log::test(pollster::test)]
async fn ac1_registrierung_startet_ohne_benutzer_und_mailt_genau_einmal() {
    let harness = E2eHarness::new().await;

    let (status, _) = harness
        .client
        .post_json(
            "/api/registrierung/start",
            json!({ "eMailAdresse": "anna@muster.at" }),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(harness.mail.sent().len(), 1);

    // AC-20: no Benutzer exists before step 2, so an unknown address cannot log in.
    let (status, _) = harness
        .client
        .post_json(
            "/api/anmeldung",
            json!({ "eMailAdresse": "anna@muster.at", "passwort": PASSWORT }),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[test_log::test(pollster::test)]
async fn ac2_vergebene_adresse_antwortet_gleich_und_mailt_nicht() {
    let harness = E2eHarness::new().await;
    registriere(&harness, "anna@muster.at").await;
    harness.mail.clear();

    let (frei_status, frei_body) = harness
        .client
        .post_json(
            "/api/registrierung/start",
            json!({ "eMailAdresse": "neu@muster.at" }),
        )
        .await;
    let (belegt_status, belegt_body) = harness
        .client
        .post_json(
            "/api/registrierung/start",
            json!({ "eMailAdresse": "anna@muster.at" }),
        )
        .await;

    assert_eq!(frei_status, belegt_status, "derselbe Status");
    assert_eq!(frei_body, belegt_body, "derselbe Body");
    // Only the free address was mailed.
    assert_eq!(harness.mail.sent().len(), 1);
    assert_eq!(
        harness.mail.latest().empfänger.as_ref(),
        "neu@muster.at",
        "die vergebene Adresse bekommt keine Mail"
    );
}

// ------------------------------------------------------------------------------- AC-3

#[test_log::test(pollster::test)]
async fn ac3_abschluss_erzeugt_benutzer_konto_und_erste_session() {
    let harness = E2eHarness::new().await;
    let (access, cookie, benutzer_id, _) = registriere(&harness, "anna@muster.at").await;

    assert!(!access.is_empty());
    assert!(!benutzer_id.is_empty());
    assert!(Uuid::parse_str(&benutzer_id).is_ok());
    assert!(!cookie.is_empty());

    let (status, body) = harness.client.get_json_auth("/api/benutzer", &access).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(body["eMailAdresse"], "anna@muster.at");
}

// ------------------------------------------------------------------------- AC-4, AC-5

#[test_log::test(pollster::test)]
async fn ac4_kurzes_passwort_oder_ungültiges_token_erzeugt_nichts() {
    let harness = E2eHarness::new().await;
    harness
        .client
        .post_json(
            "/api/registrierung/start",
            json!({ "eMailAdresse": "anna@muster.at" }),
        )
        .await;
    let token = harness.letztes_token();

    let (status, _) = harness
        .client
        .post_json(
            "/api/registrierung/abschluss",
            json!({ "token": token, "passwort": "kurz" }),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "zu kurzes Passwort");

    let (status, _) = harness
        .client
        .post_json(
            "/api/registrierung/abschluss",
            json!({ "token": "völlig-erfunden", "passwort": PASSWORT }),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "unbekanntes Token");

    // The address is still free: nothing was created by either attempt.
    let (status, _) = harness
        .client
        .post_json(
            "/api/registrierung/abschluss",
            json!({ "token": token, "passwort": PASSWORT }),
        )
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "das gültige Token wurde nicht verbraucht"
    );
}

#[test_log::test(pollster::test)]
async fn ac5_anmeldung_eröffnet_eine_gültige_session() {
    let harness = E2eHarness::new().await;
    registriere(&harness, "anna@muster.at").await;

    let (status, body, cookie) = harness
        .client
        .post_json_cookie(
            "/api/anmeldung",
            json!({ "eMailAdresse": "anna@muster.at", "passwort": PASSWORT }),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");

    let access = body["accessToken"].as_str().expect("accessToken");
    let cookie = cookie.expect("Set-Cookie");

    let (status, body) = harness.client.get_json_auth("/api/benutzer", access).await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert!(!refresh_cookie(&cookie).is_empty());
}

// ------------------------------------------------------------------------------- AC-6

#[test_log::test(pollster::test)]
async fn ac6_falsches_passwort_und_unbekannte_adresse_sind_ununterscheidbar() {
    let harness = E2eHarness::new().await;
    registriere(&harness, "anna@muster.at").await;

    let (falsch_status, falsch_body) = harness
        .client
        .post_json(
            "/api/anmeldung",
            json!({ "eMailAdresse": "anna@muster.at", "passwort": "falsch-falsch" }),
        )
        .await;
    let (unbekannt_status, unbekannt_body) = harness
        .client
        .post_json(
            "/api/anmeldung",
            json!({ "eMailAdresse": "niemand@muster.at", "passwort": PASSWORT }),
        )
        .await;

    assert_eq!(falsch_status, StatusCode::UNAUTHORIZED);
    assert_eq!(unbekannt_status, StatusCode::UNAUTHORIZED);
    assert_eq!(
        falsch_body["message"], unbekannt_body["message"],
        "die Meldung darf den Unterschied nicht verraten"
    );
}

// ------------------------------------------------------------------------ AC-7, AC-7a

#[test_log::test(pollster::test)]
async fn ac7_erneuerung_rotiert_und_das_alte_refresh_token_ist_verbraucht() {
    let harness = E2eHarness::new().await;
    let (_, erster_cookie, _, _) = registriere(&harness, "anna@muster.at").await;

    let (status, body, neuer_cookie) = harness
        .client
        .post_cookie("/api/session/erneuern", Some(&erster_cookie))
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let neuer_cookie = refresh_cookie(&neuer_cookie.expect("Set-Cookie bei Erneuerung"));
    assert_ne!(neuer_cookie, erster_cookie, "das Token rotiert");
    assert!(body["accessToken"].as_str().is_some_and(|t| !t.is_empty()));

    // The spent token is refused, and so is the successor (the family is ended, rule 12).
    let (status, _, _) = harness
        .client
        .post_cookie("/api/session/erneuern", Some(&erster_cookie))
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "Wiederverwendung");

    let (status, _, _) = harness
        .client
        .post_cookie("/api/session/erneuern", Some(&neuer_cookie))
        .await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "der Diebstahl beendet auch die Folgesitzung"
    );
}

/// AC-7a: the refresh token leaves only as a cookie, the access token only in the body.
#[test_log::test(pollster::test)]
async fn ac7a_access_token_steht_nie_in_einem_cookie() {
    let harness = E2eHarness::new().await;

    harness
        .client
        .post_json(
            "/api/registrierung/start",
            json!({ "eMailAdresse": "anna@muster.at" }),
        )
        .await;
    let token = harness.letztes_token();

    let (status, body, set_cookie) = harness
        .client
        .post_json_cookie(
            "/api/registrierung/abschluss",
            json!({ "token": token, "passwort": PASSWORT }),
            None,
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    let set_cookie = set_cookie.expect("Set-Cookie");
    let access = body["accessToken"].as_str().expect("accessToken");

    // The header names exactly one cookie, and it is the refresh cookie.
    assert_eq!(set_cookie.matches("__Host-logandaim_refresh=").count(), 1);
    assert!(
        !set_cookie.contains(access),
        "das Access-Token darf nicht im Cookie stehen: {set_cookie}"
    );

    // Every documented attribute is present. A missing one is a browser ignoring the
    // cookie, or honouring it on a cross-site request.
    for attribut in [
        "Path=/",
        "HttpOnly",
        "Secure",
        "SameSite=Lax",
        "Max-Age=2592000",
    ] {
        assert!(
            set_cookie.contains(attribut),
            "{attribut} fehlt: {set_cookie}"
        );
    }
    assert!(
        !set_cookie.contains("Domain"),
        "der __Host- Präfix verbietet Domain"
    );

    // And the body carries no refresh token: only the two documented fields.
    let objekt = body.as_object().expect("ein JSON-Objekt");
    assert_eq!(
        objekt.keys().collect::<Vec<_>>(),
        vec!["accessToken"],
        "kein weiteres Feld im Body"
    );
}

// ------------------------------------------------------------------------------- AC-8

#[test_log::test(pollster::test)]
async fn ac8_abgelaufenes_refresh_token_wird_abgelehnt() {
    let harness = E2eHarness::new().await;
    let (_, cookie, _, _) = registriere(&harness, "anna@muster.at").await;

    // Past REFRESH_TOKEN_LIFETIME (30 days).
    harness.vorspulen(Duration::days(31));

    let (status, _, neuer_cookie) = harness
        .client
        .post_cookie("/api/session/erneuern", Some(&cookie))
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let neuer_cookie = neuer_cookie.expect("auch ein Fehler löscht das Cookie");
    assert!(
        neuer_cookie.contains("Max-Age=0"),
        "das abgelaufene Cookie wird gelöscht: {neuer_cookie}"
    );
}

// ------------------------------------------------------------------------------- AC-9

#[test_log::test(pollster::test)]
async fn ac9_abmeldung_widerruft_und_das_access_token_fällt_sofort() {
    let harness = E2eHarness::new().await;
    let (access, cookie, _, _) = registriere(&harness, "anna@muster.at").await;

    let (status, _) = harness.client.get_json_auth("/api/benutzer", &access).await;
    assert_eq!(status, StatusCode::OK);

    // Logout names the session through the access token; the refresh cookie is what the
    // response clears.
    let (status, _, set_cookie) = harness
        .client
        .post_cookie_auth("/api/session/abmelden", Some(&cookie), &access)
        .await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        set_cookie
            .expect("Logout löscht das Cookie")
            .contains("Max-Age=0")
    );

    // The token is unexpired and correctly signed, but its session row is revoked.
    let (status, _) = harness.client.get_json_auth("/api/benutzer", &access).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "sofort ungültig");
}

// ------------------------------------------------------------------------------ AC-10

#[test_log::test(pollster::test)]
async fn ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich() {
    let harness = E2eHarness::new().await;
    registriere(&harness, "anna@muster.at").await;
    harness.mail.clear();

    let (bekannt_status, bekannt_body) = harness
        .client
        .post_json(
            "/api/passwort/vergessen",
            json!({ "eMailAdresse": "anna@muster.at" }),
        )
        .await;
    let (unbekannt_status, unbekannt_body) = harness
        .client
        .post_json(
            "/api/passwort/vergessen",
            json!({ "eMailAdresse": "niemand@muster.at" }),
        )
        .await;

    assert_eq!(bekannt_status, StatusCode::OK);
    assert_eq!(unbekannt_status, StatusCode::OK);
    assert_eq!(bekannt_body, unbekannt_body);
    assert_eq!(harness.mail.sent().len(), 1, "nur die bekannte Adresse");
    assert!(
        harness
            .mail
            .latest()
            .text(FRONTEND_BASIS_URL)
            .contains(WIEDERHERSTELLUNG_LINK),
        "der Link zeigt auf die Wiederherstellungsroute"
    );
}

// ------------------------------------------------------------------------------ AC-11

#[test_log::test(pollster::test)]
async fn ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig() {
    let harness = E2eHarness::new().await;
    let (erster_access, _, _, _) = registriere(&harness, "anna@muster.at").await;
    let (zweiter_access, _, _, _) = {
        let (status, body) = harness
            .client
            .post_json(
                "/api/anmeldung",
                json!({ "eMailAdresse": "anna@muster.at", "passwort": PASSWORT }),
            )
            .await;
        assert_eq!(status, StatusCode::OK);
        (body["accessToken"].as_str().unwrap().to_owned(), (), (), ())
    };

    harness
        .client
        .post_json(
            "/api/passwort/vergessen",
            json!({ "eMailAdresse": "anna@muster.at" }),
        )
        .await;
    let token = harness.letztes_token();

    let neues_passwort = "noch-besser-pferd-batterie";
    let (status, _) = harness
        .client
        .post_json(
            "/api/passwort/wiederherstellen",
            json!({ "token": token, "neuesPasswort": neues_passwort }),
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    // Every prior session is gone, and so are both old Passwörter.
    for access in [&erster_access, &zweiter_access] {
        let (status, _) = harness.client.get_json_auth("/api/benutzer", access).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
    let (status, _) = harness
        .client
        .post_json(
            "/api/anmeldung",
            json!({ "eMailAdresse": "anna@muster.at", "passwort": PASSWORT }),
        )
        .await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "das alte Passwort gilt nicht"
    );

    let (status, _) = harness
        .client
        .post_json(
            "/api/anmeldung",
            json!({ "eMailAdresse": "anna@muster.at", "passwort": neues_passwort }),
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    // Single use: the same token is refused now.
    let (status, _) = harness
        .client
        .post_json(
            "/api/passwort/wiederherstellen",
            json!({ "token": token, "neuesPasswort": "drittes-passwort-123" }),
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// ------------------------------------------------------------------------------ AC-12

#[test_log::test(pollster::test)]
async fn ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei() {
    let harness = E2eHarness::new().await;
    let (access, _, _, _) = registriere(&harness, "anna@muster.at").await;

    let (status, _) = harness
        .client
        .post_json(
            "/api/benutzer/löschung",
            json!({ "passwort": PASSWORT, "bestätigt": true }),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "ohne Anmeldung");

    let (status, _) = harness.client.get_json_auth("/api/benutzer", &access).await;
    assert_eq!(status, StatusCode::OK, "noch nicht gelöscht");

    // Delete properly, with the bearer token.
    let (status, _) = harness
        .client
        .post_json_auth(
            "/api/benutzer/löschung",
            json!({ "passwort": PASSWORT, "bestätigt": true }),
            &access,
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    let (status, _) = harness.client.get_json_auth("/api/benutzer", &access).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "die Identität ist weg");

    let (status, _) = harness
        .client
        .post_json(
            "/api/anmeldung",
            json!({ "eMailAdresse": "anna@muster.at", "passwort": PASSWORT }),
        )
        .await;
    assert_eq!(
        status,
        StatusCode::UNAUTHORIZED,
        "die alten Zugangsdaten gelten nicht"
    );

    // AC-12: the address is free again.
    registriere(&harness, "anna@muster.at").await;
}

#[test_log::test(pollster::test)]
async fn ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert() {
    let harness = E2eHarness::new().await;
    let (access, _, _, _) = registriere(&harness, "anna@muster.at").await;

    let (status, _) = harness
        .client
        .post_json_auth(
            "/api/benutzer/löschung",
            json!({ "passwort": PASSWORT, "bestätigt": false }),
            &access,
        )
        .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "ohne Bestätigung");

    let (status, _) = harness
        .client
        .post_json_auth(
            "/api/benutzer/löschung",
            json!({ "passwort": "falsch-falsch", "bestätigt": true }),
            &access,
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "falsches Passwort");

    let (status, _) = harness.client.get_json_auth("/api/benutzer", &access).await;
    assert_eq!(status, StatusCode::OK, "der Benutzer existiert weiterhin");
}

// ------------------------------------------------------------------------------ AC-13

#[test_log::test(pollster::test)]
async fn ac13_anonymer_zugriff_wird_verweigert() {
    let harness = E2eHarness::new().await;

    let (status, _) = harness.client.get_json("/api/benutzer").await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, _) = harness.client.post("/api/session/abmelden").await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    // A malformed header is a refusal, not a silent downgrade to anonymous.
    let (status, _) = harness
        .client
        .get_json_auth("/api/benutzer", "kein-jwt")
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

// ------------------------------------------------------------------------------ AC-14

#[test_log::test(pollster::test)]
async fn ac14_fremden_benutzer_liest_nur_ein_systemadministrator() {
    let harness = E2eHarness::new().await;
    let (anna_access, _, _, _) = registriere(&harness, "anna@muster.at").await;
    let (bert_access, _, bert_id, _) = registriere(&harness, "bert@muster.at").await;

    // Anna asks for Bert: refused. Not 404 — she proved who she is and is simply not
    // allowed, and pretending Bert does not exist would confuse an administrator.
    let (status, _) = harness
        .client
        .get_json_auth(&format!("/api/benutzer?benutzerId={bert_id}"), &anna_access)
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);

    // Her own record always works.
    let (status, body) = harness
        .client
        .get_json_auth("/api/benutzer", &anna_access)
        .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["eMailAdresse"], "anna@muster.at");

    let _ = bert_access;
}

// ------------------------------------------------------------------------ AC-15, AC-16

#[test_log::test(pollster::test)]
async fn ac16_registrierung_ist_auf_fünf_pro_stunde_gedrosselt() {
    let harness = E2eHarness::new().await;

    for i in 0..5 {
        let (status, _) = harness
            .client
            .post_json(
                "/api/registrierung/start",
                json!({ "eMailAdresse": format!("nr{i}@muster.at") }),
            )
            .await;
        assert_eq!(status, StatusCode::OK, "Versuch {i} liegt im Budget");
    }

    let (status, _) = harness
        .client
        .post_json(
            "/api/registrierung/start",
            json!({ "eMailAdresse": "zuviel@muster.at" }),
        )
        .await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
}

#[test_log::test(pollster::test)]
async fn ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client() {
    let harness = E2eHarness::new().await;

    for i in 0..20 {
        let (status, _) = harness
            .client
            .post_json(
                "/api/anmeldung",
                json!({ "eMailAdresse": "niemand@muster.at", "passwort": PASSWORT }),
            )
            .await;
        assert_eq!(
            status,
            StatusCode::UNAUTHORIZED,
            "Versuch {i} ist ein Zugangsfehler, keine Drosselung"
        );
    }

    let (status, _) = harness
        .client
        .post_json(
            "/api/anmeldung",
            json!({ "eMailAdresse": "niemand@muster.at", "passwort": PASSWORT }),
        )
        .await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
}

// ------------------------------------------------------------------ AC-17, AC-18, AC-19

/// AC-17: an access token is checked against the public key stored on its own session row.
///
/// Two failures, both 401: an unknown session, and a signature that does not verify with
/// the key that row carries — even though the envelope is otherwise valid.
#[test_log::test(pollster::test)]
async fn ac17_signatur_wird_gegen_den_schlüssel_der_session_zeile_geprüft() {
    let harness = E2eHarness::new().await;
    let (access, _, benutzer_id, _) = registriere(&harness, "anna@muster.at").await;

    let (status, _) = harness.client.get_json_auth("/api/benutzer", &access).await;
    assert_eq!(status, StatusCode::OK);

    // Take the live token apart and re-sign its claims with different key material. The
    // envelope is valid, so only the row's public key can reject it.
    let gelesen = logandaim_api::jwt::decode(&access).expect("unser Token parst");
    let fremd = logandaim_core::adapters::InMemorySigning::from_secret_bytes(&[9u8; 32]);
    let gefälscht = logandaim_api::jwt::encode(&gelesen.claims, &fremd).expect("encoding succeeds");

    let (status, _) = harness
        .client
        .get_json_auth("/api/benutzer", gefälscht.as_str())
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "fremde Signatur");

    // An unknown session id (`sid`) is refused just as flatly: no row, no identity.
    let mut unbekannt = gelesen.claims.clone();
    unbekannt.sid = logandaim_core::domain::konto::SessionId(Uuid::nil());
    let token = logandaim_api::jwt::encode(&unbekannt, &super::test_signing_key())
        .expect("encoding succeeds");
    let (status, _) = harness
        .client
        .get_json_auth("/api/benutzer", token.as_str())
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "unbekannte Sitzung");
    assert!(!benutzer_id.is_empty());
}

/// AC-17: a revoked session fails even though its signature still verifies.
#[test_log::test(pollster::test)]
async fn ac17_widerrufene_session_wird_abgelehnt() {
    let harness = E2eHarness::new().await;
    let (access, cookie, _, _) = registriere(&harness, "anna@muster.at").await;

    let (status, _) = harness.client.get_json_auth("/api/benutzer", &access).await;
    assert_eq!(status, StatusCode::OK);

    // Logout names its session through the access token; the cookie is only cleared.
    let (status, _, _) = harness
        .client
        .post_cookie_auth("/api/session/abmelden", Some(&cookie), &access)
        .await;
    assert_eq!(status, StatusCode::OK);

    let (status, _) = harness.client.get_json_auth("/api/benutzer", &access).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "die Sitzung ist beendet");
}

/// AC-18: a correctly signed, unrevoked token whose `exp` has passed is refused.
#[test_log::test(pollster::test)]
async fn ac18_abgelaufenes_access_token_wird_abgelehnt() {
    let harness = E2eHarness::new().await;
    let (access, _, _, _) = registriere(&harness, "anna@muster.at").await;

    let (status, _) = harness.client.get_json_auth("/api/benutzer", &access).await;
    assert_eq!(status, StatusCode::OK);

    // Past ACCESS_TOKEN_LIFETIME (24 h). The session row is still live, so only `exp` can
    // be the reason.
    harness.vorspulen(ACCESS_TOKEN_LIFETIME + Duration::minutes(5));

    let (status, _) = harness.client.get_json_auth("/api/benutzer", &access).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "exp ist abgelaufen");
}

/// AC-19 over a real Mailpit: the link is callable and carries the configured route.
///
/// On Tokio, not `pollster`: the SMTP adapter resolves its relay through `tokio::net`, and
/// a pollster-driven future has no reactor to do that on.
#[test_log::test(tokio::test)]
async fn ac19_die_mail_trägt_einen_aufrufbaren_link() {
    mailpit::leeren().await;

    let harness = E2eHarness::mit_smtp().await;
    harness
        .client
        .post_json(
            "/api/registrierung/start",
            json!({ "eMailAdresse": "anna@muster.at" }),
        )
        .await;

    let text = mailpit::letzter_text_an("anna@muster.at").await;
    assert!(
        text.contains(&format!("{BESTÄTIGUNG_LINK}?token=")),
        "die Mail trägt den konfigurierten Frontend-Link: {text}"
    );

    let token = mailpit::token_aus(&text);
    let (status, body) = harness
        .client
        .post_json(
            "/api/registrierung/abschluss",
            json!({ "token": token, "passwort": PASSWORT }),
        )
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "der gemailte Link funktioniert: {body}"
    );
}
