//! SPEC-stammdaten at the HTTP edge, Runde half.
//!
//! The rules — snapshot immutability, the ownership check, the fold — are proven in the
//! core suite. Here the question is what a client sees: 401 without a token, 403 for
//! another Benutzer's Runde, 409 for a Schuss against a Ziel the snapshot never captured,
//! and a Runde read that carries both Wertungen without a `/wertung` endpoint (AC-9, AC-13).

use poem::http::StatusCode;
use serde_json::{Value, json};
use uuid::Uuid;

use super::{E2eHarness, befördere_zum_systemadministrator, neuer_benutzer};

/// A Systemadministrator's token plus one Anlage with a Ziel and a Parkour:
/// `(admin, anlage_id, ziel_id, parkour_id)`.
async fn anlage_mit_parkour(harness: &E2eHarness) -> (String, String, String, String) {
    let (admin, _) = neuer_benutzer(harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(harness, "admin@muster.at").await;

    let (status, anlage) = harness
        .client
        .post_json_auth(
            "/api/anlagen",
            json!({
                "name": "Breitenstein",
                "adresse": {
                    "straße": "Waldweg 1",
                    "postleitzahl": "3100",
                    "ort": "St. Pölten",
                    "ländercode": "AT",
                },
                "kontakt": {},
            }),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "die Anlage wird angelegt: {anlage}");
    let anlage_id = anlage["id"].as_str().expect("id").to_owned();

    let (status, anlage) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/ziele"),
            json!({ "tiername": "Keiler" }),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "das Ziel wird angelegt: {anlage}");
    let ziel_id = anlage["ziele"][0]["id"].as_str().expect("id").to_owned();

    let weit = Uuid::new_v4();
    let (status, anlage) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/parkour"),
            json!({
                "name": "Waldrunde",
                "pflöcke": [{ "id": weit, "farbe": "#c0392b" }],
                "platzierungen": [{
                    "zielId": ziel_id,
                    "abstände": [{ "pflockId": weit, "meter": 40 }],
                }],
            }),
            &admin,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "der Parkour wird angelegt: {anlage}"
    );
    let parkour_id = anlage["parkour"][0]["id"].as_str().expect("id").to_owned();

    (admin, anlage_id, ziel_id, parkour_id)
}

/// Start a Runde for `bearer` and return its id.
async fn runde_starten(harness: &E2eHarness, bearer: &str, parkour_id: &str) -> String {
    let (status, body) = harness
        .client
        .post_json_auth("/api/runden", json!({ "parkourId": parkour_id }), bearer)
        .await;
    assert_eq!(status, StatusCode::OK, "AC-7: die Runde startet: {body}");
    body["id"].as_str().expect("id im Body").to_owned()
}

/// Log one arrow. Returns the whole `(status, body)` so a case can assert on either.
async fn schuss(
    harness: &E2eHarness,
    bearer: &str,
    runde_id: &str,
    ziel_id: &str,
    pfeil: usize,
    zone: &str,
) -> (StatusCode, Value) {
    harness
        .client
        .post_json_auth(
            &format!("/api/runden/{runde_id}/schüsse"),
            json!({ "zielId": ziel_id, "pfeil": pfeil, "wertungszone": zone }),
            bearer,
        )
        .await
}

// ------------------------------------------------------------------------- AC-7

#[test_log::test(pollster::test)]
async fn ac7_eine_runde_startet_nur_angemeldet() {
    let harness = E2eHarness::new().await;
    let (_, _, _, parkour_id) = anlage_mit_parkour(&harness).await;

    let (status, _) = harness
        .client
        .post_json("/api/runden", json!({ "parkourId": parkour_id }))
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED, "AC-7: 401 ohne Token");
}

#[test_log::test(pollster::test)]
async fn ac7_ein_unbekannter_parkour_antwortet_404() {
    let harness = E2eHarness::new().await;
    let (benutzer, _) = neuer_benutzer(&harness, "anna@muster.at").await;

    let (status, _) = harness
        .client
        .post_json_auth(
            "/api/runden",
            json!({ "parkourId": Uuid::new_v4() }),
            &benutzer,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "AC-7: 404 für einen fremden Parkour"
    );
}

