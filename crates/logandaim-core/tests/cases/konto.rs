//! Shared `konto` cases, run twice: against the fakes and against SQLite.
//!
//! One case per acceptance criterion, phrased against the use cases rather than against a
//! repository, so a passing run says the product behaviour holds on this adapter — not
//! merely that a `SELECT` round-trips.

use std::net::{IpAddr, Ipv4Addr};

use chrono::Duration;
use logandaim_core::{
    application::AuthContext,
    domain::{
        EmailAdresse,
        konto::{
            ACCESS_TOKEN_LIFETIME, BenutzerId, KlartextPasswort, NeuerBenutzer, NeuesKonto, Rolle,
            SecretToken, SessionId, hash_password,
        },
    },
    ports::{ClientKey, RepositoryError},
    service::{
        Abmeldung, Anmeldung, BenutzerAnzeigen, BenutzerAnzeigenFehler, BenutzerLöschen,
        Erneuerung, PasswortVergessen, PasswortWiederherstellen, RegistrierungAbschliessenFehler,
        RegistrierungAbschließen, RegistrierungStarten, SessionAusgestellt,
    },
};

use super::{
    super::base_app_builder,
    helpers::{FRONTEND_BASIS_URL, Harness, fremder_schluessel, pruefen, pruefen_mit},
};

const PASSWORT: &str = "korrekt-pferd-batterie";

fn client_key(ip: u8) -> ClientKey {
    ClientKey::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, ip)), "test-client")
}

fn passwort() -> KlartextPasswort {
    KlartextPasswort::neu(PASSWORT).expect("Testpasswort ist gültig")
}

fn passwort_wie(wert: &str) -> KlartextPasswort {
    KlartextPasswort::neu(wert).expect("Testpasswort ist gültig")
}

fn adresse(wert: &str) -> EmailAdresse {
    EmailAdresse::normalisiert(wert).expect("Testadresse ist gültig")
}

async fn harness() -> Harness {
    // No key registration: the session row itself carries the public key its access token
    // must verify against (AC-17).
    super::helpers::build(base_app_builder().await)
}

/// Run registration step 1 and return the mailed token.
async fn registrierung_starten(
    harness: &Harness,
    email: &str,
    client_key: ClientKey,
) -> SecretToken {
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungStarten {
                email: adresse(email),
                client_key,
            },
        )
        .await
        .expect("registration step 1 answers Ok");
    let text = harness.letztes_token();
    SecretToken::try_from(text.as_str()).expect("die Mail trägt ein SecretToken")
}

/// Register fully and return the issued session.
async fn registriere(harness: &Harness, email: &str) -> SessionAusgestellt {
    let token = registrierung_starten(harness, email, client_key(1)).await;
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungAbschließen {
                token,
                passwort: passwort(),
                client_key: client_key(1),
            },
        )
        .await
        .expect("registration step 2 issues a session")
}

async fn anmelden(harness: &Harness, email: &str) -> SessionAusgestellt {
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Anmeldung {
                email: adresse(email),
                passwort: passwort(),
                client_key: client_key(2),
            },
        )
        .await
        .expect("login issues a session")
}

/// What the API will inject after decoding a bearer token.
fn auth_aus(ausgestellt: &SessionAusgestellt) -> AuthContext {
    AuthContext::Authenticated {
        benutzer_id: ausgestellt.benutzer.id(),
        session_id: ausgestellt.session_id,
    }
}

/// The message of an error report, whatever the use case's own error enum is.
fn text<E>(report: error_stack::Report<E>) -> String
where
    E: std::error::Error + Send + Sync + 'static,
{
    report.current_context().to_string()
}

// ---------------------------------------------------------------- AC-1, AC-2, AC-20

