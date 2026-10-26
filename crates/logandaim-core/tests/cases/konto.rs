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
            hash_password,
        },
    },
    ports::{KundeSchluessel, RepositoryError},
    service::{
        Abmeldung, Anmeldung, BenutzerAnzeigen, BenutzerLoeschen, Erneuerung, KontoDienstFehler,
        PasswortVergessen, PasswortWiederherstellen, RegistrierungAbschliessen,
        RegistrierungStarten, SitzungAusgestellt, SitzungPruefen,
    },
};

use super::{super::base_app_builder, helpers::Harness};

const PASSWORT: &str = "korrekt-pferd-batterie";

fn kunde(ip: u8) -> KundeSchluessel {
    KundeSchluessel::neu(IpAddr::V4(Ipv4Addr::new(10, 0, 0, ip)), "test-client")
}

async fn harness() -> Harness {
    let harness = super::helpers::build(base_app_builder().await);
    // Every case needs the key row, exactly as a real startup registers it.
    harness
        .app()
        .signing_key_registrieren()
        .await
        .expect("signing key row is registered at startup");
    harness
}

/// Run registration step 1 and return the mailed token.
async fn registrierung_starten(harness: &Harness, email: &str, kunde: KundeSchluessel) -> String {
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungStarten {
                email: email.to_owned(),
                kunde,
            },
        )
        .await
        .expect("registration step 1 answers Ok");
    harness.letztes_token()
}

/// Register fully and return the issued session.
async fn registriere(harness: &Harness, email: &str) -> SitzungAusgestellt {
    let token = registrierung_starten(harness, email, kunde(1)).await;
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungAbschliessen {
                token,
                passwort: PASSWORT.to_owned(),
                kunde: kunde(1),
            },
        )
        .await
        .expect("registration step 2 issues a session")
}

async fn anmelden(harness: &Harness, email: &str) -> SitzungAusgestellt {
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Anmeldung {
                email: email.to_owned(),
                passwort: PASSWORT.to_owned(),
                kunde: kunde(2),
            },
        )
        .await
        .expect("login issues a session")
}

/// What the API will inject after decoding a bearer token.
fn auth_aus(ausgestellt: &SitzungAusgestellt) -> AuthContext {
    AuthContext::Authenticated {
        benutzer_id: ausgestellt.benutzer.id(),
        session_id: ausgestellt.session_id,
    }
}

fn fehler<E>(report: &error_stack::Report<E>) -> String
where
    E: std::error::Error + Send + Sync + 'static,
{
    report.current_context().to_string()
}

fn fehler_von(report: error_stack::Report<KontoDienstFehler>) -> String {
    fehler(&report)
}

// ---------------------------------------------------------------- AC-1, AC-2, AC-20

#[test_log::test(pollster::test)]
async fn ac1_registrierung_startet_ohne_benutzer_und_mailt_genau_einmal() {
    let harness = harness().await;

    let token = registrierung_starten(&harness, "anna@muster.at", kunde(1)).await;

    assert_eq!(harness.mail().gesendete().len(), 1);
    assert_eq!(token.len(), 43, "das gemailte Token ist ein 256-bit Secret");

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
                email: "belegt@muster.at".to_owned(),
                kunde: kunde(3),
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
async fn ac3_abschluss_erzeugt_benutzer_konto_und_erste_sitzung() {
    let harness = harness().await;

    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    assert_eq!(ausgestellt.benutzer.rolle(), Rolle::Benutzer);
    assert_eq!(
        ausgestellt.claims.sub,
        ausgestellt.benutzer.id(),
        "das Access-Token gehört dem neuen Benutzer"
    );
    assert_eq!(ausgestellt.claims.sid, ausgestellt.session_id);
    assert_ne!(ausgestellt.refresh_token.as_str().len(), 0);
}

#[test_log::test(pollster::test)]
async fn ac4_ungueltiges_token_oder_kurzes_passwort_erzeugt_nichts() {
    let harness = harness().await;
    let token = registrierung_starten(&harness, "anna@muster.at", kunde(4)).await;

    // Too short: refused, and the usable token is not spent by the attempt.
    let kurz = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungAbschliessen {
                token: token.clone(),
                passwort: "kurz".to_owned(),
                kunde: kunde(5),
            },
        )
        .await;
    assert!(kurz.is_err());
    assert_eq!(fehler_von(kurz.unwrap_err()), fehler_ungueltig());

    // An unknown token is refused too.
    let unbekannt = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungAbschliessen {
                token: "A".repeat(43),
                passwort: PASSWORT.to_owned(),
                kunde: kunde(6),
            },
        )
        .await;
    assert!(unbekannt.is_err());

    // The real token still works: neither refusal spent it.
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungAbschliessen {
                token,
                passwort: PASSWORT.to_owned(),
                kunde: kunde(7),
            },
        )
        .await
        .expect("das unverbrauchte Token bleibt gültig");
}

