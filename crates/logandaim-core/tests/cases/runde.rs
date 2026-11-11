//! Shared Runde and Schuss cases, run twice: against the fakes and against SQLite.
//!
//! These pin the archer-facing contract: start a Runde from an active Parkour, log arrows
//! against the captured route only, read the snapshot back exactly as captured, and close
//! the Runde. AC-9 is the spine — nothing an archer sees may change after the fact.

use std::net::{IpAddr, Ipv4Addr};

use logandaim_core::{
    application::AuthContext,
    domain::{
        Abstand, AnlageAdresse, AnlageId, AnlageKontakt, AnlageZielId, EmailAdresse, Erfasser,
        Farbe, Ländercode, NonEmptyString, ParkourId, Pflock, PflockId, RundeId, RundenZielId,
        Schussposition, Wertungszone, Zielplatzierung,
        konto::KlartextPasswort,
        konto::{BenutzerId, SecretToken},
    },
    ports::ClientKey,
    service::{
        AnlageDeaktivieren, AnlageErstellen, ParkourEntfernen, ParkourHinzufügen,
        RegistrierungAbschließen, RegistrierungStarten, RundeAbschliessen, RundeAbschliessenFehler,
        RundeLöschen, RundeStarten, RundeStartenFehler, RundeZielEntfernen,
        RundeZielEntfernenFehler, RundeZielHinzufügen, RundeZielHinzufügenFehler,
        RundeZielUmbenennen, RundeZielUmbenennenFehler, SchussLöschen, SchussRegistrieren,
        SchussRegistrierenFehler, SchussÄndern, SchussÄndernFehler, SessionAusgestellt,
        SystemadministratorBefördern, ZielHinzufügen,
    },
};

use super::{super::base_app_builder, helpers::Harness};

const PASSWORT: &str = "korrekt-pferd-batterie";

fn client_key(ip: u8) -> ClientKey {
    ClientKey::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, ip)), "runde-case")
}

fn adresse(wert: &str) -> EmailAdresse {
    EmailAdresse::normalisiert(wert).expect("Testadresse ist gültig")
}

async fn harness() -> Harness {
    super::helpers::build(base_app_builder().await)
}

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

async fn systemadministrator(harness: &Harness, email: &str) -> AuthContext {
    let auth = benutzer_mit_ip(harness, email, 1).await;
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

/// One Anlage with a two-Pflock Parkour and one Ziel, created by `auth`.
async fn parkour_mit_einem_ziel(
    harness: &Harness,
    auth: AuthContext,
) -> (AnlageId, AnlageZielId, ParkourId) {
    let anlage = harness
        .app()
        .execute(
            auth,
            AnlageErstellen {
                name: NonEmptyString::neu("Breitenstein").expect("gültig"),
                adresse: AnlageAdresse::neu("Waldweg 1", "3100", "St. Pölten", Ländercode::AT)
                    .expect("gültig"),
                kontakt: AnlageKontakt::leer(),
            },
        )
        .await
        .expect("die Anlage entsteht");
    let anlage_id = anlage.id();

    let anlage = harness
        .app()
        .execute(
            auth,
            ZielHinzufügen {
                anlage_id,
                tiername: NonEmptyString::neu("Keiler").expect("gültig"),
            },
        )
        .await
        .expect("das Ziel entsteht");
    let ziel_id = anlage.ziele().last().expect("ein Ziel").id();

    let weit = PflockId(uuid::Uuid::new_v4());
    let nah = PflockId(uuid::Uuid::new_v4());
    // Near to far: the first Pflock is the nearest, distances strictly increase.
    let pflöcke = vec![
        Pflock::neu(nah, Farbe::neu("#1b4f9c").expect("gültig")),
        Pflock::neu(weit, Farbe::neu("#c0392b").expect("gültig")),
    ];
    let platzierung = Zielplatzierung::neu(
        ziel_id,
        vec![
            (nah, Abstand::neu(20).expect("positiv")),
            (weit, Abstand::neu(40).expect("positiv")),
        ],
    )
    .expect("gültig");

    let anlage = harness
        .app()
        .execute(
            auth,
            ParkourHinzufügen {
                anlage_id,
                name: NonEmptyString::neu("Waldrunde").expect("gültig"),
                pflöcke,
                platzierungen: vec![platzierung],
            },
        )
        .await
        .expect("der Parkour entsteht");
    let parkour_id = anlage.parkour().last().expect("ein Parkour").id();

    (anlage_id, ziel_id, parkour_id)
}

// --------------------------------------------------------------------------- AC-9

// The Runde read is composed at the HTTP edge now; its snapshot payload is pinned there
// (`e2e/runde.rs::ac9_eine_runde_zeigt_beide_wertungen_und_den_snapshot`). The rule that a
// captured snapshot never moves is proven by `ac7_ac9_der_snapshot_bleibt_von_spaeteren_
// aenderungen_unberuehrt` in `stammdaten.rs`.

// --------------------------------------------------------------------------- Starten

#[test_log::test(pollster::test)]
async fn starten_braucht_eine_anmeldung() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;

    let fehler = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RundeStarten::mit_parkour(parkour_id),
        )
        .await
        .expect_err("ein unangemeldeter Start antwortet 401");
    assert!(matches!(
        fehler.current_context(),
        RundeStartenFehler::NichtAngemeldet
    ));
}

#[test_log::test(pollster::test)]
async fn starten_auf_einen_unbekannten_parkour_ist_404() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    parkour_mit_einem_ziel(&harness, admin).await;

    let fehler = harness
        .app()
        .execute(
            admin,
            RundeStarten::mit_parkour(ParkourId(uuid::Uuid::new_v4())),
        )
        .await
        .expect_err("ein unbekannter Parkour ist 404");
    assert!(matches!(
        fehler.current_context(),
        RundeStartenFehler::NichtGefunden
    ));
}

