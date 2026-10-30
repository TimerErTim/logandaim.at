//! Shared master-data cases, run twice: against the fakes and against SQLite.
//!
//! Every case drives a use case rather than a repository, so a passing run says the
//! *product* behaviour of AC-1..AC-13 holds on this adapter — not merely that a `SELECT`
//! round-trips. Cases are named after the acceptance criterion they pin down.

use std::net::{IpAddr, Ipv4Addr};

use logandaim_core::{
    application::AuthContext,
    domain::{
        Abstand, Anlage, AnlageAdresse, AnlageKontakt, AnlageName, AnlageVerwaltungRolle,
        EmailAdresse, Farbe, Ländercode, NeuerParkour, NonEmptyString, ParkourId, Pflock, PflockId,
        Wertungszone, ZielId, Zielplatzierung,
        konto::KlartextPasswort,
        konto::{BenutzerId, SecretToken},
    },
    ports::ClientKey,
    service::{
        AnlageBeschreibendeDatenÄndern, AnlageDeaktivieren, AnlageErstellen,
        AnlageVerwaltungEntziehen, AnlageVerwaltungGewähren, ParkourEntfernen, ParkourHinzufügen,
        ParkourSetupÄndern, RegistrierungAbschließen, RegistrierungStarten, RundeAbschliessen,
        RundeStarten, RundeStartenFehler, SchussRegistrieren, SessionAusgestellt, StammdatenFehler,
        SystemadministratorBefördern, ZielEntfernen, ZielErsetzen, ZielHinzufügen, ZielÄndern,
    },
};

use super::{
    super::base_app_builder,
    helpers::{Harness, anlage_ansicht, jetzt, runde_ansicht},
};

const PASSWORT: &str = "korrekt-pferd-batterie";

// The list read and the Anlage detail read are composed at the HTTP edge now, so their
// assertions live in `e2e/stammdaten.rs` (`die_anlagenliste_zeigt_je_nach_rolle_die_eigenen_anlagen`,
// `die_anlage_zeigt_wo_ihre_ziele_verwendet_werden`). A core case that only needs the
// aggregate reads it through `anlage_ansicht`.

fn client_key(ip: u8) -> ClientKey {
    ClientKey::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, ip)), "stammdaten-case")
}

fn adresse(wert: &str) -> EmailAdresse {
    EmailAdresse::normalisiert(wert).expect("Testadresse ist gültig")
}

fn anlage_name(wert: &str) -> AnlageName {
    AnlageName::neu(wert).expect("Testname ist gültig")
}

fn anlage_adresse() -> AnlageAdresse {
    AnlageAdresse::neu("Waldweg 1", "3100", "St. Pölten", Ländercode::AT)
        .expect("Testadresse ist gültig")
}

fn tiername(wert: &str) -> NonEmptyString {
    NonEmptyString::neu(wert).expect("Testtiername ist gültig")
}

fn abstand(meter: u32) -> Abstand {
    Abstand::neu(meter).expect("Testabstand ist positiv")
}

async fn harness() -> Harness {
    super::helpers::build(base_app_builder().await)
}

/// Register one Benutzer and return the auth context the API would inject.
async fn benutzer_mit_ip(harness: &Harness, email: &str, ip: u8) -> AuthContext {
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungStarten {
                email: adresse(email),
                client_key: client_key(ip),
            },
        )
        .await
        .expect("Registrierung startet");
    let token = SecretToken::try_from(harness.letztes_token().as_str())
        .expect("die Mail trägt ein SecretToken");
    let ausgestellt: SessionAusgestellt = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RegistrierungAbschließen {
                token,
                passwort: KlartextPasswort::neu(PASSWORT).expect("Testpasswort ist gültig"),
                client_key: client_key(ip),
            },
        )
        .await
        .expect("Registrierung schließt ab");
    AuthContext::Authenticated {
        benutzer_id: ausgestellt.benutzer.id(),
        session_id: ausgestellt.session_id,
    }
}

async fn benutzer(harness: &Harness, email: &str) -> AuthContext {
    benutzer_mit_ip(harness, email, 1).await
}

/// Register one Benutzer and promote it through the configured-superuser path (rule 9).
async fn systemadministrator(harness: &Harness, email: &str) -> AuthContext {
    let auth = benutzer(harness, email).await;
    harness
        .app()
        .execute(
            AuthContext::Anonymous,
            SystemadministratorBefördern {
                email: adresse(email),
            },
        )
        .await
        .expect("die Beförderung läuft");
    auth
}

fn id_von(auth: AuthContext) -> BenutzerId {
    auth.benutzer_id().expect("authentifizierter Kontext")
}

async fn anlage_erstellen(harness: &Harness, auth: AuthContext, name: &str) -> Anlage {
    harness
        .app()
        .execute(
            auth,
            AnlageErstellen {
                name: anlage_name(name),
                adresse: anlage_adresse(),
                kontakt: AnlageKontakt::leer(),
            },
        )
        .await
        .expect("der Systemadministrator legt eine Anlage an")
}