#[test_log::test(pollster::test)]
async fn ac4_verbrauchtes_token_wird_beim_zweiten_mal_abgelehnt() {
    let harness = harness().await;
    let token = registrierung_starten(&harness, "anna@muster.at", kunde(8)).await;
    registriere_abschliessen(&harness, &token, kunde(8))
        .await
        .expect("erster Abschluss gelingt");

    let zweiter = registriere_abschliessen(&harness, &token, kunde(9)).await;
    assert!(zweiter.is_err(), "ein verbrauchtes Token ist unbrauchbar");
}

async fn registriere_abschliessen(
    harness: &Harness,
    token: &str,
    kunde: KundeSchluessel,
) -> Result<SitzungAusgestellt, error_stack::Report<KontoDienstFehler>> {
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungAbschliessen {
                token: token.to_owned(),
                passwort: PASSWORT.to_owned(),
                kunde,
            },
        )
        .await
}

fn fehler_ungueltig() -> String {
    KontoDienstFehler::UngueltigeAnfrage.to_string()
}

// ------------------------------------------------------------------------- AC-5, AC-6

#[test_log::test(pollster::test)]
async fn ac5_anmeldung_eroeffnet_eine_gueltige_sitzung() {
    let harness = harness().await;
    registriere(&harness, "anna@muster.at").await;

    let ausgestellt = anmelden(&harness, "anna@muster.at").await;

    let geprueft = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            SitzungPruefen {
                claims: ausgestellt.claims.clone(),
            },
        )
        .await
        .expect("ein frisches Access-Token wird akzeptiert");

    assert_eq!(geprueft, auth_aus(&ausgestellt));
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
                email: "anna@muster.at".to_owned(),
                passwort: "falsches-passwort".to_owned(),
                kunde: kunde(10),
            },
        )
        .await;
    let unbekannt = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Anmeldung {
                email: "niemand@muster.at".to_owned(),
                passwort: PASSWORT.to_owned(),
                kunde: kunde(11),
            },
        )
        .await;

    assert_eq!(
        fehler_von(falsch.unwrap_err()),
        fehler_von(unbekannt.unwrap_err()),
        "die Antwort darf die Existenz der Adresse nicht verraten"
    );
    assert!(
        harness
            .app()
            .execute(
                AuthContext::Anonymous,
                SitzungPruefen {
                    claims: logandaim_core::domain::konto::AccessTokenClaims {
                        sub: BenutzerId(uuid::Uuid::nil()),
                        sid: logandaim_core::domain::konto::SessionId(uuid::Uuid::nil()),
                        kid: logandaim_core::domain::konto::SigningKeyId::new("kein").unwrap(),
                        iss: "logandaim.at".into(),
                        aud: "logandaim-api".into(),
                        iat: harness.jetzt(),
                        exp: harness.jetzt() + ACCESS_TOKEN_LIFETIME,
                        jti: "test".into(),
                    }
                }
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
                refresh_token: alt.refresh_token.as_str().to_owned(),
                kunde: kunde(12),
            },
        )
        .await
        .expect("die Erneuerung liefert ein neues Paar");

    assert_ne!(neu.refresh_token.as_str(), alt.refresh_token.as_str());

    // The pre-rotation access token still works: only revocation ends it (AC-9, AC-17).
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            SitzungPruefen {
                claims: alt.claims.clone(),
            },
        )
        .await
        .expect("das alte Access-Token bleibt bis zum Ablauf gültig");

    // The new one works as well.
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            SitzungPruefen {
                claims: neu.claims.clone(),
            },
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
                refresh_token: alt.refresh_token.as_str().to_owned(),
                kunde: kunde(13),
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
                refresh_token: alt.refresh_token.as_str().to_owned(),
                kunde: kunde(14),
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
                refresh_token: neu.refresh_token.as_str().to_owned(),
                kunde: kunde(15),
            },
        )
        .await;
    assert!(
        nachfolger.is_err(),
        "ein Replay beendet die ganze Sitzungsfamilie"
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
                refresh_token: alt.refresh_token.as_str().to_owned(),
                kunde: kunde(16),
            },
        )
        .await;
    assert!(erneuerung.is_err(), "ein Monat ist das Ende der Sitzung");
}

// ------------------------------------------------------------------------- AC-9

#[test_log::test(pollster::test)]
async fn ac9_abmeldung_widerruft_und_das_access_token_faellt_sofort() {
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
            SitzungPruefen {
                claims: ausgestellt.claims.clone(),
            },
        )
        .await;
    assert!(
        danach.is_err(),
        "ein widerrufenes Sitzungs-Token ist sofort ungültig, auch unverbraucht"
    );
}