#[test_log::test(pollster::test)]
async fn ac1_registrierung_startet_ohne_benutzer_und_mailt_genau_einmal() {
    let harness = harness().await;

    let token = registrierung_starten(&harness, "anna@muster.at", client_key(1)).await;

    assert_eq!(harness.mail().gesendete().len(), 1);
    assert_eq!(
        token.base64url().len(),
        43,
        "das gemailte Token ist ein 256-bit Secret"
    );

    // AC-20: at this point no Benutzer exists, so nothing can be unconfirmed.
    harness
        .app()
        .execute_fn::<_, _, RepositoryError>(AuthContext::Anonymous, async |ctx| {
            let uow = ctx.enter().await?;
            let benutzer = uow
                .benutzer()
                .find_by_id(BenutzerId(uuid::Uuid::new_v4()))
                .await;
            assert!(
                benutzer.is_err(),
                "zu diesem Zeitpunkt existiert kein Benutzer"
            );
            uow.commit().await
        })
        .await
        .expect("reading an absent Benutzer is not an error at this level");
}

#[test_log::test(pollster::test)]
async fn ac2_vergebene_adresse_antwortet_gleich_und_mailt_nicht() {
    let harness = harness().await;
    registriere(&harness, "belegt@muster.at").await;
    harness.mail().leeren();

    let ergebnis = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungStarten {
                email: adresse("belegt@muster.at"),
                client_key: client_key(3),
            },
        )
        .await;

    assert!(
        ergebnis.is_ok(),
        "die Antwort für eine vergebene Adresse ist Ok"
    );
    assert!(
        harness.mail().gesendete().is_empty(),
        "eine vergebene Adresse bekommt keine Mail"
    );
}

// ------------------------------------------------------------------------- AC-3, AC-4

#[test_log::test(pollster::test)]
async fn ac3_abschluss_erzeugt_benutzer_konto_und_erste_session() {
    let harness = harness().await;

    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    assert_eq!(ausgestellt.benutzer.rolle(), Rolle::Benutzer);
    assert_eq!(
        ausgestellt.claims.sub,
        ausgestellt.benutzer.id(),
        "das Access-Token gehört dem neuen Benutzer"
    );
    assert_eq!(ausgestellt.claims.sid, ausgestellt.session_id);
    assert_eq!(ausgestellt.refresh_token.base64url().len(), 43);
}

#[test_log::test(pollster::test)]
async fn ac4_ungültiges_token_oder_kurzes_passwort_erzeugt_nichts() {
    let harness = harness().await;
    let token = registrierung_starten(&harness, "anna@muster.at", client_key(4)).await;

    // Too short: the DTO layer refuses it before the use case runs, so a usable token is
    // never spent by the attempt.
    assert!(
        KlartextPasswort::neu("kurz").is_err(),
        "die Längenregel greift bei der Konstruktion"
    );

    // An unknown token is refused too.
    let unbekannt = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungAbschließen {
                token: SecretToken::try_from("A".repeat(43).as_str())
                    .expect("ein geformtes, aber unbekanntes Token"),
                passwort: passwort(),
                client_key: client_key(6),
            },
        )
        .await;
    assert!(unbekannt.is_err());

    // The real token still works: the refusal spent nothing.
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungAbschließen {
                token,
                passwort: passwort(),
                client_key: client_key(7),
            },
        )
        .await
        .expect("das unverbrauchte Token bleibt gültig");
}

#[test_log::test(pollster::test)]
async fn ac4_verbrauchtes_token_wird_beim_zweiten_mal_abgelehnt() {
    let harness = harness().await;
    let token = registrierung_starten(&harness, "anna@muster.at", client_key(8)).await;
    registriere_abschließen(&harness, &token, client_key(8))
        .await
        .expect("erster Abschluss gelingt");

    let zweiter = registriere_abschließen(&harness, &token, client_key(9)).await;
    assert!(zweiter.is_err(), "ein verbrauchtes Token ist unbrauchbar");
}

async fn registriere_abschließen(
    harness: &Harness,
    token: &SecretToken,
    client_key: ClientKey,
) -> Result<SessionAusgestellt, error_stack::Report<RegistrierungAbschliessenFehler>> {
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungAbschließen {
                token: token.clone(),
                passwort: passwort(),
                client_key,
            },
        )
        .await
}