/// Two Pflöcke far to near, plus one placement of `ziel` at 40 m and 20 m.
fn parkour_setup(
    ziel: ZielId,
    pflöcke: Option<(PflockId, PflockId)>,
) -> (Vec<Pflock>, Vec<Zielplatzierung>) {
    let (weit_id, nah_id) = pflöcke.unwrap_or_else(|| {
        (
            PflockId(uuid::Uuid::new_v4()),
            PflockId(uuid::Uuid::new_v4()),
        )
    });
    let fern = Pflock::neu(weit_id, Farbe::neu("#c0392b").expect("gültig"));
    let nah = Pflock::neu(nah_id, Farbe::neu("#1b4f9c").expect("gültig"));
    // Near to far: the first Pflock is the nearest, distances strictly increase.
    let platzierung =
        Zielplatzierung::neu(ziel, vec![(nah_id, abstand(20)), (weit_id, abstand(40))])
            .expect("die Platzierung ist gültig");
    (vec![nah, fern], vec![platzierung])
}

/// An active Anlage with one Ziel and one Parkour, maintained by `auth`.
async fn anlage_mit_parkour(
    harness: &Harness,
    auth: AuthContext,
    name: &str,
) -> (Anlage, ZielId, ParkourId) {
    let anlage = anlage_erstellen(harness, auth, name).await;
    let anlage_id = anlage.id();

    let mit_ziel = harness
        .app()
        .execute(
            auth,
            ZielHinzufügen {
                anlage_id,
                tiername: tiername("Keiler"),
            },
        )
        .await
        .expect("Masterdaten: ein Ziel wird angelegt");
    let ziel_id = mit_ziel
        .ziele()
        .last()
        .expect("die Anlage hat ein Ziel")
        .id();

    let (pflöcke, platzierungen) = parkour_setup(ziel_id, None);
    let mit_parkour = harness
        .app()
        .execute(
            auth,
            ParkourHinzufügen {
                anlage_id,
                name: NonEmptyString::neu("Waldrunde").expect("gültig"),
                pflöcke,
                platzierungen,
            },
        )
        .await
        .expect("Masterdaten: ein Parkour wird angelegt");
    let parkour_id = mit_parkour
        .parkour()
        .last()
        .expect("die Anlage hat einen Parkour")
        .id();

    (mit_parkour, ziel_id, parkour_id)
}

/// Grant `rolle` on `anlage_id` from a caller that already may administer it.
async fn verwaltung_gewähren(
    harness: &Harness,
    auth: AuthContext,
    anlage_id: logandaim_core::domain::AnlageId,
    benutzer_id: BenutzerId,
    rolle: AnlageVerwaltungRolle,
) {
    harness
        .app()
        .execute(
            auth,
            AnlageVerwaltungGewähren {
                anlage_id,
                benutzer_id,
                rolle,
            },
        )
        .await
        .expect("die Verwaltung wird vergeben");
}

// --------------------------------------------------------------------------- AC-1

#[test_log::test(pollster::test)]
async fn ac1_ein_systemadministrator_legt_eine_aktive_anlage_ohne_administrator_an() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;

    let anlage = harness
        .app()
        .execute(
            admin,
            AnlageErstellen {
                name: anlage_name("Breitenstein"),
                adresse: anlage_adresse(),
                kontakt: AnlageKontakt::neu(
                    Some("Verein Breitenstein"),
                    Some(adresse("verein@muster.at")),
                    None,
                    None,
                ),
            },
        )
        .await
        .expect("AC-1: die Anlage wird angelegt");

    assert!(matches!(anlage, Anlage::Aktiv(_)), "AC-1: sie ist aktiv");
    assert!(
        anlage.verwaltungen().is_empty(),
        "AC-1: sie darf ohne Administrator entstehen"
    );
    assert_eq!(anlage.name().as_str(), "Breitenstein");
    assert_eq!(anlage.adresse().ort(), "St. Pölten");
}

#[test_log::test(pollster::test)]
async fn ac1_doppelte_namen_sind_erlaubt() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;

    let erste = anlage_erstellen(&harness, admin, "Wallhalla Pregarten").await;
    let zweite = anlage_erstellen(&harness, admin, "Wallhalla Pregarten").await;
    assert_ne!(erste.id(), zweite.id(), "AC-1: gleiche Namen sind erlaubt");
}

#[test_log::test(pollster::test)]
async fn ac1_nur_ein_systemadministrator_legt_an() {
    let harness = harness().await;
    let gewoehnlich = benutzer(&harness, "anna@muster.at").await;

    let fehler = harness
        .app()
        .execute(
            gewoehnlich,
            AnlageErstellen {
                name: anlage_name("Privat"),
                adresse: anlage_adresse(),
                kontakt: AnlageKontakt::leer(),
            },
        )
        .await
        .expect_err("AC-1: nur ein Systemadministrator legt an");
    assert!(matches!(
        fehler.current_context(),
        StammdatenFehler::NichtErlaubt
    ));
}