#[test_log::test(pollster::test)]
async fn starten_auf_einen_archivierten_parkour_ist_409() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage_id, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("eine Runde referenziert den Parkour");
    harness
        .app()
        .execute(
            admin,
            ParkourEntfernen {
                anlage_id,
                parkour_id,
            },
        )
        .await
        .expect("der Parkour wird archiviert");

    let fehler = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect_err("ein archivierter Parkour ist 409");
    assert!(matches!(
        fehler.current_context(),
        RundeStartenFehler::NichtAktiv
    ));
}

#[test_log::test(pollster::test)]
async fn starten_an_einer_deaktivierten_anlage_ist_abgelehnt() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (anlage_id, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    harness
        .app()
        .execute(admin, AnlageDeaktivieren { anlage_id })
        .await
        .expect("die Anlage wird deaktiviert");

    let fehler = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect_err("eine deaktivierte Anlage nimmt keine Runde an");
    assert!(matches!(
        fehler.current_context(),
        RundeStartenFehler::NichtGefunden | RundeStartenFehler::NichtAktiv
    ));
}

// --------------------------------------------------------------------------- Schüsse

#[test_log::test(pollster::test)]
async fn schuesse_zaehlen_nur_in_der_eigenen_offenen_runde() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let (_, ziel_id, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde des Systemadministrators");

    let fehler = harness
        .app()
        .execute(
            anna,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: ziel_id.0,
                wertungszone: Wertungszone::Kill,
                position: None,
            },
        )
        .await
        .expect_err("eine fremde Runde nimmt keine Schüsse an");
    assert!(matches!(
        fehler.current_context(),
        SchussRegistrierenFehler::NichtErlaubt
    ));
}

#[test_log::test(pollster::test)]
async fn ein_schuss_auf_ein_ziel_ausserhalb_des_snapshots_wird_abgelehnt() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde startet");

    let fehler = harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: uuid::Uuid::new_v4(),
                wertungszone: Wertungszone::Kill,
                position: None,
            },
        )
        .await
        .expect_err("AC-9: ein Ziel außerhalb des Schnappschusses ist 409");
    assert!(matches!(
        fehler.current_context(),
        SchussRegistrierenFehler::ZielNichtImSnapshot
    ));
}

#[test_log::test(pollster::test)]
async fn eine_abgeschlossene_runde_nimmt_keine_schuesse_mehr_an() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, ziel_id, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde startet");
    harness
        .app()
        .execute(
            admin,
            RundeAbschliessen {
                runde_id: runde.id(),
            },
        )
        .await
        .expect("die Runde wird abgeschlossen");

    let fehler = harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: ziel_id.0,
                wertungszone: Wertungszone::Kill,
                position: None,
            },
        )
        .await
        .expect_err("eine abgeschlossene Runde ist 409");
    assert!(matches!(
        fehler.current_context(),
        SchussRegistrierenFehler::RundeNichtInBearbeitung
    ));
}

#[test_log::test(pollster::test)]
async fn schuesse_erhalten_ihre_pfeilnummer_in_registrierungsreihenfolge() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, ziel_id, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde startet");

    for (index, zone) in [Wertungszone::Kill, Wertungszone::Spot]
        .into_iter()
        .enumerate()
    {
        let schuss = harness
            .app()
            .execute(
                admin,
                SchussRegistrieren {
                    runde_id: runde.id(),
                    ziel_id: ziel_id.0,
                    wertungszone: zone,
                    position: None,
                },
            )
            .await
            .expect("der Schuss wird registriert");
        assert_eq!(schuss.pfeil().get(), index + 1);
    }
}

#[test_log::test(pollster::test)]
async fn ein_schuss_traegt_den_angemeldeten_benutzer_als_eigentuemer() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let anna_id: BenutzerId = anna.benutzer_id().expect("angemeldet");
    let (_, ziel_id, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(anna, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("Anna startet eine Runde");

    let schuss = harness
        .app()
        .execute(
            anna,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: ziel_id.0,
                wertungszone: Wertungszone::Spot,
                position: None,
            },
        )
        .await
        .expect("der Schuss wird registriert");
    assert_eq!(schuss.erfasser(), Erfasser::Benutzer(anna_id));
    assert_eq!(schuss.runde_id(), runde.id());
    assert_eq!(schuss.ziel_id(), ziel_id.into());
}

// --------------------------------------------------------------------------- AC-13

#[test_log::test(pollster::test)]
async fn ac13_ein_benutzer_hat_hoechstens_eine_offene_runde() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let erste = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde startet");

    let fehler = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect_err("eine zweite offene Runde wird abgelehnt");
    assert!(matches!(
        fehler.current_context(),
        RundeStartenFehler::RundeLaeuft
    ));
    assert_eq!(
        harness
            .app()
            .execute(admin, RundeStarten::mit_parkour(parkour_id))
            .await
            .expect_err("die laufende Runde bleibt bestehen")
            .current_context()
            .to_string(),
        "für diesen Benutzer läuft bereits eine Runde"
    );

    harness
        .app()
        .execute(
            admin,
            RundeAbschliessen {
                runde_id: erste.id(),
            },
        )
        .await
        .expect("der Abschluss gibt den Slot frei");
    harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("nach Abschluss ist eine neue Runde möglich");
}