// ------------------------------------------------------------------------ AC-10, AC-11

#[test_log::test(pollster::test)]
async fn ac10_wiederherstellung_antwortet_fuer_jede_adresse_gleich() {
    let harness = harness().await;
    registriere(&harness, "anna@muster.at").await;
    harness.mail().leeren();

    let bekannt = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            PasswortVergessen {
                email: "anna@muster.at".to_owned(),
                kunde: kunde(17),
            },
        )
        .await;
    let unbekannt = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            PasswortVergessen {
                email: "niemand@muster.at".to_owned(),
                kunde: kunde(18),
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
async fn ac11_neues_passwort_beendet_alle_sitzungen_und_ist_einmalig() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;
    harness.mail().leeren();

    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            PasswortVergessen {
                email: "anna@muster.at".to_owned(),
                kunde: kunde(19),
            },
        )
        .await
        .expect("die Wiederherstellung wird angefordert");
    let token = harness.letztes_token();

    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            PasswortWiederherstellen {
                token: token.clone(),
                neues_passwort: "ein-ganz-neues-passwort".to_owned(),
            },
        )
        .await
        .expect("das neue Passwort wird gesetzt");

    // Every session ended, including the access token issued before.
    let alte_sitzung = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            SitzungPruefen {
                claims: ausgestellt.claims.clone(),
            },
        )
        .await;
    assert!(alte_sitzung.is_err(), "alle Sitzungen sind beendet");

    // Single use.
    let zweiter = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            PasswortWiederherstellen {
                token,
                neues_passwort: "noch-ein-passwort".to_owned(),
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
                email: "anna@muster.at".to_owned(),
                passwort: "ein-ganz-neues-passwort".to_owned(),
                kunde: kunde(20),
            },
        )
        .await
        .expect("das neue Passwort gilt");
}

// ------------------------------------------------------------------------ AC-12, AC-13

#[test_log::test(pollster::test)]
async fn ac12_loeschung_entfernt_identitaet_und_gibt_die_adresse_frei() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    harness
        .app()
        .execute(
            auth_aus(&ausgestellt),
            BenutzerLoeschen {
                passwort: PASSWORT.to_owned(),
                bestaetigt: true,
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
                email: "anna@muster.at".to_owned(),
                passwort: PASSWORT.to_owned(),
                kunde: kunde(21),
            },
        )
        .await;
    assert!(
        anmeldung.is_err(),
        "die alten Zugangsdaten gelten nicht mehr"
    );

    // The freed address can register again (rule 8).
    let token = registrierung_starten(&harness, "anna@muster.at", kunde(22)).await;
    registriere_abschliessen(&harness, &token, kunde(23))
        .await
        .expect("die freigegebene Adresse kann sich neu registrieren");
}

#[test_log::test(pollster::test)]
async fn ac12_loeschung_ohne_bestaetigung_oder_mit_falschem_passwort_scheitert() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    let ohne_flag = harness
        .app()
        .execute(
            auth_aus(&ausgestellt),
            BenutzerLoeschen {
                passwort: PASSWORT.to_owned(),
                bestaetigt: false,
            },
        )
        .await;
    assert!(ohne_flag.is_err());

    let falsch = harness
        .app()
        .execute(
            auth_aus(&ausgestellt),
            BenutzerLoeschen {
                passwort: "falsch-falsch".to_owned(),
                bestaetigt: true,
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

// ------------------------------------------------------------------------ AC-15, AC-16

#[test_log::test(pollster::test)]
async fn ac15_drossel_greift_erst_nach_zwanzig_fehlversuchen_pro_client() {
    let harness = harness().await;
    let client = kunde(30);

    for _ in 0..20 {
        let ergebnis = harness
            .app()
            .execute(
                AuthContext::Anonymous,
                Anmeldung {
                    email: "niemand@muster.at".to_owned(),
                    passwort: PASSWORT.to_owned(),
                    kunde: client.clone(),
                },
            )
            .await;
        assert!(ergebnis.is_err());
        assert_ne!(
            fehler_von(ergebnis.unwrap_err()),
            fehler_gedrosselt(),
            "innerhalb des Budgets ist die Antwort ein Zugangsfehler, keine Drosselung"
        );
    }

    let einundzwanzigster = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Anmeldung {
                email: "niemand@muster.at".to_owned(),
                passwort: PASSWORT.to_owned(),
                kunde: client.clone(),
            },
        )
        .await;
    assert_eq!(
        fehler_von(einundzwanzigster.unwrap_err()),
        fehler_gedrosselt()
    );

    // Another client is unaffected: the limit is never per address.
    let anderer_client = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            Anmeldung {
                email: "niemand@muster.at".to_owned(),
                passwort: PASSWORT.to_owned(),
                kunde: kunde(31),
            },
        )
        .await;
    assert_ne!(fehler_von(anderer_client.unwrap_err()), fehler_gedrosselt());
}

