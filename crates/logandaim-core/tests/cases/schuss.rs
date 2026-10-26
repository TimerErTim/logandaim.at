use uuid::Uuid;

use logandaim_core::{
    domain::{SchussId, Wertungszone, ZielId},
    service::{SchussRegistrieren, WertungBerechnen},
};

use super::{super::base_app_builder, helpers};

fn ziel_id() -> ZielId {
    ZielId(Uuid::new_v4())
}

#[test_log::test(pollster::test)]
async fn schuss_registrieren_persists_and_can_be_loaded() {
    let harness = helpers::build(base_app_builder().await);
    let app = harness.app();

    let schuss = app
        .execute(
            helpers::anonym(),
            SchussRegistrieren {
                ziel_id: ziel_id(),
                pfeil: 1,
                wertungszone: Wertungszone::Kill,
            },
        )
        .await
        .unwrap();

    let ids = [schuss.id()];
    let ergebnis = app
        .execute(
            helpers::anonym(),
            WertungBerechnen {
                schuss_ids: ids.to_vec(),
            },
        )
        .await
        .unwrap();

    assert_eq!(ergebnis.erster_treffer.value(), 18);
    assert_eq!(ergebnis.mehrschuss.value(), 18);
    assert!(ergebnis.erster_pfeil_getroffen);
    assert_eq!(ergebnis.anzahl_schüsse, 1);
}

#[test_log::test(pollster::test)]
async fn wertung_summiert_nach_pfeil_innerhalb_eines_ziels() {
    let harness = helpers::build(base_app_builder().await);
    let app = harness.app();
    let ziel = ziel_id();

    let erster = app
        .execute(
            helpers::anonym(),
            SchussRegistrieren {
                ziel_id: ziel,
                pfeil: 1,
                wertungszone: Wertungszone::Kill,
            },
        )
        .await
        .unwrap();
    let zweiter = app
        .execute(
            helpers::anonym(),
            SchussRegistrieren {
                ziel_id: ziel,
                pfeil: 2,
                wertungszone: Wertungszone::Kill,
            },
        )
        .await
        .unwrap();
    let dritter = app
        .execute(
            helpers::anonym(),
            SchussRegistrieren {
                ziel_id: ziel,
                pfeil: 3,
                wertungszone: Wertungszone::Spot,
            },
        )
        .await
        .unwrap();

    let ergebnis = app
        .execute(
            helpers::anonym(),
            WertungBerechnen {
                schuss_ids: vec![dritter.id(), erster.id(), zweiter.id()],
            },
        )
        .await
        .unwrap();

    assert_eq!(ergebnis.erster_treffer.value(), 18);
    assert_eq!(ergebnis.mehrschuss.value(), 38);
    assert!(ergebnis.erster_pfeil_getroffen);
}

#[test_log::test(pollster::test)]
async fn wertung_leere_liste_ist_fehler() {
    let harness = helpers::build(base_app_builder().await);
    let app = harness.app();

    let ergebnis = app
        .execute(
            helpers::anonym(),
            WertungBerechnen {
                schuss_ids: Vec::new(),
            },
        )
        .await;

    assert!(ergebnis.is_err());
}

#[test_log::test(pollster::test)]
async fn wertung_unbekannte_id_ist_fehler() {
    let harness = helpers::build(base_app_builder().await);
    let app = harness.app();

    let ergebnis = app
        .execute(
            helpers::anonym(),
            WertungBerechnen {
                schuss_ids: vec![SchussId(Uuid::new_v4())],
            },
        )
        .await;

    assert!(ergebnis.is_err());
}