#[test_log::test(pollster::test)]
async fn ac13_gleichzeitiger_start_laesst_genau_eine_offene_runde() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let (links, rechts) = futures::join!(
        harness
            .app()
            .execute(admin, RundeStarten::mit_parkour(parkour_id)),
        harness
            .app()
            .execute(admin, RundeStarten::mit_parkour(parkour_id)),
    );
    assert_ne!(
        links.is_ok(),
        rechts.is_ok(),
        "genau ein paralleler Start gewinnt"
    );
    let fehler = if let Err(fehler) = links {
        fehler
    } else {
        rechts.unwrap_err()
    };
    assert!(matches!(
        fehler.current_context(),
        RundeStartenFehler::RundeLaeuft
    ));
}

#[test_log::test(pollster::test)]
async fn ac9_editieren_von_schuss_und_datum_erhaelt_snapshot_und_berechnet_wertung_neu() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, ziel_id, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde startet");
    let snapshot = match runde.aufbau() {
        logandaim_core::domain::Rundenaufbau::Parkour { snapshot, .. } => Some(snapshot.clone()),
        logandaim_core::domain::Rundenaufbau::Freie(_) => None,
    };
    let erster = harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: ziel_id.0,
                wertungszone: Wertungszone::Kill,
                position: Some(Schussposition::Distanz(Abstand::neu(35).unwrap())),
            },
        )
        .await
        .expect("erster Schuss");
    let zweiter = harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: ziel_id.0,
                wertungszone: Wertungszone::Spot,
                position: None,
            },
        )
        .await
        .expect("zweiter Schuss");

    let geändert = harness
        .app()
        .execute(
            admin,
            SchussÄndern {
                runde_id: runde.id(),
                schuss_id: erster.id(),
                wertungszone: Wertungszone::Miss,
                position: Some(Schussposition::Pflock(Farbe::neu("#1b4f9c").unwrap())),
            },
        )
        .await
        .expect("Schuss wird korrigiert");
    assert_eq!(
        geändert.position(),
        Some(Schussposition::Pflock(Farbe::neu("#1b4f9c").unwrap()))
    );
    let (geladen_nach_aenderung, schüsse) = super::helpers::runde_laden(&harness, runde.id()).await;
    assert_eq!(
        match geladen_nach_aenderung.aufbau() {
            logandaim_core::domain::Rundenaufbau::Parkour { snapshot, .. } => Some(snapshot),
            logandaim_core::domain::Rundenaufbau::Freie(_) => None,
        },
        snapshot.as_ref()
    );
    assert_eq!(
        geladen_nach_aenderung
            .wertung(schüsse.clone())
            .erster_treffer(),
        14
    );
    assert_eq!(geladen_nach_aenderung.wertung(schüsse).mehrschuss(), 14);

    harness
        .app()
        .execute(
            admin,
            SchussLöschen {
                runde_id: runde.id(),
                schuss_id: zweiter.id(),
            },
        )
        .await
        .expect("Schuss wird gelöscht");
    let (geladen, schüsse) = super::helpers::runde_laden(&harness, runde.id()).await;
    assert_eq!(
        match geladen.aufbau() {
            logandaim_core::domain::Rundenaufbau::Parkour { snapshot, .. } => Some(snapshot),
            logandaim_core::domain::Rundenaufbau::Freie(_) => None,
        },
        snapshot.as_ref()
    );
    assert_eq!(schüsse.len(), 1);
    assert_eq!(schüsse[0].pfeil().get(), 1);
    assert_eq!(geladen.wertung(schüsse).mehrschuss(), 0);
}

#[test_log::test(pollster::test)]
async fn ac10_runde_loeschen_entfernt_alle_schuesse() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, ziel_id, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde startet");
    harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: ziel_id.0,
                wertungszone: Wertungszone::Kill,
                position: None,
            },
        )
        .await
        .expect("Schuss");
    harness
        .app()
        .execute(
            admin,
            RundeLöschen {
                runde_id: runde.id(),
            },
        )
        .await
        .expect("die Runde wird mit Schüssen gelöscht");
    assert!(
        super::helpers::schuesse_laden(&harness, runde.id())
            .await
            .is_empty(),
        "gelöschte Runde hat keine Schüsse mehr"
    );
}

// --------------------------------------------------------------------------- Abschluss / Liste

#[test_log::test(pollster::test)]
async fn abschliessen_ist_einmalig_und_setzt_den_zustand() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde startet");

    let geschlossen = harness
        .app()
        .execute(
            admin,
            RundeAbschliessen {
                runde_id: runde.id(),
            },
        )
        .await
        .expect("die Runde wird abgeschlossen");
    assert_eq!(geschlossen.zustand(), "Abgeschlossen");
    assert!(geschlossen.abgeschlossen_am().is_some());

    let fehler = harness
        .app()
        .execute(
            admin,
            RundeAbschliessen {
                runde_id: runde.id(),
            },
        )
        .await
        .expect_err("zweimal abschließen ist 409");
    assert!(matches!(
        fehler.current_context(),
        RundeAbschliessenFehler::BereitsAbgeschlossen
    ));
}

#[test_log::test(pollster::test)]
async fn eine_fremde_runde_wird_nicht_abgeschlossen() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let (_, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde des Systemadministrators");

    let fehler = harness
        .app()
        .execute(
            anna,
            RundeAbschliessen {
                runde_id: runde.id(),
            },
        )
        .await
        .expect_err("nur der Eigentümer schließt ab");
    assert!(matches!(
        fehler.current_context(),
        RundeAbschliessenFehler::NichtErlaubt
    ));
}

// The list read and its newest-first order live at the HTTP edge
// (`e2e/runde.rs::die_rundenliste_zeigt_die_eigenen_runden_neueste_zuerst`), because
// `RundenAnzeigen` is no longer a use case.

// --------------------------------------------------------------------------- #72: per-Ziel Pflöcke