#[test_log::test(pollster::test)]
async fn ac3_ein_unangemeldetes_schreiben_wird_abgelehnt() {
    let harness = harness().await;

    let fehler = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            AnlageErstellen {
                name: anlage_name("Anonym"),
                adresse: anlage_adresse(),
                kontakt: AnlageKontakt::leer(),
            },
        )
        .await
        .expect_err("AC-3: ein unangemeldetes Schreiben antwortet 401");
    assert!(matches!(
        fehler.current_context(),
        StammdatenFehler::NichtAngemeldet
    ));
}

// --------------------------------------------------------------------------- AC-2

#[test_log::test(pollster::test)]
async fn ac2_ein_systemadministrator_vergibt_eine_rolle_an_einen_bestehenden_benutzer() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let mitarbeiter = benutzer(&harness, "mitarbeiter@muster.at").await;
    let anlage = anlage_erstellen(&harness, admin, "Breitenstein").await;

    let nachher = harness
        .app()
        .execute(
            admin,
            AnlageVerwaltungGewähren {
                anlage_id: anlage.id(),
                benutzer_id: id_von(mitarbeiter),
                rolle: AnlageVerwaltungRolle::Administrator,
            },
        )
        .await
        .expect("AC-2: die Rolle wird vergeben");
    assert_eq!(
        nachher.rolle_von(id_von(mitarbeiter)),
        Some(AnlageVerwaltungRolle::Administrator)
    );
}

#[test_log::test(pollster::test)]
async fn ac2_der_letzte_administrator_kann_nicht_entzogen_werden() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let mitarbeiter = benutzer(&harness, "mitarbeiter@muster.at").await;
    let anlage = anlage_erstellen(&harness, admin, "Breitenstein").await;
    verwaltung_gewähren(
        &harness,
        admin,
        anlage.id(),
        id_von(mitarbeiter),
        AnlageVerwaltungRolle::Administrator,
    )
    .await;

    let fehler = harness
        .app()
        .execute(
            admin,
            AnlageVerwaltungEntziehen {
                anlage_id: anlage.id(),
                benutzer_id: id_von(mitarbeiter),
            },
        )
        .await
        .expect_err("AC-2: der letzte Administrator bleibt");
    assert!(matches!(
        fehler.current_context(),
        StammdatenFehler::UngültigeAnfrage
    ));

    let danach = anlage_ansicht(&harness, anlage.id()).await;
    assert_eq!(
        danach.anlage.rolle_von(id_von(mitarbeiter)),
        Some(AnlageVerwaltungRolle::Administrator),
        "AC-2: die Anlage behält einen Administrator"
    );
}

#[test_log::test(pollster::test)]
async fn ac2_ein_mitarbeiter_vergibt_keine_verwaltung() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let mitarbeiter = benutzer_mit_ip(&harness, "mitarbeiter@muster.at", 2).await;
    let fremd = benutzer_mit_ip(&harness, "fremd@muster.at", 3).await;
    let anlage = anlage_erstellen(&harness, admin, "Breitenstein").await;
    verwaltung_gewähren(
        &harness,
        admin,
        anlage.id(),
        id_von(mitarbeiter),
        AnlageVerwaltungRolle::Mitarbeiter,
    )
    .await;

    let fehler = harness
        .app()
        .execute(
            mitarbeiter,
            AnlageVerwaltungGewähren {
                anlage_id: anlage.id(),
                benutzer_id: id_von(fremd),
                rolle: AnlageVerwaltungRolle::Mitarbeiter,
            },
        )
        .await
        .expect_err("AC-2: ein Mitarbeiter vergibt nichts");
    assert!(matches!(
        fehler.current_context(),
        StammdatenFehler::NichtErlaubt
    ));
}

// --------------------------------------------------------------------------- AC-3

#[test_log::test(pollster::test)]
async fn ac3_ein_mitarbeiter_pflegt_zuerst_die_masterdaten_aber_keine_anlagenfelder() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let mitarbeiter = benutzer_mit_ip(&harness, "mitarbeiter@muster.at", 2).await;
    let anlage = anlage_erstellen(&harness, admin, "Breitenstein").await;
    verwaltung_gewähren(
        &harness,
        admin,
        anlage.id(),
        id_von(mitarbeiter),
        AnlageVerwaltungRolle::Mitarbeiter,
    )
    .await;

    harness
        .app()
        .execute(
            mitarbeiter,
            ZielHinzufügen {
                anlage_id: anlage.id(),
                tiername: tiername("Reh"),
            },
        )
        .await
        .expect("AC-3: ein Mitarbeiter legt ein Ziel an");

    let fehler = harness
        .app()
        .execute(
            mitarbeiter,
            AnlageBeschreibendeDatenÄndern {
                anlage_id: anlage.id(),
                name: anlage_name("Umbenannt"),
                adresse: anlage_adresse(),
                kontakt: AnlageKontakt::leer(),
            },
        )
        .await
        .expect_err("AC-3: die Anlagenfelder brauchen einen Administrator");
    assert!(matches!(
        fehler.current_context(),
        StammdatenFehler::NichtErlaubt
    ));
}