#[test_log::test(pollster::test)]
async fn ac7_ein_archivierter_parkour_startet_keine_runde() {
    let harness = E2eHarness::new().await;
    let (admin, anlage_id, _, parkour_id) = anlage_mit_parkour(&harness).await;
    let (benutzer, _) = neuer_benutzer(&harness, "anna@muster.at").await;

    // Reference it first: an unreferenced Parkour would be deleted outright (AC-12), and a
    // deleted one is 404, not the 409 an archived one gives.
    let _ = runde_starten(&harness, &admin, &parkour_id).await;

    let (status, _) = harness
        .client
        .delete_auth(
            &format!("/api/anlagen/{anlage_id}/parkour/{parkour_id}"),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "der Parkour wird entfernt");

    let (status, _) = harness
        .client
        .post_json_auth("/api/runden", json!({ "parkourId": parkour_id }), &benutzer)
        .await;
    assert_eq!(
        status,
        StatusCode::CONFLICT,
        "AC-7: ein archivierter Parkour startet nicht"
    );
}

// ------------------------------------------------------------------------- AC-9

#[test_log::test(pollster::test)]
async fn ac9_ein_schuss_ausserhalb_des_snapshots_antwortet_409() {
    let harness = E2eHarness::new().await;
    let (_, _, _, parkour_id) = anlage_mit_parkour(&harness).await;
    let (benutzer, _) = neuer_benutzer(&harness, "anna@muster.at").await;
    let runde_id = runde_starten(&harness, &benutzer, &parkour_id).await;

    let (status, _) = schuss(
        &harness,
        &benutzer,
        &runde_id,
        &Uuid::new_v4().to_string(),
        1,
        "Kill",
    )
    .await;
    assert_eq!(
        status,
        StatusCode::CONFLICT,
        "AC-9: ein Ziel ausserhalb des Snapshots ist 409"
    );
}

#[test_log::test(pollster::test)]
async fn ac9_pfeil_null_ist_bad_request() {
    let harness = E2eHarness::new().await;
    let (_, _, ziel_id, parkour_id) = anlage_mit_parkour(&harness).await;
    let (benutzer, _) = neuer_benutzer(&harness, "anna@muster.at").await;
    let runde_id = runde_starten(&harness, &benutzer, &parkour_id).await;

    let (status, _) = schuss(&harness, &benutzer, &runde_id, &ziel_id, 0, "Kill").await;
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "AC-9: Pfeil 0 ist ungültig"
    );
}

#[test_log::test(pollster::test)]
async fn ac9_ein_schuss_in_eine_fremde_runde_antwortet_403() {
    let harness = E2eHarness::new().await;
    let (admin, _, ziel_id, parkour_id) = anlage_mit_parkour(&harness).await;
    let (eigner, _) = neuer_benutzer(&harness, "anna@muster.at").await;
    let (fremder, _) = neuer_benutzer(&harness, "bea@muster.at").await;
    let runde_id = runde_starten(&harness, &eigner, &parkour_id).await;

    let (status, _) = schuss(&harness, &fremder, &runde_id, &ziel_id, 1, "Kill").await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "AC-9: 403 in einer fremden Runde"
    );

    // AC-13: and the read is refused too.
    let (status, _) = harness
        .client
        .get_json_auth(&format!("/api/runden/{runde_id}"), &fremder)
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "AC-13: 403 beim Lesen");

    // ... while a Systemadministrator may read it (AC-9's owner-or-admin rule).
    let (status, _) = harness
        .client
        .get_json_auth(&format!("/api/runden/{runde_id}"), &admin)
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-9: ein Systemadministrator liest mit"
    );
}

#[test_log::test(pollster::test)]
async fn ac9_eine_runde_zeigt_beide_wertungen_und_den_snapshot() {
    let harness = E2eHarness::new().await;
    let (_, _, ziel_id, parkour_id) = anlage_mit_parkour(&harness).await;
    let (benutzer, _) = neuer_benutzer(&harness, "anna@muster.at").await;
    let runde_id = runde_starten(&harness, &benutzer, &parkour_id).await;

    for (pfeil, zone) in [(1, "Kill"), (2, "Kill"), (3, "Spot")] {
        let (status, body) = schuss(&harness, &benutzer, &runde_id, &ziel_id, pfeil, zone).await;
        assert_eq!(status, StatusCode::OK, "AC-9: schuss {pfeil}: {body}");
    }

    let (status, body) = harness
        .client
        .get_json_auth(&format!("/api/runden/{runde_id}"), &benutzer)
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-9: die Runde wird gelesen: {body}"
    );

    // The snapshot, not current master data.
    assert_eq!(body["parkourName"], "Waldrunde");
    assert_eq!(body["pflöcke"][0]["farbe"], "#c0392b");

    // Both Wertungen folded at the one Ziel and over the whole Runde.
    let ziel = &body["ziele"][0];
    assert_eq!(ziel["wertung"]["ersterTreffer"], 18);
    assert_eq!(ziel["wertung"]["mehrschuss"], 38);
    assert_eq!(ziel["wertung"]["ersterPfeilGetroffen"], true);
    assert_eq!(ziel["wertung"]["anzahlSchüsse"], 3);
    assert_eq!(body["gesamtErsterTreffer"], 18);
    assert_eq!(body["gesamtMehrschuss"], 38);
    assert_eq!(body["zieleMitWertung"], 1);
}

