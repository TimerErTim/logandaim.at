use serde_json::json;
use uuid::Uuid;

use super::{LogandaimApiTestClient, assert_status_ok, base_app_builder};

#[test_log::test(pollster::test)]
async fn schuss_erfassen_und_wertung_berechnen() {
    let client = LogandaimApiTestClient::new(base_app_builder().await);
    let ziel_id = Uuid::new_v4();

    let mut ids = Vec::new();
    for (pfeil, zone) in [(1, "Kill"), (2, "Kill"), (3, "Spot")] {
        let (status, body) = client
            .post_json(
                "/api/schuss",
                json!({ "zielId": ziel_id, "pfeil": pfeil, "wertungszone": zone }),
            )
            .await;
        assert_status_ok(status);
        assert_eq!(body["pfeil"], pfeil);
        assert_eq!(body["zielId"], ziel_id.to_string());
        ids.push(body["id"].clone());
    }

    let (status, body) = client
        .post_json("/api/wertung", json!({ "schussIds": ids }))
        .await;
    assert_status_ok(status);
    assert_eq!(body["ersterTreffer"], 18);
    assert_eq!(body["mehrschuss"], 38);
    assert_eq!(body["ersterPfeilGetroffen"], true);
    assert_eq!(body["anzahlSchüsse"], 3);
}

#[test_log::test(pollster::test)]
async fn pfeil_null_ist_bad_request() {
    let client = LogandaimApiTestClient::new(base_app_builder().await);

    let (status, _body) = client
        .post_json(
            "/api/schuss",
            json!({ "zielId": Uuid::new_v4(), "pfeil": 0, "wertungszone": "Spot" }),
        )
        .await;

    assert_eq!(status, poem::http::StatusCode::BAD_REQUEST);
}

#[test_log::test(pollster::test)]
async fn leere_schussliste_ist_bad_request() {
    let client = LogandaimApiTestClient::new(base_app_builder().await);

    let (status, _body) = client
        .post_json("/api/wertung", json!({ "schussIds": [] }))
        .await;

    assert_eq!(status, poem::http::StatusCode::BAD_REQUEST);
}