#[test_log::test(pollster::test)]
async fn ac3_ohne_verwaltung_kein_schreibzugriff() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let fremd = benutzer_mit_ip(&harness, "fremd@muster.at", 2).await;
    let anlage = anlage_erstellen(&harness, admin, "Breitenstein").await;

    let fehler = harness
        .app()
        .execute(
            fremd,
            ZielHinzufügen {
                anlage_id: anlage.id(),
                tiername: tiername("Reh"),
            },
        )
        .await
        .expect_err("AC-3: ohne AnlageVerwaltung kein Schreiben");
    assert!(matches!(
        fehler.current_context(),
        StammdatenFehler::NichtErlaubt
    ));
}

#[test_log::test(pollster::test)]
async fn ac3_die_verwaltung_gilt_nur_fuer_ihre_eigene_anlage() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let mitarbeiter = benutzer_mit_ip(&harness, "mitarbeiter@muster.at", 2).await;
    let eine = anlage_erstellen(&harness, admin, "Breitenstein").await;
    let andere = anlage_erstellen(&harness, admin, "Pregarten").await;
    verwaltung_gewähren(
        &harness,
        admin,
        eine.id(),
        id_von(mitarbeiter),
        AnlageVerwaltungRolle::Mitarbeiter,
    )
    .await;

    let fehler = harness
        .app()
        .execute(
            mitarbeiter,
            ZielHinzufügen {
                anlage_id: andere.id(),
                tiername: tiername("Reh"),
            },
        )
        .await
        .expect_err("AC-3: die Befugnis überschreitet keine Anlagengrenze");
    assert!(matches!(
        fehler.current_context(),
        StammdatenFehler::NichtErlaubt
    ));
}

// --------------------------------------------------------------------------- AC-4

#[test_log::test(pollster::test)]
async fn ac4_ein_parkour_verwendet_nur_ziele_seiner_eigenen_anlage() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, _, _) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;
    let fremd = anlage_erstellen(&harness, admin, "Pregarten").await;

    let fremdes_ziel = harness
        .app()
        .execute(
            admin,
            ZielHinzufügen {
                anlage_id: fremd.id(),
                tiername: tiername("Fuchs"),
            },
        )
        .await
        .expect("ein Ziel an der zweiten Anlage")
        .ziele()
        .last()
        .expect("die Anlage hat ein Ziel")
        .id();

    let (pflöcke, platzierungen) = parkour_setup(fremdes_ziel, None);
    let fehler = harness
        .app()
        .execute(
            admin,
            ParkourHinzufügen {
                anlage_id: anlage.id(),
                name: NonEmptyString::neu("Fremdrunde").expect("gültig"),
                pflöcke,
                platzierungen,
            },
        )
        .await
        .expect_err("AC-4: ein fremdes Ziel wird abgelehnt");
    assert!(matches!(
        fehler.current_context(),
        StammdatenFehler::UngültigeAnfrage
    ));
}

#[test_log::test(pollster::test)]
async fn ac4_dasselbe_ziel_darf_in_zwei_parkour_liegen() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, ziel_id, _) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;

    let (pflöcke, platzierungen) = parkour_setup(ziel_id, None);
    let nachher = harness
        .app()
        .execute(
            admin,
            ParkourHinzufügen {
                anlage_id: anlage.id(),
                name: NonEmptyString::neu("Feldrunde").expect("gültig"),
                pflöcke,
                platzierungen,
            },
        )
        .await
        .expect("AC-4: dasselbe Ziel in einem zweiten Parkour");
    assert_eq!(nachher.parkour().len(), 2);
}

#[test_log::test(pollster::test)]
async fn ac4_dasselbe_ziel_zweimal_im_selben_parkour_wird_abgelehnt() {
    let ziel = ZielId(uuid::Uuid::new_v4());
    let (pflöcke, platzierungen) = parkour_setup(ziel, None);
    let ergebnis = NeuerParkour::neu(
        NonEmptyString::neu("Doppelt").expect("gültig"),
        pflöcke,
        vec![platzierungen[0].clone(), platzierungen[0].clone()],
    );
    assert!(
        ergebnis.is_err(),
        "AC-4: ein Ziel höchstens einmal je Parkour"
    );
}

// --------------------------------------------------------------------------- AC-5

#[test_log::test(pollster::test)]
async fn ac5_tiername_wird_geprueft() {
    assert!(
        NonEmptyString::neu("  ").is_err(),
        "AC-5: der Tiername ist Pflicht"
    );
}

#[test_log::test(pollster::test)]
async fn ac5_aendern_erhaelt_die_identitaet() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, ziel_id, _) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;

    let nachher = harness
        .app()
        .execute(
            admin,
            ZielÄndern {
                anlage_id: anlage.id(),
                ziel_id,
                tiername: tiername("Reh"),
            },
        )
        .await
        .expect("AC-5: die Details sind änderbar");
    let ziel = nachher.ziel(ziel_id).expect("das Ziel existiert weiter");
    assert_eq!(ziel.id(), ziel_id, "AC-5: die Identität bleibt");
    assert_eq!(ziel.tiername().as_str(), "Reh");
    assert_eq!(ziel.zielnummer().get(), 1, "AC-5: die Nummer bleibt");
}