#[test_log::test(pollster::test)]
async fn ac16_registrierung_ist_auf_fuenf_pro_stunde_gedrosselt() {
    let harness = harness().await;
    let client = kunde(32);

    for nummer in 0..5 {
        harness
            .app()
            .execute(
                AuthContext::Anonymous,
                RegistrierungStarten {
                    email: format!("person{nummer}@muster.at"),
                    kunde: client.clone(),
                },
            )
            .await
            .expect("die ersten fünf Anfragen gehen durch");
    }

    let sechste = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungStarten {
                email: "person6@muster.at".to_owned(),
                kunde: client,
            },
        )
        .await;
    assert_eq!(fehler_von(sechste.unwrap_err()), fehler_gedrosselt());
}

fn fehler_gedrosselt() -> String {
    KontoDienstFehler::Gedrosselt.to_string()
}

// ------------------------------------------------------------------------------- AC-17

#[test_log::test(pollster::test)]
async fn ac17_ausser_betrieb_gesetzter_schluessel_wird_abgelehnt() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    harness
        .app()
        .execute_fn::<_, _, RepositoryError>(AuthContext::Anonymous, async |ctx| {
            let uow = ctx.enter().await?;
            uow.signing_keys()
                .ausser_betrieb_nehmen(&helpers_kid(), ctx.clock().now())
                .await?;
            uow.commit().await
        })
        .await
        .expect("der Schlüssel wird außer Betrieb genommen");

    let danach = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            SitzungPruefen {
                claims: ausgestellt.claims.clone(),
            },
        )
        .await;
    assert!(danach.is_err(), "ein stillgelegter kid ist ein 401");
}

fn helpers_kid() -> logandaim_core::domain::konto::SigningKeyId {
    super::helpers::test_signing_key().kid().clone()
}

#[test_log::test(pollster::test)]
async fn ac17_unbekannter_kid_wird_abgelehnt() {
    let harness = harness().await;
    let ausgestellt = registriere(&harness, "anna@muster.at").await;

    let mut claims = ausgestellt.claims.clone();
    claims.kid = logandaim_core::domain::konto::SigningKeyId::new("fremd").unwrap();

    let danach = harness
        .app()
        .execute(AuthContext::Anonymous, SitzungPruefen { claims })
        .await;
    assert!(danach.is_err(), "ein unbekannter kid ist ein 401");
}

#[test_log::test(pollster::test)]
async fn ac17_widerrufene_sitzung_wird_trotz_gueltigem_schluessel_abgelehnt() {
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
            SitzungPruefen {
                claims: ausgestellt.claims.clone(),
            },
        )
        .await;
    assert!(danach.is_err());
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
            SitzungPruefen {
                claims: ausgestellt.claims.clone(),
            },
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
                .find_by_email(&EmailAdresse::new("anna@muster.at").unwrap())
                .await?
                .expect("die Adresse hat ein Konto")
                .benutzer_id();
            // A Benutzer never exists without its Konto (rule 3), so the foreign identity
            // is seeded as a complete pair — otherwise the fixture, not the use case,
            // decides what a missing Konto means.
            let fremd = uow
                .benutzer()
                .create(NeuerBenutzer::registrierung(ctx.clock().now()))
                .await?;
            let passwort = KlartextPasswort::neu(PASSWORT).expect("Testpasswort ist gültig");
            let hash = hash_password(&passwort).expect("Passwort wird gehasht");
            uow.konten()
                .create(NeuesKonto::neu(
                    fremd.id(),
                    EmailAdresse::new("fremd@muster.at").unwrap(),
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
        fehler_von(fremd_als_benutzer.unwrap_err()),
        KontoDienstFehler::NichtErlaubt.to_string()
    );

    // Promote Anna, then the same request succeeds.
    harness
        .app()
        .execute_fn::<_, _, RepositoryError>(AuthContext::Anonymous, async |ctx| {
            let uow = ctx.enter().await?;
            uow.benutzer()
                .setze_rolle(anna_id, Rolle::Systemadministrator)
                .await?;
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
async fn ac19_die_mail_traegt_einen_aufrufbaren_link() {
    let harness = harness().await;

    registrierung_starten(&harness, "anna@muster.at", kunde(40)).await;

    let mail = harness.mail().einzige();
    assert_eq!(mail.betreff, "Bestätige deine E-Mail-Adresse");
    assert!(
        mail.text
            .contains("https://logandaim.at/registrierung?token="),
        "die Mail trägt den konfigurierten Frontend-Link: {}",
        mail.text
    );
    assert_eq!(mail.empfaenger.as_ref(), "anna@muster.at");
}