/// One Anlage with two Ziele: `erstes` carries its own Pflock set, `zweites` inherits the
/// Parkour-wide one. Returns `(ziel_id with own set, ziel_id inheriting, parkour_id, own colours)`.
async fn parkour_mit_eigenem_satz_je_ziel(
    harness: &Harness,
    auth: AuthContext,
) -> (AnlageZielId, AnlageZielId, ParkourId, Vec<Farbe>) {
    let anlage = harness
        .app()
        .execute(
            auth,
            AnlageErstellen {
                name: NonEmptyString::neu("Breitenstein").expect("gültig"),
                adresse: AnlageAdresse::neu("Waldweg 1", "3100", "St. Pölten", Ländercode::AT)
                    .expect("gültig"),
                kontakt: AnlageKontakt::leer(),
            },
        )
        .await
        .expect("die Anlage entsteht");
    let anlage_id = anlage.id();

    let mit_zielen = harness
        .app()
        .execute(
            auth,
            ZielHinzufügen {
                anlage_id,
                tiername: NonEmptyString::neu("Keiler").expect("gültig"),
            },
        )
        .await
        .expect("das erste Ziel entsteht");
    let erstes = mit_zielen.ziele().last().expect("ein Ziel").id();
    let mit_zielen = harness
        .app()
        .execute(
            auth,
            ZielHinzufügen {
                anlage_id,
                tiername: NonEmptyString::neu("Reh").expect("gültig"),
            },
        )
        .await
        .expect("das zweite Ziel entsteht");
    let zweites = mit_zielen.ziele().last().expect("ein Ziel").id();

    let weit = PflockId(uuid::Uuid::new_v4());
    let nah = PflockId(uuid::Uuid::new_v4());
    let pflöcke = vec![
        Pflock::neu(nah, Farbe::neu("#1b4f9c").expect("gültig")),
        Pflock::neu(weit, Farbe::neu("#c0392b").expect("gültig")),
    ];

    let eigen_nah = PflockId(uuid::Uuid::new_v4());
    let eigen_weit = PflockId(uuid::Uuid::new_v4());
    let eigen_nah_farbe = Farbe::neu("#2e8b57").expect("gültig");
    let eigen_weit_farbe = Farbe::neu("#e8b81c").expect("gültig");
    let eigene = vec![
        Pflock::neu(eigen_nah, eigen_nah_farbe),
        Pflock::neu(eigen_weit, eigen_weit_farbe),
    ];

    let platzierungen = vec![
        Zielplatzierung::neu_mit_eigenen_pflöcken(
            erstes,
            eigene,
            vec![
                (eigen_nah, Abstand::neu(15).expect("positiv")),
                (eigen_weit, Abstand::neu(45).expect("positiv")),
            ],
        )
        .expect("#72: der eigene Satz ist gültig"),
        Zielplatzierung::neu(zweites, Vec::new()).expect("gültig"),
    ];

    let anlage = harness
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
        .expect("der Parkour entsteht");
    let parkour_id = anlage.parkour().last().expect("ein Parkour").id();

    (
        erstes,
        zweites,
        parkour_id,
        vec![eigen_nah_farbe, eigen_weit_farbe],
    )
}

#[test_log::test(pollster::test)]
async fn ac9_ein_schuss_am_eigenen_pflock_ist_nur_am_eigenen_ziel_erlaubt() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (erstes, zweites, parkour_id, eigene) =
        parkour_mit_eigenem_satz_je_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde startet");

    // The Ziel with its own set accepts a Pflock of that set...
    harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: erstes.0,
                wertungszone: Wertungszone::Kill,
                position: Some(Schussposition::Pflock(eigene[0])),
            },
        )
        .await
        .expect("#72: der eigene Pflock gilt an seinem Ziel");

    // ...while the inheriting Ziel refuses the same colour: it does not have that Pflock.
    let fehler = harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: zweites.0,
                wertungszone: Wertungszone::Kill,
                position: Some(Schussposition::Pflock(eigene[0])),
            },
        )
        .await
        .expect_err("#72: ein eigener Pflock gilt nicht an einem anderen Ziel");
    assert!(matches!(
        fehler.current_context(),
        SchussRegistrierenFehler::PflockNichtImSnapshot
    ));

    // The Parkour-wide colour stays selectable where the set is inherited...
    harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: zweites.0,
                wertungszone: Wertungszone::Kill,
                position: Some(Schussposition::Pflock(Farbe::neu("#c0392b").unwrap())),
            },
        )
        .await
        .expect("der Parkour-Pflock gilt am erbenden Ziel");

    // ...but not at the Ziel whose own set replaced it.
    let fehler = harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: erstes.0,
                wertungszone: Wertungszone::Kill,
                position: Some(Schussposition::Pflock(Farbe::neu("#c0392b").unwrap())),
            },
        )
        .await
        .expect_err("#72: ein ersetzter Parkour-Pflock gilt nicht mehr");
    assert!(matches!(
        fehler.current_context(),
        SchussRegistrierenFehler::PflockNichtImSnapshot
    ));
}