#[test_log::test(pollster::test)]
async fn ac5_ersetzen_erzeugt_eine_neue_identitaet() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, ziel_id, parkour_id) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;

    let nachher = harness
        .app()
        .execute(
            admin,
            ZielErsetzen {
                anlage_id: anlage.id(),
                ziel_id,
                tiername: tiername("Neuer Keiler"),
            },
        )
        .await
        .expect("AC-5: das physische Ziel wird ersetzt");

    let altes = nachher
        .ziel(ziel_id)
        .expect("AC-5: die alte Identität bleibt");
    assert!(altes.ist_archiviert(), "AC-5: sie ist archiviert");
    let neues = nachher
        .ziele()
        .iter()
        .find(|ziel| ziel.id() != ziel_id)
        .expect("AC-5: eine neue Identität entsteht");
    assert_eq!(neues.tiername().as_str(), "Neuer Keiler");

    let parkour = nachher
        .parkour_mit_id(parkour_id)
        .expect("der Parkour bleibt");
    assert_eq!(
        parkour.platzierungen()[0].ziel_id(),
        neues.id(),
        "AC-5: das neue Ziel übernimmt den Platz, nicht die Historie"
    );
}

// --------------------------------------------------------------------------- AC-6

#[test_log::test(pollster::test)]
async fn ac6_pflock_farben_sind_je_parkour_eindeutig() {
    let doppelt = vec![
        Pflock::neu(
            PflockId(uuid::Uuid::new_v4()),
            Farbe::neu("#c0392b").expect("gültig"),
        ),
        Pflock::neu(
            PflockId(uuid::Uuid::new_v4()),
            Farbe::neu("#C0392B").expect("gültig"),
        ),
    ];
    let ergebnis = NeuerParkour::neu(
        NonEmptyString::neu("Waldrunde").expect("gültig"),
        doppelt,
        Vec::new(),
    );
    assert!(ergebnis.is_err(), "AC-6: die Farben sind eindeutig");
}

#[test_log::test(pollster::test)]
async fn ac6_konfigurierte_abstaende_nehmen_streng_von_nah_nach_fern_zu() {
    let weit = PflockId(uuid::Uuid::new_v4());
    let nah = PflockId(uuid::Uuid::new_v4());
    let ziel = ZielId(uuid::Uuid::new_v4());
    let pflöcke = vec![
        Pflock::neu(weit, Farbe::neu("#c0392b").expect("gültig")),
        Pflock::neu(nah, Farbe::neu("#1b4f9c").expect("gültig")),
    ];

    let gleich = Zielplatzierung::neu(ziel, vec![(weit, abstand(30)), (nah, abstand(30))])
        .expect("eine Platzierung prüft die Pflockliste nicht selbst");
    let ergebnis = NeuerParkour::neu(
        NonEmptyString::neu("Waldrunde").expect("gültig"),
        pflöcke.clone(),
        vec![gleich],
    );
    assert!(
        ergebnis.is_err(),
        "AC-6: gleiche Abstände nehmen nicht streng zu"
    );

    let ohne_abstand =
        Zielplatzierung::neu(ziel, Vec::new()).expect("fehlende Abstände sind erlaubt");
    NeuerParkour::neu(
        NonEmptyString::neu("Waldrunde").expect("gültig"),
        pflöcke,
        vec![ohne_abstand],
    )
    .expect("AC-6: fehlende Abstände werden angenommen");
}

// --------------------------------------------------------------------------- AC-7 / AC-9

#[test_log::test(pollster::test)]
async fn ac7_ac9_der_snapshot_bleibt_von_spaeteren_aenderungen_unberuehrt() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, ziel_id, parkour_id) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;

    let runde = harness
        .app()
        .execute(admin, RundeStarten { parkour_id })
        .await
        .expect("AC-7: eine Runde startet");
    let vorher = runde.snapshot().clone();
    let anlage_id = anlage.id();

    harness
        .app()
        .execute(
            admin,
            ZielÄndern {
                anlage_id,
                ziel_id,
                tiername: tiername("Reh"),
            },
        )
        .await
        .expect("Ziel ändern");

    let (pflöcke, platzierungen) = parkour_setup(ziel_id, None);
    harness
        .app()
        .execute(
            admin,
            ParkourSetupÄndern {
                anlage_id,
                parkour_id,
                pflöcke,
                platzierungen,
            },
        )
        .await
        .expect("Parkour-Setup ändern");

    // AC-9: the existing Runde still displays L1 exactly.
    let gelesen = runde_ansicht(&harness, runde.id()).await;
    assert_eq!(gelesen.runde.snapshot(), &vorher);
    assert_eq!(gelesen.ziele[0].tiername.as_str(), "Keiler");
    assert_eq!(gelesen.ziele[0].zielnummer.get(), 1);
    assert_eq!(
        gelesen.ziele[0].abstände,
        vec![
            (Farbe::neu("#1b4f9c").unwrap(), abstand(20)),
            (Farbe::neu("#c0392b").unwrap(), abstand(40)),
        ]
    );
    assert_eq!(
        gelesen.runde.snapshot().parkour_name().as_str(),
        "Waldrunde",
        "AC-9: der Name ist Teil des Schnappschusses"
    );

    // AC-7: a Runde started after the edit captures L2.
    let neue = harness
        .app()
        .execute(admin, RundeStarten { parkour_id })
        .await
        .expect("AC-7: eine neue Runde startet");
    assert_eq!(neue.snapshot().ziele()[0].tiername().as_str(), "Reh");
    assert_ne!(neue.snapshot(), &vorher);
}