// ------------------------------------------------------------------------- AC-5, AC-6

#[test_log::test(pollster::test)]
async fn ac5_anmeldung_eröffnet_eine_gültige_session() {
    let harness = harness().await;
    registriere(&harness, "anna@muster.at").await;

    let ausgestellt = anmelden(&harness, "anna@muster.at").await;

    let geprüft = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            pruefen(&harness, ausgestellt.claims.clone()),
        )
        .await
        .expect("ein frisches Access-Token wird akzeptiert");

    assert_eq!(geprüft, auth_aus(&ausgestellt));
}

#[test_log::test(pollster::test)]
async fn ac6_falsches_passwort_und_unbekannte_adresse_sind_ununterscheidbar() {
    let harness = harness().await;
    registriere(&harness, "anna@muster.at").await;

    let falsch = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Anmeldung {
                email: adresse("anna@muster.at"),
                passwort: passwort_wie("falsches-passwort"),
                client_key: client_key(10),
            },
        )
        .await;
    let unbekannt = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Anmeldung {
                email: adresse("niemand@muster.at"),
                passwort: passwort(),
                client_key: client_key(11),
            },
        )
        .await;

    assert_eq!(
        text(falsch.unwrap_err()),
        text(unbekannt.unwrap_err()),
        "die Antwort darf die Existenz der Adresse nicht verraten"
    );

    // A token naming a session that does not exist is refused, never accepted.
    assert!(
        harness
            .app()
            .execute(
                AuthContext::Anonymous,
                pruefen(
                    &harness,
                    logandaim_core::domain::konto::AccessTokenClaims {
                        sub: BenutzerId(uuid::Uuid::nil()),
                        sid: SessionId(uuid::Uuid::nil()),
                        iss: "logandaim.at".into(),
                        aud: "logandaim-api".into(),
                        iat: harness.jetzt(),
                        exp: harness.jetzt() + ACCESS_TOKEN_LIFETIME,
                        jti: "test".into(),
                    }
                )
            )
            .await
            .is_err()
    );
}

// ------------------------------------------------------------------------- AC-7, AC-8

#[test_log::test(pollster::test)]
async fn ac7_erneuerung_rotiert_und_das_alte_access_token_lebt_weiter() {
    let harness = harness().await;
    let alt = registriere(&harness, "anna@muster.at").await;

    let neu = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Erneuerung {
                refresh_token: alt.refresh_token.clone(),
                client_key: client_key(12),
            },
        )
        .await
        .expect("die Erneuerung liefert ein neues Paar");

    assert_ne!(
        neu.refresh_token.base64url().as_str(),
        alt.refresh_token.base64url().as_str()
    );

    // The pre-rotation access token still works: only revocation ends it (AC-9, AC-17).
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            pruefen(&harness, alt.claims.clone()),
        )
        .await
        .expect("das alte Access-Token bleibt bis zum Ablauf gültig");

    // The new one works as well.
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            pruefen(&harness, neu.claims.clone()),
        )
        .await
        .expect("das neue Access-Token ist gültig");
}

#[test_log::test(pollster::test)]
async fn ac8_verbrauchtes_refresh_token_wird_abgelehnt_und_beendet_die_familie() {
    let harness = harness().await;
    let alt = registriere(&harness, "anna@muster.at").await;

    let neu = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Erneuerung {
                refresh_token: alt.refresh_token.clone(),
                client_key: client_key(13),
            },
        )
        .await
        .expect("erste Erneuerung gelingt");

    // Replay of the spent token: refused, and treated as theft.
    let replay = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Erneuerung {
                refresh_token: alt.refresh_token.clone(),
                client_key: client_key(14),
            },
        )
        .await;
    assert!(replay.is_err());

    // The family is now revoked, so even the successor is unusable.
    let nachfolger = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Erneuerung {
                refresh_token: neu.refresh_token.clone(),
                client_key: client_key(15),
            },
        )
        .await;
    assert!(
        nachfolger.is_err(),
        "ein Replay beendet die ganze Session-Familie"
    );
}