#[test_log::test(pollster::test)]
async fn ac9_eine_beendete_runde_nimmt_keinen_schuss_mehr_an() {
    let harness = E2eHarness::new().await;
    let (_, _, ziel_id, parkour_id) = anlage_mit_parkour(&harness).await;
    let (benutzer, _) = neuer_benutzer(&harness, "anna@muster.at").await;
    let runde_id = runde_starten(&harness, &benutzer, &parkour_id).await;

    let (status, body) = harness
        .client
        .post_json_auth(
            &format!("/api/runden/{runde_id}/abschluss"),
            json!({}),
            &benutzer,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-9: die Runde schließt ab: {body}"
    );
    assert_eq!(body["zustand"], "Abgeschlossen");

    let (status, _) = schuss(&harness, &benutzer, &runde_id, &ziel_id, 1, "Kill").await;
    assert_eq!(
        status,
        StatusCode::CONFLICT,
        "AC-9: eine beendete Runde nimmt nichts mehr an"
    );
}

// ------------------------------------------------------------------------- AC-9 listing

#[test_log::test(pollster::test)]
async fn ac9_die_rundenliste_zeigt_nur_eigene_runden() {
    let harness = E2eHarness::new().await;
    let (_, _, _, parkour_id) = anlage_mit_parkour(&harness).await;
    let (anna, _) = neuer_benutzer(&harness, "anna@muster.at").await;
    let (bea, _) = neuer_benutzer(&harness, "bea@muster.at").await;

    let annas = runde_starten(&harness, &anna, &parkour_id).await;
    let _ = runde_starten(&harness, &bea, &parkour_id).await;

    let (status, body) = harness.client.get_json_auth("/api/runden", &anna).await;
    assert_eq!(status, StatusCode::OK);
    let runden = body.as_array().expect("die Antwort ist eine Liste");
    assert_eq!(runden.len(), 1, "AC-13: nur die eigene Runde");
    assert_eq!(runden[0]["id"], annas);
    assert_eq!(runden[0]["parkourName"], "Waldrunde");
}

// ------------------------------------------------------------------------- 404, Liste

#[test_log::test(pollster::test)]
async fn eine_unbekannte_runde_antwortet_404() {
    let harness = E2eHarness::new().await;
    let (admin, _, _, _) = anlage_mit_parkour(&harness).await;

    let (status, _) = harness
        .client
        .get_json_auth(&format!("/api/runden/{}", Uuid::new_v4()), &admin)
        .await;
    assert_eq!(
        status,
        StatusCode::NOT_FOUND,
        "eine unbekannte Runde ist 404"
    );
}

#[test_log::test(pollster::test)]
async fn die_rundenliste_zeigt_die_eigenen_runden_neueste_zuerst() {
    let harness = E2eHarness::new().await;
    let (_, _, _, parkour_id) = anlage_mit_parkour(&harness).await;
    let (anna, _) = neuer_benutzer(&harness, "anna@muster.at").await;

    let erste = runde_starten(&harness, &anna, &parkour_id).await;
    harness.vorspulen(chrono::Duration::minutes(5));
    let zweite = runde_starten(&harness, &anna, &parkour_id).await;

    let (status, body) = harness.client.get_json_auth("/api/runden", &anna).await;
    assert_eq!(status, StatusCode::OK);
    let runden = body.as_array().expect("die Antwort ist eine Liste");
    assert_eq!(runden.len(), 2);
    assert_eq!(runden[0]["id"], zweite, "die jüngste Runde steht vorn");
    assert_eq!(runden[1]["id"], erste);
}