// --------------------------------------------------------------------------- AC-8

#[test_log::test(pollster::test)]
async fn ac8_die_zielhistorie_bleibt_ueber_runden_und_parkour_hinweg_kontinuierlich() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, ziel_id, parkour_id) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;

    let erste = harness
        .app()
        .execute(admin, RundeStarten { parkour_id })
        .await
        .expect("die erste Runde startet");
    harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: erste.id(),
                ziel_id,
                pfeil: 1,
                wertungszone: Wertungszone::Kill,
            },
        )
        .await
        .expect("AC-8: der Schuss hängt an der physischen Zielidentität");
    harness
        .app()
        .execute(
            admin,
            RundeAbschliessen {
                runde_id: erste.id(),
            },
        )
        .await
        .expect("die Runde wird abgeschlossen");

    // A second Parkour placing the same physical Ziel, with its own distances.
    let (pflöcke, platzierungen) = parkour_setup(ziel_id, None);
    let nachher = harness
        .app()
        .execute(
            admin,
            ParkourHinzufügen {
                anlage_id: anlage.id(),
                name: NonEmptyString::neu("Feldrunde").expect("gültig"),
                pflöcke,
                platzierungen,
            },
        )
        .await
        .expect("ein zweiter Parkour entsteht");
    let zweiter_parkour = nachher
        .parkour()
        .last()
        .expect("der zweite Parkour existiert")
        .id();
    assert_ne!(zweiter_parkour, parkour_id);

    let zweite = harness
        .app()
        .execute(
            admin,
            RundeStarten {
                parkour_id: zweiter_parkour,
            },
        )
        .await
        .expect("die zweite Runde startet");
    harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: zweite.id(),
                ziel_id,
                pfeil: 1,
                wertungszone: Wertungszone::Spot,
            },
        )
        .await
        .expect("AC-8: derselbe Ziel-Datensatz nimmt einen zweiten Schuss an");

    let eine = runde_ansicht(&harness, erste.id()).await;
    let andere = runde_ansicht(&harness, zweite.id()).await;

    assert_eq!(eine.ziele[0].ziel_id, ziel_id);
    assert_eq!(andere.ziele[0].ziel_id, ziel_id);
    assert_eq!(eine.gesamt_erster_treffer, 18);
    assert_eq!(andere.gesamt_erster_treffer, 20);
    assert_eq!(
        andere.ziele[0].abstände,
        vec![
            (Farbe::neu("#1b4f9c").unwrap(), abstand(20)),
            (Farbe::neu("#c0392b").unwrap(), abstand(40)),
        ],
        "AC-8: der zweite Parkour bringt seine eigenen Abstände mit"
    );
}

// --------------------------------------------------------------------------- AC-10

#[test_log::test(pollster::test)]
async fn ac10_deaktivieren_verhindert_neue_runden_und_haelt_alte_lesbar() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, _, parkour_id) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;

    let runde = harness
        .app()
        .execute(admin, RundeStarten { parkour_id })
        .await
        .expect("eine Runde vor der Deaktivierung");

    let archiviert = harness
        .app()
        .execute(
            admin,
            AnlageDeaktivieren {
                anlage_id: anlage.id(),
            },
        )
        .await
        .expect("AC-10: ein Administrator deaktiviert");
    assert!(archiviert.ist_archiviert());

    let fehler = harness
        .app()
        .execute(admin, RundeStarten { parkour_id })
        .await
        .expect_err("AC-10: keine neuen Runden");
    assert!(matches!(
        fehler.current_context(),
        RundeStartenFehler::NichtGefunden | RundeStartenFehler::NichtAktiv
    ));

    runde_ansicht(&harness, runde.id()).await;
}

#[test_log::test(pollster::test)]
async fn ac10_ein_mitarbeiter_deaktiviert_nicht() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let mitarbeiter = benutzer_mit_ip(&harness, "mitarbeiter@muster.at", 2).await;
    let anlage = anlage_erstellen(&harness, admin, "Breitenstein").await;
    verwaltung_gewähren(
        &harness,
        admin,
        anlage.id(),
        id_von(mitarbeiter),
        AnlageVerwaltungRolle::Mitarbeiter,
    )
    .await;

    let fehler = harness
        .app()
        .execute(
            mitarbeiter,
            AnlageDeaktivieren {
                anlage_id: anlage.id(),
            },
        )
        .await
        .expect_err("AC-10: ein Mitarbeiter deaktiviert nicht");
    assert!(matches!(
        fehler.current_context(),
        StammdatenFehler::NichtErlaubt
    ));
}