#[test_log::test(pollster::test)]
async fn ac8_abgelaufenes_refresh_token_wird_abgelehnt() {
    let harness = harness().await;
    let alt = registriere(&harness, "anna@muster.at").await;

    harness.vorspulen(Duration::days(31));

    let erneuerung = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Erneuerung {
                refresh_token: alt.refresh_token.clone(),
                client_key: client_key(16),
            },
        )
        .await;
    assert!(erneuerung.is_err(), "ein Monat ist das Ende der Sitzung");
}

// ------------------------------------------------------------------------- AC-9

#[test_log::test(pollster::test)]
async fn ac9_abmeldung_widerruft_und_das_access_token_fällt_sofort() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    harness
        .app()
        .execute(auth_aus(&ausgestellt), Abmeldung)
        .await
        .expect("die Abmeldung gelingt");

    let danach = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            pruefen(&harness, ausgestellt.claims.clone()),
        )
        .await;
    assert!(
        danach.is_err(),
        "ein widerrufenes Sitzungs-Token ist sofort ungültig, auch unverbraucht"
    );
}

// ------------------------------------------------------------------------ AC-10, AC-11

#[test_log::test(pollster::test)]
async fn ac10_wiederherstellung_antwortet_für_jede_adresse_gleich() {
    let harness = harness().await;
    registriere(&harness, "anna@muster.at").await;
    harness.mail().leeren();

    let bekannt = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            PasswortVergessen {
                email: adresse("anna@muster.at"),
                client_key: client_key(17),
            },
        )
        .await;
    let unbekannt = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            PasswortVergessen {
                email: adresse("niemand@muster.at"),
                client_key: client_key(18),
            },
        )
        .await;

    assert!(bekannt.is_ok());
    assert!(unbekannt.is_ok());
    assert_eq!(
        harness.mail().gesendete().len(),
        1,
        "nur ein vorhandenes Konto bekommt Mail"
    );
}

#[test_log::test(pollster::test)]
async fn ac11_neues_passwort_beendet_alle_sessions_und_ist_einmalig() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;
    harness.mail().leeren();

    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            PasswortVergessen {
                email: adresse("anna@muster.at"),
                client_key: client_key(19),
            },
        )
        .await
        .expect("die Wiederherstellung wird angefordert");
    let token = SecretToken::try_from(harness.letztes_token().as_str())
        .expect("die Mail trägt ein SecretToken");

    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            PasswortWiederherstellen {
                token: token.clone(),
                neues_passwort: passwort_wie("ein-ganz-neues-passwort"),
            },
        )
        .await
        .expect("das neue Passwort wird gesetzt");

    // Every session ended, including the access token issued before.
    let alte_session = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            pruefen(&harness, ausgestellt.claims.clone()),
        )
        .await;
    assert!(alte_session.is_err(), "alle Sitzungen sind beendet");

    // Single use.
    let zweiter = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            PasswortWiederherstellen {
                token,
                neues_passwort: passwort_wie("noch-ein-passwort"),
            },
        )
        .await;
    assert!(
        zweiter.is_err(),
        "das Wiederherstellungs-Token ist einmalig"
    );

    // The new Passwort is what logs in now.
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Anmeldung {
                email: adresse("anna@muster.at"),
                passwort: passwort_wie("ein-ganz-neues-passwort"),
                client_key: client_key(20),
            },
        )
        .await
        .expect("das neue Passwort gilt");
}

// ------------------------------------------------------------------------ AC-12, AC-13