#[test_log::test(pollster::test)]
async fn ac9_eine_korrektur_prueft_den_pflock_gegen_das_ziel_des_schusses() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (erstes, zweites, parkour_id, eigene) =
        parkour_mit_eigenem_satz_je_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde startet");

    // A Schuss at the inheriting Ziel: it may only ever name that Ziel's effective set.
    let schuss = harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: zweites.0,
                wertungszone: Wertungszone::Kill,
                position: None,
            },
        )
        .await
        .expect("Schuss am erbenden Ziel");

    let fehler = harness
        .app()
        .execute(
            admin,
            SchussÄndern {
                runde_id: runde.id(),
                schuss_id: schuss.id(),
                wertungszone: Wertungszone::Spot,
                position: Some(Schussposition::Pflock(eigene[1])),
            },
        )
        .await
        .expect_err("#72: die Korrektur prüft gegen das Ziel des Schusses");
    assert!(matches!(
        fehler.current_context(),
        SchussÄndernFehler::PflockNichtImSnapshot
    ));

    // The Parkour-wide colour is accepted there.
    harness
        .app()
        .execute(
            admin,
            SchussÄndern {
                runde_id: runde.id(),
                schuss_id: schuss.id(),
                wertungszone: Wertungszone::Spot,
                position: Some(Schussposition::Pflock(Farbe::neu("#1b4f9c").unwrap())),
            },
        )
        .await
        .expect("der Parkour-Pflock wird angenommen");

    // And a correction at the own-set Ziel accepts its own colour.
    let eigener = harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: erstes.0,
                wertungszone: Wertungszone::Kill,
                position: None,
            },
        )
        .await
        .expect("Schuss am eigenen Ziel");
    harness
        .app()
        .execute(
            admin,
            SchussÄndern {
                runde_id: runde.id(),
                schuss_id: eigener.id(),
                wertungszone: Wertungszone::Kill,
                position: Some(Schussposition::Pflock(eigene[0])),
            },
        )
        .await
        .expect("der eigene Pflock wird angenommen");
}

// --------------------------------------------------------------------------- Freie Runde

fn name(wert: &str) -> NonEmptyString {
    NonEmptyString::neu(wert).expect("Testname ist gültig")
}

/// Start a `Freie Runde` owned by `auth`.
async fn freie_runde(
    harness: &Harness,
    auth: AuthContext,
    wert: &str,
) -> logandaim_core::domain::Runde {
    harness
        .app()
        .execute(auth, RundeStarten::ohne_parkour(name(wert)))
        .await
        .expect("die freie Runde startet")
}

#[test_log::test(pollster::test)]
async fn ac21_eine_freie_runde_startet_ohne_parkour() {
    let harness = harness().await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;

    let runde = freie_runde(&harness, anna, "Wiese daheim").await;
    let logandaim_core::domain::Rundenaufbau::Freie(frei) = runde.aufbau() else {
        panic!("die Runde ist frei");
    };
    assert_eq!(frei.name().as_str(), "Wiese daheim");
    assert!(frei.ziele().is_empty());

    // The stored setup survives a round trip through either repository.
    let (geladen, schüsse) = super::helpers::runde_laden(&harness, runde.id()).await;
    assert_eq!(geladen.id(), runde.id());
    let logandaim_core::domain::Rundenaufbau::Freie(frei) = geladen.aufbau() else {
        panic!("die Runde bleibt frei");
    };
    assert_eq!(frei.name().as_str(), "Wiese daheim");
    assert!(schüsse.is_empty());
}

#[test_log::test(pollster::test)]
async fn ac21_die_eine_offene_runde_gilt_ueber_beide_aufbauten() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let parkour_runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Parkour-Runde startet");

    let fehler = harness
        .app()
        .execute(admin, RundeStarten::ohne_parkour(name("Wiese daheim")))
        .await
        .expect_err("eine zweite offene Runde ist abgelehnt");
    assert!(matches!(
        fehler.current_context(),
        RundeStartenFehler::RundeLaeuft
    ));

    // And the other way round: a free round blocks a Parkour round (rule 20).
    harness
        .app()
        .execute(
            admin,
            RundeAbschliessen {
                runde_id: parkour_runde.id(),
            },
        )
        .await
        .expect("die Parkour-Runde wird geschlossen");
    let freie = freie_runde(&harness, admin, "Wiese daheim").await;

    let fehler = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect_err("eine freie Runde blockiert einen Parkour-Start");
    assert!(matches!(
        fehler.current_context(),
        RundeStartenFehler::RundeLaeuft
    ));

    // Closing the free round frees the slot again.
    harness
        .app()
        .execute(
            admin,
            RundeAbschliessen {
                runde_id: freie.id(),
            },
        )
        .await
        .expect("die freie Runde wird geschlossen");
    harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("nach dem Abschluss startet wieder eine Parkour-Runde");
}

#[test_log::test(pollster::test)]
async fn ac21_starten_und_ziel_editieren_brauchen_eine_anmeldung() {
    let harness = harness().await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let runde = freie_runde(&harness, anna, "Wiese daheim").await;

    let fehler = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RundeZielHinzufügen {
                runde_id: runde.id(),
                name: None,
            },
        )
        .await
        .expect_err("ein unangemeldeter Aufruf antwortet 401");
    assert!(matches!(
        fehler.current_context(),
        RundeZielHinzufügenFehler::NichtAngemeldet
    ));

    let fehler = harness
        .app()
        .execute(
            AuthContext::Anonymous,
            RundeZielEntfernen {
                runde_id: runde.id(),
                ziel_id: logandaim_core::domain::FreiesZielId(uuid::Uuid::new_v4()),
            },
        )
        .await
        .expect_err("ein unangemeldeter Aufruf antwortet 401");
    assert!(matches!(
        fehler.current_context(),
        RundeZielEntfernenFehler::NichtAngemeldet
    ));
}