// --------------------------------------------------------------------------- AC-11 / AC-12

#[test_log::test(pollster::test)]
async fn ac11_ein_referenziertes_ziel_wird_archiviert_nicht_geloescht() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, ziel_id, parkour_id) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;
    harness
        .app()
        .execute(admin, RundeStarten { parkour_id })
        .await
        .expect("eine Runde referenziert das Ziel");

    let nachher = harness
        .app()
        .execute(
            admin,
            ZielEntfernen {
                anlage_id: anlage.id(),
                ziel_id,
            },
        )
        .await
        .expect("AC-11: Entfernen archiviert");
    let ziel = nachher.ziel(ziel_id).expect("AC-11: die Identität bleibt");
    assert!(ziel.ist_archiviert());
    assert!(
        nachher.parkour()[0].platzierungen().is_empty(),
        "AC-11: es verlässt die aktuelle Route"
    );
}

#[test_log::test(pollster::test)]
async fn ac11_ein_archiviertes_ziel_verweigert_jede_aenderung() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, ziel_id, parkour_id) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;
    harness
        .app()
        .execute(admin, RundeStarten { parkour_id })
        .await
        .expect("eine Runde referenziert das Ziel");
    harness
        .app()
        .execute(
            admin,
            ZielEntfernen {
                anlage_id: anlage.id(),
                ziel_id,
            },
        )
        .await
        .expect("das Ziel wird archiviert");

    let fehler = harness
        .app()
        .execute(
            admin,
            ZielÄndern {
                anlage_id: anlage.id(),
                ziel_id,
                tiername: tiername("Reh"),
            },
        )
        .await
        .expect_err("AC-11: archiviert ist unveränderlich");
    assert!(
        matches!(fehler.current_context(), StammdatenFehler::UngültigeAnfrage),
        "AC-11: die Änderung wird als ungültige Anfrage abgelehnt"
    );
}

#[test_log::test(pollster::test)]
async fn ac11_eine_deaktivierte_anlage_verweigert_schreibzugriffe() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, _, _) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;
    harness
        .app()
        .execute(
            admin,
            AnlageDeaktivieren {
                anlage_id: anlage.id(),
            },
        )
        .await
        .expect("deaktivieren");

    let fehler = harness
        .app()
        .execute(
            admin,
            ZielHinzufügen {
                anlage_id: anlage.id(),
                tiername: tiername("Reh"),
            },
        )
        .await
        .expect_err("AC-11: eine deaktivierte Anlage nimmt keine Änderung an");
    assert!(matches!(
        fehler.current_context(),
        StammdatenFehler::Archiviert
    ));
}

#[test_log::test(pollster::test)]
async fn ac11_ein_referenzierter_parkour_wird_archiviert() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, _, parkour_id) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;
    harness
        .app()
        .execute(admin, RundeStarten { parkour_id })
        .await
        .expect("eine Runde referenziert den Parkour");

    let nachher = harness
        .app()
        .execute(
            admin,
            ParkourEntfernen {
                anlage_id: anlage.id(),
                parkour_id,
            },
        )
        .await
        .expect("AC-11: Entfernen archiviert");
    assert!(
        nachher
            .parkour_mit_id(parkour_id)
            .expect("der Parkour bleibt")
            .ist_archiviert()
    );
}

#[test_log::test(pollster::test)]
async fn ac12_ein_unreferenziertes_ziel_wird_geloescht() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, ziel_id, _) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;

    let nachher = harness
        .app()
        .execute(
            admin,
            ZielEntfernen {
                anlage_id: anlage.id(),
                ziel_id,
            },
        )
        .await
        .expect("AC-12: ohne Historie wird gelöscht");
    assert!(nachher.ziel(ziel_id).is_none());
}

#[test_log::test(pollster::test)]
async fn ac12_ein_unreferenzierter_parkour_wird_geloescht() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, _, parkour_id) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;

    let nachher = harness
        .app()
        .execute(
            admin,
            ParkourEntfernen {
                anlage_id: anlage.id(),
                parkour_id,
            },
        )
        .await
        .expect("AC-12: ohne Historie wird gelöscht");
    assert!(nachher.parkour_mit_id(parkour_id).is_none());
}

// --------------------------------------------------------------------------- AC-13

// The public Parkour list is composed at the HTTP edge and pinned there
// (`e2e/stammdaten.rs::ac13_die_parkourliste_ist_oeffentlich_und_zeigt_nur_aktive_parkour`);
// `ParkourAnzeigen` is no longer a use case, so no core case drives it.