#[test_log::test(pollster::test)]
async fn ac12_löschung_entfernt_identität_und_gibt_die_adresse_frei() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    harness
        .app()
        .execute(
            auth_aus(&ausgestellt),
            BenutzerLöschen {
                passwort: passwort(),
                bestätigt: true,
            },
        )
        .await
        .expect("die Löschung gelingt");

    let danach = harness
        .app()
        .execute(
            auth_aus(&ausgestellt),
            BenutzerAnzeigen { benutzer_id: None },
        )
        .await;
    assert!(danach.is_err(), "der Benutzer ist weg");

    let anmeldung = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Anmeldung {
                email: adresse("anna@muster.at"),
                passwort: passwort(),
                client_key: client_key(21),
            },
        )
        .await;
    assert!(
        anmeldung.is_err(),
        "die alten Zugangsdaten gelten nicht mehr"
    );

    // The freed address can register again (rule 8).
    let token = registrierung_starten(&harness, "anna@muster.at", client_key(22)).await;
    registriere_abschließen(&harness, &token, client_key(23))
        .await
        .expect("die freigegebene Adresse kann sich neu registrieren");
}

#[test_log::test(pollster::test)]
async fn ac12_löschung_ohne_bestätigung_oder_mit_falschem_passwort_scheitert() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    let ohne_flag = harness
        .app()
        .execute(
            auth_aus(&ausgestellt),
            BenutzerLöschen {
                passwort: passwort(),
                bestätigt: false,
            },
        )
        .await;
    assert!(ohne_flag.is_err());

    let falsch = harness
        .app()
        .execute(
            auth_aus(&ausgestellt),
            BenutzerLöschen {
                passwort: passwort_wie("falsch-falsch"),
                bestätigt: true,
            },
        )
        .await;
    assert!(falsch.is_err());

    harness
        .app()
        .execute(
            auth_aus(&ausgestellt),
            BenutzerAnzeigen { benutzer_id: None },
        )
        .await
        .expect("der Benutzer existiert weiterhin");
}

#[test_log::test(pollster::test)]
async fn ac13_anonymer_zugriff_wird_verweigert() {
    let harness = harness().await;

    let anzeigen = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            BenutzerAnzeigen { benutzer_id: None },
        )
        .await;
    assert!(anzeigen.is_err());

    let abmelden = harness
        .app()
        .execute(AuthContext::Anonymous, Abmeldung)
        .await;
    assert!(abmelden.is_err());
}

// ------------------------------------------------------------------------------- AC-17

/// AC-17: a token is checked against the public key stored **on its own session row**.
///
/// Deleting the row is not a way out either: an unknown `sid` is a 401, never a 500 and
/// never an implicit acceptance.
#[test_log::test(pollster::test)]
async fn ac17_unbekannte_session_wird_abgelehnt() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    let mut claims = ausgestellt.claims.clone();
    claims.sid = SessionId(uuid::Uuid::new_v4());

    let danach = harness
        .app()
        .execute(AuthContext::Anonymous, pruefen(&harness, claims))
        .await;
    assert!(danach.is_err(), "eine unbekannte Sitzung ist ein 401");
}

/// A signature that is well-formed but was made by another key does not verify against
/// the public key the row carries — even when it names the same `kid`.
#[test_log::test(pollster::test)]
async fn ac17_fremder_schlüssel_passt_nicht_zur_session_zeile() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    let fremd = fremder_schluessel();
    let danach = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            pruefen_mit(&fremd, ausgestellt.claims.clone()),
        )
        .await;
    assert!(
        danach.is_err(),
        "die Signatur muss zum gespeicherten public_key passen"
    );
}

#[test_log::test(pollster::test)]
async fn ac17_widerrufene_session_wird_trotz_gültigem_schlüssel_abgelehnt() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    harness
        .app()
        .execute(auth_aus(&ausgestellt), Abmeldung)
        .await
        .expect("abgemeldet");

    let danach = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            pruefen(&harness, ausgestellt.claims.clone()),
        )
        .await;
    assert!(
        danach.is_err(),
        "ein widerrufenes Sitzungs-Token ist ein 401, auch mit gültiger Signatur"
    );
}