#[test_log::test(pollster::test)]
async fn ac22_freie_ziele_werden_jit_hinzugefuegt_und_entfernt() {
    let harness = harness().await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let runde = freie_runde(&harness, anna, "Wiese daheim").await;

    let mit_einem = harness
        .app()
        .execute(
            anna,
            RundeZielHinzufügen {
                runde_id: runde.id(),
                name: None,
            },
        )
        .await
        .expect("das erste Ziel entsteht");
    let ziele: Vec<_> = mit_einem.aufbau().ziele().collect();
    assert_eq!(ziele.len(), 1);
    assert_eq!(ziele[0].name().as_str(), "Ziel 1");
    assert_eq!(ziele[0].ordinal().get(), 1);
    let RundenZielId::Frei(erstes) = ziele[0].ziel_id() else {
        panic!("freie Ziele haben freie Identitäten");
    };

    let mit_zwei = harness
        .app()
        .execute(
            anna,
            RundeZielHinzufügen {
                runde_id: runde.id(),
                name: Some(name("Baumstumpf")),
            },
        )
        .await
        .expect("das zweite Ziel entsteht");
    let ziele: Vec<_> = mit_zwei.aufbau().ziele().collect();
    assert_eq!(ziele[1].name().as_str(), "Baumstumpf");
    assert_eq!(ziele[1].ordinal().get(), 2);

    // The order and the names survive the repository, not just the in-memory aggregate.
    let (geladen, _) = super::helpers::runde_laden(&harness, runde.id()).await;
    let namen: Vec<String> = geladen
        .aufbau()
        .ziele()
        .map(|ziel| ziel.name().to_string())
        .collect();
    assert_eq!(namen, ["Ziel 1", "Baumstumpf"]);

    let nach_entfernen = harness
        .app()
        .execute(
            anna,
            RundeZielEntfernen {
                runde_id: runde.id(),
                ziel_id: erstes,
            },
        )
        .await
        .expect("das Ziel wird entfernt");
    assert_eq!(nach_entfernen.aufbau().ziele().count(), 1);

    // Removing an unknown Ziel is a 404, not a silent success.
    let fehler = harness
        .app()
        .execute(
            anna,
            RundeZielEntfernen {
                runde_id: runde.id(),
                ziel_id: erstes,
            },
        )
        .await
        .expect_err("ein unbekanntes Ziel ist nicht gefunden");
    assert!(matches!(
        fehler.current_context(),
        RundeZielEntfernenFehler::ZielNichtGefunden
    ));
}

#[test_log::test(pollster::test)]
async fn ac26_freie_ziele_lassen_sich_waehrend_der_runde_umbenennen() {
    let harness = harness().await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let runde = freie_runde(&harness, anna, "Wiese").await;
    let angelegt = harness
        .app()
        .execute(
            anna,
            RundeZielHinzufügen {
                runde_id: runde.id(),
                name: None,
            },
        )
        .await
        .expect("das Ziel wird angelegt");
    let RundenZielId::Frei(ziel_id) = angelegt.aufbau().ziele().next().unwrap().ziel_id() else {
        panic!("freie Ziele haben freie Identitäten");
    };

    let umbenannt = harness
        .app()
        .execute(
            anna,
            RundeZielUmbenennen {
                runde_id: runde.id(),
                ziel_id,
                name: name("Alter Apfelbaum"),
            },
        )
        .await
        .expect("das Ziel wird umbenannt");
    assert_eq!(
        umbenannt.aufbau().ziele().next().unwrap().name().as_str(),
        "Alter Apfelbaum"
    );

    let (geladen, _) = super::helpers::runde_laden(&harness, runde.id()).await;
    assert_eq!(
        geladen.aufbau().ziele().next().unwrap().name().as_str(),
        "Alter Apfelbaum"
    );
}

#[test_log::test(pollster::test)]
async fn ac26_freie_ziele_koennen_nur_in_eigenen_laufenden_freien_runden_umbenannt_werden() {
    let harness = harness().await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let bea = benutzer_mit_ip(&harness, "bea@muster.at", 3).await;
    let runde = freie_runde(&harness, anna, "Wiese").await;
    let angelegt = harness
        .app()
        .execute(
            anna,
            RundeZielHinzufügen {
                runde_id: runde.id(),
                name: None,
            },
        )
        .await
        .unwrap();
    let RundenZielId::Frei(ziel_id) = angelegt.aufbau().ziele().next().unwrap().ziel_id() else {
        panic!("freie Ziele haben freie Identitäten");
    };

    let fehler = harness
        .app()
        .execute(
            bea,
            RundeZielUmbenennen {
                runde_id: runde.id(),
                ziel_id,
                name: name("Fremd"),
            },
        )
        .await
        .expect_err("fremde Runde ist nicht bearbeitbar");
    assert!(matches!(
        fehler.current_context(),
        RundeZielUmbenennenFehler::NichtErlaubt
    ));

    let fehler = harness
        .app()
        .execute(
            anna,
            RundeZielUmbenennen {
                runde_id: RundeId(uuid::Uuid::new_v4()),
                ziel_id,
                name: name("Fehlt"),
            },
        )
        .await
        .expect_err("unbekannte Runde fehlt");
    assert!(matches!(
        fehler.current_context(),
        RundeZielUmbenennenFehler::NichtGefunden
    ));

    let unbekannt = harness
        .app()
        .execute(
            anna,
            RundeZielUmbenennen {
                runde_id: runde.id(),
                ziel_id: logandaim_core::domain::FreiesZielId(uuid::Uuid::new_v4()),
                name: name("Fehlt"),
            },
        )
        .await
        .expect_err("unbekanntes Ziel fehlt");
    assert!(matches!(
        unbekannt.current_context(),
        RundeZielUmbenennenFehler::ZielNichtGefunden
    ));

    harness
        .app()
        .execute(
            anna,
            RundeAbschliessen {
                runde_id: runde.id(),
            },
        )
        .await
        .expect("Runde wird abgeschlossen");
    let abgeschlossen = harness
        .app()
        .execute(
            anna,
            RundeZielUmbenennen {
                runde_id: runde.id(),
                ziel_id,
                name: name("Danach"),
            },
        )
        .await
        .expect_err("abgeschlossene Runde ist nicht umbenennbar");
    assert!(matches!(
        abgeschlossen.current_context(),
        RundeZielUmbenennenFehler::RundeNichtInBearbeitung
    ));
}