#[test_log::test(pollster::test)]
async fn ac13_ein_mitarbeiter_liest_keine_fremde_runde() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let mitarbeiter = benutzer_mit_ip(&harness, "mitarbeiter@muster.at", 2).await;
    let (anlage, _, parkour_id) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;
    verwaltung_gewähren(
        &harness,
        admin,
        anlage.id(),
        id_von(mitarbeiter),
        AnlageVerwaltungRolle::Mitarbeiter,
    )
    .await;

    let fremde_runde = harness
        .app()
        .execute(admin, RundeStarten { parkour_id })
        .await
        .expect("die Runde gehört dem Systemadministrator");

    // AC-13: the owner-or-Systemadministrator rule is a person rule, not an Anlage role, so
    // the Mitarbeiter's Verwaltung grants nothing. The HTTP suite pins the 403.
    assert!(matches!(
        fremde_runde.erfasser(),
        logandaim_core::domain::Erfasser::Benutzer(eigner)
            if eigner != id_von(mitarbeiter)
    ));
}

#[test_log::test(pollster::test)]
async fn ac13_der_systemadministrator_handelt_anlagenuebergreifend() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let (_, _, parkour_id) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;

    let runde = harness
        .app()
        .execute(anna, RundeStarten { parkour_id })
        .await
        .expect("Anna startet eine Runde");

    runde_ansicht(&harness, runde.id()).await;
}

// --------------------------------------------------------------------------- Listen

// The admin list and the Anlage detail are HTTP reads now; both are pinned in
// `e2e/stammdaten.rs`. What stays here is the Anlage's own placement aggregation, which is
// domain data the admin view renders.

#[test_log::test(pollster::test)]
async fn die_anlage_zeigt_wo_ihre_ziele_verwendet_werden() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage, _, _) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;

    let ansicht = anlage_ansicht(&harness, anlage.id()).await;
    assert_eq!(ansicht.anlage.id(), anlage.id());
    assert_eq!(ansicht.zielverwendungen.len(), 1);
    let verwendungen = &ansicht.zielverwendungen[0].1;
    assert_eq!(verwendungen.len(), 1);
    let parkour = ansicht
        .anlage
        .parkour_mit_id(verwendungen[0].parkour_id)
        .expect("der Parkour der Verwendung existiert");
    assert_eq!(parkour.name().as_str(), "Waldrunde");
    assert_eq!(
        verwendungen[0]
            .abstände
            .iter()
            .map(|(pflock_id, abstand)| (
                parkour
                    .pflöcke()
                    .iter()
                    .find(|pflock| pflock.id() == *pflock_id)
                    .expect("der Pflock der Verwendung existiert")
                    .farbe(),
                *abstand
            ))
            .collect::<Vec<_>>(),
        vec![
            (Farbe::neu("#1b4f9c").unwrap(), abstand(20)),
            (Farbe::neu("#c0392b").unwrap(), abstand(40)),
        ]
    );
}

/// The clock is part of the contract: an Anlage deactivated "now" records this instant.
#[test_log::test(pollster::test)]
async fn deaktivieren_stempelt_den_zeitpunkt() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let anlage = anlage_erstellen(&harness, admin, "Breitenstein").await;

    let archiviert = harness
        .app()
        .execute(
            admin,
            AnlageDeaktivieren {
                anlage_id: anlage.id(),
            },
        )
        .await
        .expect("deaktivieren");
    assert_eq!(
        archiviert
            .archiviert()
            .expect("archiviert")
            .deaktiviert_am(),
        jetzt()
    );
}

/// NFR-3: a Benutzer deletion anonymises the rows instead of destroying the record.
#[test_log::test(pollster::test)]
async fn ein_schuss_ueberlebt_das_loeschen_des_benutzers_anonymisiert() {
    use logandaim_core::{domain::Erfasser, service::BenutzerLöschen};

    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let (_, ziel_id, parkour_id) = anlage_mit_parkour(&harness, admin, "Breitenstein").await;

    let runde = harness
        .app()
        .execute(anna, RundeStarten { parkour_id })
        .await
        .expect("Anna startet eine Runde");
    harness
        .app()
        .execute(
            anna,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id,
                pfeil: 1,
                wertungszone: Wertungszone::Kill,
            },
        )
        .await
        .expect("Anna registriert einen Schuss");

    harness
        .app()
        .execute(
            anna,
            BenutzerLöschen {
                passwort: KlartextPasswort::neu(PASSWORT).expect("Testpasswort ist gültig"),
                bestätigt: true,
            },
        )
        .await
        .expect("das Konto wird gelöscht");

    let ansicht = runde_ansicht(&harness, runde.id()).await;
    assert!(matches!(
        ansicht.runde.erfasser(),
        Erfasser::Anonymisiert { .. }
    ));
    assert_eq!(ansicht.schüsse.len(), 1);
    assert!(
        matches!(ansicht.schüsse[0].erfasser(), Erfasser::Anonymisiert { .. }),
        "der Schuss überlebt, ohne den Benutzer zu nennen"
    );
    assert_eq!(ansicht.gesamt_erster_treffer, 18);
}