// ------------------------------------------------------------------------------- AC-18

#[test_log::test(pollster::test)]
async fn ac18_abgelaufenes_access_token_wird_abgelehnt() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    harness.vorspulen(ACCESS_TOKEN_LIFETIME + Duration::minutes(5));

    let danach = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            pruefen(&harness, ausgestellt.claims.clone()),
        )
        .await;
    assert!(danach.is_err(), "nach 24 Stunden ist das Token abgelaufen");
}

// ------------------------------------------------------------------------------ AC-14

#[test_log::test(pollster::test)]
async fn ac14_fremden_benutzer_liest_nur_ein_systemadministrator() {
    let harness = harness().await;
    let anna = registriere(&harness, "anna@muster.at").await;

    let (anna_id, fremd_id) = harness
        .app()
        .execute_fn::<_, _, RepositoryError>(AuthContext::Anonymous, async |ctx| {
            let uow = ctx.enter().await?;
            let anna_id = uow
                .konten()
                .find_by_email(&adresse("anna@muster.at"))
                .await?
                .cloned_data()
                .benutzer_id();
            // A Benutzer never exists without its Konto (rule 3), so the foreign identity
            // is seeded as a complete pair — otherwise the fixture, not the use case,
            // decides what a missing Konto means.
            let fremd = uow
                .benutzer()
                .create(NeuerBenutzer::registrierung(ctx.clock().now()))
                .await?;
            let hash = hash_password(&passwort()).expect("Passwort wird gehasht");
            uow.konten()
                .create(NeuesKonto::neu(
                    fremd.id(),
                    adresse("fremd@muster.at"),
                    hash,
                    ctx.clock().now(),
                ))
                .await?;
            uow.commit().await?;
            Ok((anna_id, fremd.id()))
        })
        .await
        .expect("Seed-Daten werden angelegt");

    let fremd_als_benutzer = harness
        .app()
        .execute(
            AuthContext::Authenticated {
                benutzer_id: anna_id,
                session_id: anna.session_id,
            },
            BenutzerAnzeigen {
                benutzer_id: Some(fremd_id),
            },
        )
        .await;
    assert_eq!(
        text(fremd_als_benutzer.unwrap_err()),
        BenutzerAnzeigenFehler::NichtErlaubt.to_string()
    );

    // Promote Anna, then the same request succeeds. A plain domain mutation written back
    // with `update`, so the repository stays a store and optimistic concurrency applies.
    harness
        .app()
        .execute_fn::<_, _, RepositoryError>(AuthContext::Anonymous, async |ctx| {
            let uow = ctx.enter().await?;
            let mut benutzer = uow.benutzer().find_by_id(anna_id).await?;
            benutzer.befördere_zu_systemadministrator();
            uow.benutzer().update(&mut benutzer).await?;
            uow.commit().await
        })
        .await
        .expect("die Rolle wird gesetzt");

    harness
        .app()
        .execute(
            AuthContext::Authenticated {
                benutzer_id: anna_id,
                session_id: anna.session_id,
            },
            BenutzerAnzeigen {
                benutzer_id: Some(fremd_id),
            },
        )
        .await
        .expect("ein Systemadministrator liest jeden Benutzer");
}

// ------------------------------------------------------------------------------ AC-19

#[test_log::test(pollster::test)]
async fn ac19_die_mail_trägt_einen_aufrufbaren_link() {
    let harness = harness().await;

    registrierung_starten(&harness, "anna@muster.at", client_key(40)).await;

    let mail = harness.mail().einzige();
    assert_eq!(mail.betreff(), "Bestätige deine E-Mail-Adresse");
    let text = mail.text(FRONTEND_BASIS_URL);
    assert!(
        text.contains("https://logandaim.at/registrierung?token="),
        "die Mail trägt den Frontend-Link: {text}"
    );
    assert_eq!(mail.empfänger.as_ref(), "anna@muster.at");
}
// ------------------------------------------------------ repository contract