#[test_log::test(pollster::test)]
async fn ac22_ein_parkour_aufbau_nimmt_keine_freien_ziele_an() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let ziel_id = logandaim_core::domain::FreiesZielId(uuid::Uuid::new_v4());
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde startet");

    let fehler = harness
        .app()
        .execute(
            admin,
            RundeZielHinzufügen {
                runde_id: runde.id(),
                name: None,
            },
        )
        .await
        .expect_err("ein Parkour-Aufbau hat keinen freien Ziel-Mutator");
    assert!(matches!(
        fehler.current_context(),
        RundeZielHinzufügenFehler::RundeIstKeineFreieRunde
    ));

    let fehler = harness
        .app()
        .execute(
            admin,
            RundeZielEntfernen {
                runde_id: runde.id(),
                ziel_id,
            },
        )
        .await
        .expect_err("ein Parkour-Aufbau hat keinen freien Ziel-Mutator");
    assert!(matches!(
        fehler.current_context(),
        RundeZielEntfernenFehler::RundeIstKeineFreieRunde
    ));

    let fehler = harness
        .app()
        .execute(
            admin,
            RundeZielUmbenennen {
                runde_id: runde.id(),
                ziel_id,
                name: name("Geändert"),
            },
        )
        .await
        .expect_err("Parkour-Ziele sind in der Runde nicht umbenennbar");
    assert!(matches!(
        fehler.current_context(),
        RundeZielUmbenennenFehler::RundeIstKeineFreieRunde
    ));
}

#[test_log::test(pollster::test)]
async fn ac22_das_entfernen_eines_ziels_nimmt_seine_schuesse_mit() {
    let harness = harness().await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let runde = freie_runde(&harness, anna, "Wiese daheim").await;
    let mit_zielen = harness
        .app()
        .execute(
            anna,
            RundeZielHinzufügen {
                runde_id: runde.id(),
                name: None,
            },
        )
        .await
        .expect("das erste Ziel entsteht");
    let mit_zielen = harness
        .app()
        .execute(
            anna,
            RundeZielHinzufügen {
                runde_id: mit_zielen.id(),
                name: None,
            },
        )
        .await
        .expect("das zweite Ziel entsteht");
    let ziele: Vec<_> = mit_zielen.aufbau().ziele().collect();
    let logandaim_core::domain::RundenZielId::Frei(erstes) = ziele[0].ziel_id() else {
        panic!("freie Identität");
    };
    let logandaim_core::domain::RundenZielId::Frei(zweites) = ziele[1].ziel_id() else {
        panic!("freie Identität");
    };

    for ziel_id in [erstes, erstes, zweites] {
        harness
            .app()
            .execute(
                anna,
                SchussRegistrieren {
                    runde_id: runde.id(),
                    ziel_id: ziel_id.0,
                    wertungszone: Wertungszone::Kill,
                    position: None,
                },
            )
            .await
            .expect("der Schuss wird erfasst");
    }
    assert_eq!(
        super::helpers::schuesse_laden(&harness, runde.id())
            .await
            .len(),
        3
    );

    harness
        .app()
        .execute(
            anna,
            RundeZielEntfernen {
                runde_id: runde.id(),
                ziel_id: erstes,
            },
        )
        .await
        .expect("das Ziel wird entfernt");

    // The cascade is one write: the Ziel and its two Schüsse are gone, the other one stays.
    let schüsse = super::helpers::schuesse_laden(&harness, runde.id()).await;
    assert_eq!(
        schüsse.len(),
        1,
        "nur der Schuss am verbliebenen Ziel bleibt"
    );
    assert_eq!(schüsse[0].ziel_id(), zweites.into());
}

#[test_log::test(pollster::test)]
async fn ac22_fremde_runden_werden_nicht_editiert() {
    let harness = harness().await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let bert = benutzer_mit_ip(&harness, "bert@muster.at", 3).await;
    let runde = freie_runde(&harness, anna, "Wiese daheim").await;

    let fehler = harness
        .app()
        .execute(
            bert,
            RundeZielHinzufügen {
                runde_id: runde.id(),
                name: None,
            },
        )
        .await
        .expect_err("eine fremde Runde gehört nicht zu Bert");
    assert!(matches!(
        fehler.current_context(),
        RundeZielHinzufügenFehler::NichtErlaubt
    ));

    // A round that does not exist is 404, whatever the caller owns.
    let fehler = harness
        .app()
        .execute(
            anna,
            RundeZielHinzufügen {
                runde_id: logandaim_core::domain::RundeId(uuid::Uuid::new_v4()),
                name: None,
            },
        )
        .await
        .expect_err("eine erfundene Runde ist 404");
    assert!(matches!(
        fehler.current_context(),
        RundeZielHinzufügenFehler::NichtGefunden
    ));
}

