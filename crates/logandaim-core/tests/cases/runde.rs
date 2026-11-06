//! Shared Runde and Schuss cases, run twice: against the fakes and against SQLite.
//!
//! These pin the archer-facing contract: start a Runde from an active Parkour, log arrows
//! against the captured route only, read the snapshot back exactly as captured, and close
//! the Runde. AC-9 is the spine — nothing an archer sees may change after the fact.

use std::net::{IpAddr, Ipv4Addr};

use logandaim_core::{
    application::AuthContext,
    domain::{
        Abstand, AnlageAdresse, AnlageId, AnlageKontakt, EmailAdresse, Erfasser, Farbe, Ländercode,
        NonEmptyString, ParkourId, Pflock, PflockId, Schussdistanz, Schussposition, Wertungszone,
        ZielId, Zielplatzierung,
        konto::KlartextPasswort,
        konto::{BenutzerId, SecretToken},
    },
    ports::ClientKey,
    service::{
        AnlageDeaktivieren, AnlageErstellen, ParkourEntfernen, ParkourHinzufügen,
        RegistrierungAbschließen, RegistrierungStarten, RundeAbschliessen, RundeAbschliessenFehler,
        RundeDatumÄndern, RundeLöschen, RundeStarten, RundeStartenFehler, SchussLöschen,
        SchussRegistrieren, SchussRegistrierenFehler, SchussÄndern, SessionAusgestellt,
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
) -> (AnlageId, ZielId, ParkourId) {
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
            RundeStarten {
                parkour_id,
                datum: None,
            },
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
            RundeStarten {
                parkour_id: ParkourId(uuid::Uuid::new_v4()),
                datum: None,
            },
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
        .await
        .expect("die Runde des Systemadministrators");

    let fehler = harness
        .app()
        .execute(
            anna,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id,
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
        .await
        .expect("die Runde startet");

    let fehler = harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id: ZielId(uuid::Uuid::new_v4()),
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
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
                ziel_id,
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
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
                    ziel_id,
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
        .execute(
            anna,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
        .await
        .expect("Anna startet eine Runde");

    let schuss = harness
        .app()
        .execute(
            anna,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id,
                wertungszone: Wertungszone::Spot,
                position: None,
            },
        )
        .await
        .expect("der Schuss wird registriert");
    assert_eq!(schuss.erfasser(), Erfasser::Benutzer(anna_id));
    assert_eq!(schuss.runde_id(), runde.id());
    assert_eq!(schuss.ziel_id(), ziel_id);
}

// --------------------------------------------------------------------------- AC-13

#[test_log::test(pollster::test)]
async fn ac13_ein_benutzer_hat_hoechstens_eine_offene_runde() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let erste = harness
        .app()
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
        .await
        .expect("die Runde startet");

    let fehler = harness
        .app()
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
        .await
        .expect_err("eine zweite offene Runde wird abgelehnt");
    assert!(matches!(
        fehler.current_context(),
        RundeStartenFehler::RundeLaeuft
    ));
    assert_eq!(
        harness
            .app()
            .execute(
                admin,
                RundeStarten {
                    parkour_id,
                    datum: None
                }
            )
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
        .await
        .expect("nach Abschluss ist eine neue Runde möglich");
}

#[test_log::test(pollster::test)]
async fn ac13_gleichzeitiger_start_laesst_genau_eine_offene_runde() {
    let harness = harness().await;
    let admin = systemadministrator(&harness, "admin@muster.at").await;
    let (_, _, parkour_id) = parkour_mit_einem_ziel(&harness, admin).await;
    let (links, rechts) = futures::join!(
        harness.app().execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None
            }
        ),
        harness.app().execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None
            }
        ),
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
        .await
        .expect("die Runde startet");
    let snapshot = runde.snapshot().clone();
    let erster = harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id,
                wertungszone: Wertungszone::Kill,
                position: Some(Schussposition::Distanz(Schussdistanz::neu(35).unwrap())),
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
                ziel_id,
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
    let neuer_tag = chrono::NaiveDate::from_ymd_opt(2026, 11, 6).unwrap();
    let geändert = harness
        .app()
        .execute(
            admin,
            RundeDatumÄndern {
                runde_id: runde.id(),
                datum: neuer_tag,
            },
        )
        .await
        .expect("Datum wird geändert");
    assert_eq!(geändert.datum(), neuer_tag);
    assert_eq!(geändert.snapshot(), &snapshot);
    let schüsse = super::helpers::schuesse_laden(&harness, runde.id()).await;
    assert_eq!(geändert.wertung(schüsse.clone()).erster_treffer(), 14);
    assert_eq!(geändert.wertung(schüsse).mehrschuss(), 14);

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
    assert_eq!(geladen.snapshot(), &snapshot);
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
        .await
        .expect("die Runde startet");
    harness
        .app()
        .execute(
            admin,
            SchussRegistrieren {
                runde_id: runde.id(),
                ziel_id,
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
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
        .execute(
            admin,
            RundeStarten {
                parkour_id,
                datum: None,
            },
        )
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