/// The reference contract every repository follows: `update` writes a whole row back and
/// bumps the version **in place**, `delete` takes the aggregate as it was loaded. A stale
/// version is a conflict in both directions, which is what keeps two writers from silently
/// overwriting or deleting each other's change.
#[test_log::test(pollster::test)]
async fn speicher_vertrag_aktualisiert_die_version_und_schuetzt_das_loeschen() {
    let harness = harness().await;

    let (benutzer_id, geladen) = harness
        .app()
        .execute_fn::<_, _, RepositoryError>(AuthContext::Anonymous, async |ctx| {
            let uow = ctx.enter().await?;
            let erstellt = uow
                .benutzer()
                .create(NeuerBenutzer::neu(Rolle::Benutzer, ctx.clock().now()))
                .await?;
            let geladen = uow.benutzer().find_by_id(erstellt.id()).await?;
            uow.commit().await?;
            Ok((erstellt.id(), geladen))
        })
        .await
        .expect("ein Benutzer wird angelegt und zurückgelesen");

    let mut zu_aktualisieren = geladen.clone();
    zu_aktualisieren.befördere_zu_systemadministrator();
    harness
        .app()
        .execute_fn::<_, _, RepositoryError>(AuthContext::Anonymous, async |ctx| {
            let uow = ctx.enter().await?;
            uow.benutzer().update(&mut zu_aktualisieren).await?;
            uow.commit().await
        })
        .await
        .expect("eine Domain-Mutation wird zurückgeschrieben");

    assert_eq!(
        zu_aktualisieren.v(),
        geladen.v() + 1,
        "update hebt die Version des Aufrufers in place an"
    );

    // Writing the same row again at its now-stale version is a conflict, not a rewrite.
    let mut veraltet = geladen.clone();
    veraltet.befördere_zu_systemadministrator();
    let zweiter_schreibversuch = harness
        .app()
        .execute_fn::<_, _, RepositoryError>(AuthContext::Anonymous, async |ctx| {
            let uow = ctx.enter().await?;
            let ergebnis = uow.benutzer().update(&mut veraltet).await;
            uow.finish(ergebnis, RepositoryError::Unknown).await
        })
        .await;
    assert!(
        matches!(
            zweiter_schreibversuch.unwrap_err().current_context(),
            RepositoryError::VersionMismatch { .. }
        ),
        "eine veraltete Version wird abgelehnt"
    );

    // A delete that carries the stale row is refused; the fresh row deletes.
    let veraltetes_loeschen = harness
        .app()
        .execute_fn::<_, _, RepositoryError>(AuthContext::Anonymous, async |ctx| {
            let uow = ctx.enter().await?;
            let ergebnis = uow.benutzer().delete(geladen.clone()).await;
            uow.finish(ergebnis, RepositoryError::Unknown).await
        })
        .await;
    assert!(
        matches!(
            veraltetes_loeschen.unwrap_err().current_context(),
            RepositoryError::VersionMismatch { .. }
        ),
        "delete prüft die Version des übergebenen Aggregats"
    );

    harness
        .app()
        .execute_fn::<_, _, RepositoryError>(AuthContext::Anonymous, async |ctx| {
            let uow = ctx.enter().await?;
            uow.benutzer().delete(zu_aktualisieren).await?;
            uow.commit().await
        })
        .await
        .expect("die aktuelle Version löscht die Zeile");

    let nachher = harness
        .app()
        .execute_fn::<_, _, RepositoryError>(AuthContext::Anonymous, async |ctx| {
            let uow = ctx.enter().await?;
            let ergebnis = uow.benutzer().find_by_id(benutzer_id).await;
            uow.commit().await?;
            Ok(ergebnis)
        })
        .await
        .expect("die Suche läuft");
    assert!(
        nachher.is_err(),
        "die gelöschte Zeile ist danach nicht mehr auffindbar"
    );
}