#[test_log::test(pollster::test)]
async fn ac23_eine_freie_runde_nimmt_jede_farbe_und_jede_distanz() {
    let harness = harness().await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let runde = freie_runde(&harness, anna, "Wiese daheim").await;
    let mit_ziel = harness
        .app()
        .execute(
            anna,
            RundeZielHinzufügen {
                runde_id: runde.id(),
                name: None,
            },
        )
        .await
        .expect("das Ziel entsteht");
    let logandaim_core::domain::RundenZielId::Frei(ziel_id) = mit_ziel
        .aufbau()
        .ziele()
        .next()
        .expect("ein Ziel")
        .ziel_id()
    else {
        panic!("freie Ziele haben freie Identitäten");
    };

    // A colour the archer chose, and one the round has never seen: both are positions.
    for hex in ["#c0392b", "#1b4f9c", "#abc123"] {
        let schuss = harness
            .app()
            .execute(
                anna,
                SchussRegistrieren {
                    runde_id: runde.id(),
                    ziel_id: ziel_id.0,
                    wertungszone: Wertungszone::Kill,
                    position: Some(Schussposition::Pflock(Farbe::neu(hex).unwrap())),
                },
            )
            .await
            .expect("jede Farbe ist eine Position einer freien Runde");
        assert_eq!(
            schuss.position(),
            Some(Schussposition::Pflock(Farbe::neu(hex).unwrap()))
        );
    }

    let schuss = harness
        .app()
        .execute(
            anna,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: ziel_id.0,
                wertungszone: Wertungszone::Kill,
                position: Some(Schussposition::Distanz(Abstand::neu(35).unwrap())),
            },
        )
        .await
        .expect("eine Distanz ist eine Position");
    assert_eq!(
        schuss.position(),
        Some(Schussposition::Distanz(Abstand::neu(35).unwrap()))
    );

    // One exclusive value: replacing the position drops the other form entirely.
    let geändert = harness
        .app()
        .execute(
            anna,
            SchussÄndern {
                runde_id: runde.id(),
                schuss_id: schuss.id(),
                wertungszone: Wertungszone::Kill,
                position: Some(Schussposition::Pflock(Farbe::neu("#c0392b").unwrap())),
            },
        )
        .await
        .expect("die Korrektur wird angenommen");
    assert_eq!(
        geändert.position(),
        Some(Schussposition::Pflock(Farbe::neu("#c0392b").unwrap()))
    );

    // The stored row carries the colour and no distance, so a reader can never see both:
    // `Schussposition` is one exclusive value, not two columns that could disagree.
    let (_, schüsse) = super::helpers::runde_laden(&harness, runde.id()).await;
    let gespeichert = schüsse
        .iter()
        .find(|eintrag| eintrag.id() == schuss.id())
        .expect("der Schuss ist lesbar");
    assert_eq!(
        gespeichert.position(),
        Some(Schussposition::Pflock(Farbe::neu("#c0392b").unwrap()))
    );
}

#[test_log::test(pollster::test)]
async fn ac23_eine_unbekannte_farbe_ist_nur_auf_einem_parkour_abgelehnt() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, ziel_id, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let runde = harness
        .app()
        .execute(admin, RundeStarten::mit_parkour(parkour_id))
        .await
        .expect("die Runde startet");

    let fehler = harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: ziel_id.0,
                wertungszone: Wertungszone::Kill,
                // Not one of the Parkour's two captured colours.
                position: Some(Schussposition::Pflock(Farbe::neu("#abc123").unwrap())),
            },
        )
        .await
        .expect_err("ein fremder Pflock bleibt auf einem Parkour abgelehnt");
    assert!(matches!(
        fehler.current_context(),
        SchussRegistrierenFehler::PflockNichtImSnapshot
    ));
}

#[test_log::test(pollster::test)]
async fn ac24_die_wertung_faltet_ueber_die_freien_ziele() {
    let harness = harness().await;
    let anna = benutzer_mit_ip(&harness, "anna@muster.at", 2).await;
    let runde = freie_runde(&harness, anna, "Wiese daheim").await;
    let mit_zielen = harness
        .app()
        .execute(
            anna,
            RundeZielHinzufügen {
                runde_id: runde.id(),
                name: None,
            },
        )
        .await
        .expect("das erste Ziel entsteht");
    let mit_zielen = harness
        .app()
        .execute(
            anna,
            RundeZielHinzufügen {
                runde_id: mit_zielen.id(),
                name: None,
            },
        )
        .await
        .expect("das zweite Ziel entsteht");
    let ziele: Vec<_> = mit_zielen.aufbau().ziele().collect();
    let logandaim_core::domain::RundenZielId::Frei(erstes) = ziele[0].ziel_id() else {
        panic!("freie Identität");
    };
    let logandaim_core::domain::RundenZielId::Frei(zweites) = ziele[1].ziel_id() else {
        panic!("freie Identität");
    };

    for (ziel_id, zone) in [
        (erstes, Wertungszone::Kill),
        (erstes, Wertungszone::Spot),
        (zweites, Wertungszone::Miss),
    ] {
        harness
            .app()
            .execute(
                anna,
                SchussRegistrieren {
                    runde_id: runde.id(),
                    ziel_id: ziel_id.0,
                    wertungszone: zone,
                    position: None,
                },
            )
            .await
            .expect("der Schuss wird erfasst");
    }

    let (geladen, schüsse) = super::helpers::runde_laden(&harness, runde.id()).await;
    let wertung = geladen.wertung(schüsse);
    assert_eq!(wertung.erster_treffer(), 18);
    assert_eq!(wertung.mehrschuss(), 32);
    assert_eq!(wertung.ziele_mit_wertung(), 2);

    // Removing a Ziel recomputes the totals from the remaining Ziele (AC-22/AC-24).
    let nach_entfernen = harness
        .app()
        .execute(
            anna,
            RundeZielEntfernen {
                runde_id: runde.id(),
                ziel_id: zweites,
            },
        )
        .await
        .expect("das zweite Ziel wird entfernt");
    let (geladen, schüsse) = super::helpers::runde_laden(&harness, nach_entfernen.id()).await;
    let wertung = geladen.wertung(schüsse);
    assert_eq!(wertung.ziele_mit_wertung(), 1);
    assert_eq!(wertung.erster_treffer(), 18);
    assert_eq!(wertung.mehrschuss(), 32);
}
