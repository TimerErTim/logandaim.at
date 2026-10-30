//! SPEC-stammdaten at the HTTP edge.
//!
//! The core suite already proves the rules against both adapters. What only this layer can
//! prove is the contract a client sees: which status an unauthenticated or unprivileged
//! caller gets, that the public Parkour list needs no header, that a read of archived data
//! answers `200` while its writes answer `409` (AC-11), and that a Ziel or Parkour is
//! removed as archive-or-delete depending on whether history references it (AC-11, AC-12).

use poem::http::StatusCode;
use serde_json::{Value, json};
use uuid::Uuid;

use super::{E2eHarness, befördere_zum_systemadministrator, neuer_benutzer};

/// A body every AC-1 case can post.
fn anlage_body(name: &str) -> Value {
    json!({
        "name": name,
        "adresse": {
            "straße": "Waldweg 1",
            "postleitzahl": "3100",
            "ort": "St. Pölten",
            "ländercode": "AT",
        },
        "kontakt": { "name": "Verein Breitenstein" },
    })
}

/// Create an Anlage and return its id. `admin` must be a Systemadministrator's token.
async fn anlage_erstellen(harness: &E2eHarness, admin: &str, name: &str) -> String {
    let (status, body) = harness
        .client
        .post_json_auth("/api/anlagen", anlage_body(name), admin)
        .await;
    assert_eq!(status, StatusCode::OK, "AC-1: anlage anlegen: {body}");
    assert_eq!(body["zustand"], "Aktiv", "AC-1: sie ist aktiv");
    body["id"].as_str().expect("id im Body").to_owned()
}

/// Add one Ziel and return its id.
async fn ziel_hinzufügen(harness: &E2eHarness, bearer: &str, anlage_id: &str) -> String {
    let (status, body) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/ziele"),
            json!({ "tiername": "Keiler" }),
            bearer,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "AC-5: ziel anlegen: {body}");
    body["ziele"]
        .as_array()
        .expect("ziele im Body")
        .last()
        .expect("die Anlage hat ein Ziel")["id"]
        .as_str()
        .expect("id im Ziel")
        .to_owned()
}

/// Create a Parkour over `ziel_id` with two Pflöcke and return its id.
async fn parkour_hinzufügen(
    harness: &E2eHarness,
    bearer: &str,
    anlage_id: &str,
    ziel_id: &str,
) -> String {
    let weit = Uuid::new_v4();
    let nah = Uuid::new_v4();
    let (status, body) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/parkour"),
            json!({
                "name": "Waldrunde",
                "pflöcke": [
                    { "id": nah, "farbe": "#1b4f9c" },
                    { "id": weit, "farbe": "#c0392b" },
                ],
                "platzierungen": [{
                    "zielId": ziel_id,
                    "abstände": [
                        { "pflockId": nah, "meter": 20 },
                        { "pflockId": weit, "meter": 40 },
                    ],
                }],
            }),
            bearer,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "AC-4: parkour anlegen: {body}");
    body["parkour"]
        .as_array()
        .expect("parkour im Body")
        .last()
        .expect("die Anlage hat einen Parkour")["id"]
        .as_str()
        .expect("id im Parkour")
        .to_owned()
}

/// A promoted Systemadministrator's token and BenutzerId, plus an ordinary Benutzer's
/// token and id.
async fn admin_und_benutzer(harness: &E2eHarness) -> (String, String, String, String) {
    let (admin, admin_id) = neuer_benutzer(harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(harness, "admin@muster.at").await;
    let (status, body) = harness.client.get_json_auth("/api/benutzer", &admin).await;
    assert_eq!(status, StatusCode::OK, "Rolle laden: {body}");
    assert_eq!(body["rolle"], "Systemadministrator");
    let (benutzer, benutzer_id) = neuer_benutzer(harness, "anna@muster.at").await;
    (admin, admin_id, benutzer, benutzer_id)
}

// ------------------------------------------------------------------------- AC-3, AC-1

#[test_log::test(pollster::test)]
async fn ac3_ein_unangemeldetes_schreiben_antwortet_401() {
    let harness = E2eHarness::new().await;

    let (status, _) = harness
        .client
        .post_json("/api/anlagen", anlage_body("Anonym"))
        .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED, "AC-3: 401 ohne Token");
}

#[test_log::test(pollster::test)]
async fn ac1_nur_ein_systemadministrator_legt_eine_anlage_an() {
    let harness = E2eHarness::new().await;
    let (benutzer, _) = neuer_benutzer(&harness, "anna@muster.at").await;

    let (status, _) = harness
        .client
        .post_json_auth("/api/anlagen", anlage_body("Privat"), &benutzer)
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "AC-1: 403 ohne Rolle");

    // The promoted account sees the same body succeed.
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;
    let (status, _) = harness
        .client
        .post_json_auth("/api/anlagen", anlage_body("Breitenstein"), &admin)
        .await;
    assert_eq!(status, StatusCode::OK, "AC-1: die Beförderung genügt");
}

#[test_log::test(pollster::test)]
async fn ac3_schreiben_auf_eine_fremde_anlage_antwortet_403() {
    let harness = E2eHarness::new().await;
    let (admin, _, außenstehender, _) = admin_und_benutzer(&harness).await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;

    // The outsider holds no AnlageVerwaltung on this Anlage.
    let (status, _) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/ziele"),
            json!({ "tiername": "Keiler" }),
            &außenstehender,
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN, "AC-3: 403 ohne Verwaltung");
}

// ------------------------------------------------------------------------- AC-2

#[test_log::test(pollster::test)]
async fn ac2_verwaltung_gewaehren_und_einen_mitarbeiter_zulassen() {
    let harness = E2eHarness::new().await;
    let (admin, _, benutzer, benutzer_id) = admin_und_benutzer(&harness).await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;

    let (status, body) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/verwaltungen"),
            json!({ "benutzerId": benutzer_id, "rolle": "Mitarbeiter" }),
            &admin,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-2: die Verwaltung wird vergeben: {body}"
    );
    assert_eq!(body["verwaltungen"][0]["rolle"], "Mitarbeiter");

    // A Mitarbeiter maintains Ziele but sees no admin list entry for a foreign Anlage.
    let (status, _) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/ziele"),
            json!({ "tiername": "Keiler" }),
            &benutzer,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-2: ein Mitarbeiter pflegt Masterdaten"
    );
}

#[test_log::test(pollster::test)]
async fn ac2_systemadministrator_sucht_benutzer_exakt_nach_email() {
    let harness = E2eHarness::new().await;
    let (admin, _, benutzer, benutzer_id) = admin_und_benutzer(&harness).await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;

    let (status, treffer) = harness
        .client
        .get_json_auth(
            &format!("/api/anlagen/{anlage_id}/benutzer-suche?e_mail_adresse=anna%40muster.at"),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "AC-2: exakte Suche: {treffer}");
    assert_eq!(treffer[0]["id"], benutzer_id);
    assert_eq!(treffer[0]["eMailAdresse"], "anna@muster.at");

    let (status, nicht_gefunden) = harness
        .client
        .get_json_auth(
            &format!("/api/anlagen/{anlage_id}/benutzer-suche?e_mail_adresse=niemand%40muster.at"),
            &admin,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-2: unbekannte Adresse: {nicht_gefunden}"
    );
    assert_eq!(nicht_gefunden, json!([]));

    let (status, _) = harness
        .client
        .get_json_auth(
            &format!("/api/anlagen/{anlage_id}/benutzer-suche?e_mail_adresse=anna%40muster.at"),
            &benutzer,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "AC-2: Benutzer darf nicht suchen"
    );
}

#[test_log::test(pollster::test)]
async fn ac2_der_letzte_administrator_kann_nicht_entzogen_werden() {
    let harness = E2eHarness::new().await;
    let (admin, admin_id, _, benutzer_id) = admin_und_benutzer(&harness).await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;

    for (ziel, rolle) in [(&benutzer_id, "Mitarbeiter"), (&admin_id, "Administrator")] {
        let (status, body) = harness
            .client
            .post_json_auth(
                &format!("/api/anlagen/{anlage_id}/verwaltungen"),
                json!({ "benutzerId": ziel, "rolle": rolle }),
                &admin,
            )
            .await;
        assert_eq!(
            status,
            StatusCode::OK,
            "AC-2: {rolle} wird vergeben: {body}"
        );
    }

    // The only Administrator cannot go; the Mitarbeiter can.
    let (status, _) = harness
        .client
        .delete_auth(
            &format!("/api/anlagen/{anlage_id}/verwaltungen/{admin_id}"),
            &admin,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "AC-2: der letzte Administrator bleibt"
    );

    let (status, _) = harness
        .client
        .delete_auth(
            &format!("/api/anlagen/{anlage_id}/verwaltungen/{benutzer_id}"),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "AC-2: der Mitarbeiter geht");
}

// ------------------------------------------------------------------------- AC-11

#[test_log::test(pollster::test)]
async fn ac11_eine_deaktivierte_anlage_schreibt_nicht_und_liest_nur_der_systemadministrator() {
    let harness = E2eHarness::new().await;
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;
    let ziel_id = ziel_hinzufügen(&harness, &admin, &anlage_id).await;

    // An Administrator of the Anlage, to prove the rule is about the account role.
    let (verwalter, verwalter_id) = neuer_benutzer(&harness, "verwalter@muster.at").await;
    let (status, body) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/verwaltungen"),
            json!({ "benutzerId": verwalter_id, "rolle": "Administrator" }),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "AC-2: die Verwaltung: {body}");

    let (status, body) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/deaktivierung"),
            json!({}),
            &admin,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-10: die Deaktivierung greift: {body}"
    );
    assert_eq!(body["zustand"], "Archiviert");

    // AC-11: a Systemadministrator still reads it, with the state in words.
    let (status, body) = harness
        .client
        .get_json_auth(&format!("/api/anlagen/{anlage_id}"), &admin)
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-11: der Systemadministrator liest weiter: {body}"
    );
    assert_eq!(body["zustand"], "Archiviert");
    assert_eq!(
        body["ziele"][0]["id"], ziel_id,
        "die Historie bleibt lesbar"
    );

    // ... while the Anlage's own Administrator is refused: archived is closed to staff.
    let (status, _) = harness
        .client
        .get_json_auth(&format!("/api/anlagen/{anlage_id}"), &verwalter)
        .await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "AC-11: eine archivierte Anlage liest kein Anlagen-Administrator"
    );

    // ... but every write answers 409, even for the Systemadministrator.
    let (status, _) = harness
        .client
        .put_json_auth(
            &format!("/api/anlagen/{anlage_id}"),
            anlage_body("Neuer Name"),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::CONFLICT, "AC-11: schreiben ist 409");

    let (status, _) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/ziele"),
            json!({ "tiername": "Reh" }),
            &admin,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::CONFLICT,
        "AC-11: kind-schreiben ist 409"
    );

    // A deactivated Anlage disappears from the admin list.
    let (status, body) = harness.client.get_json_auth("/api/anlagen", &admin).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body.as_array().expect("liste").len(),
        0,
        "AC-11: nicht gelistet"
    );
}

#[test_log::test(pollster::test)]
async fn ac11_eine_archivierte_anlage_liest_auch_ein_mitarbeiter_nicht() {
    let harness = E2eHarness::new().await;
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;
    let (mitarbeiter, mitarbeiter_id) = neuer_benutzer(&harness, "mitarbeiter@muster.at").await;
    let (status, _) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/verwaltungen"),
            json!({ "benutzerId": mitarbeiter_id, "rolle": "Mitarbeiter" }),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    let (status, _) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/deaktivierung"),
            json!({}),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    let (status, _) = harness
        .client
        .get_json_auth(&format!("/api/anlagen/{anlage_id}"), &mitarbeiter)
        .await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "AC-11: eine archivierte Anlage liest kein Mitarbeiter"
    );
}

// ------------------------------------------------------------------------- AC-11, AC-12

#[test_log::test(pollster::test)]
async fn ac11_eine_archivierte_anlage_fehlt_auch_in_der_liste_ihres_mitarbeiters() {
    let harness = E2eHarness::new().await;
    let (admin, _, mitarbeiter, mitarbeiter_id) = admin_und_benutzer(&harness).await;
    let eine = anlage_erstellen(&harness, &admin, "Breitenstein").await;
    let (status, _) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{eine}/verwaltungen"),
            json!({ "benutzerId": mitarbeiter_id, "rolle": "Mitarbeiter" }),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    let (status, body) = harness
        .client
        .get_json_auth("/api/anlagen", &mitarbeiter)
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        body.as_array().expect("liste").len(),
        1,
        "vor der Deaktivierung ist die eigene Anlage gelistet"
    );

    let (status, _) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{eine}/deaktivierung"),
            json!({}),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    // The same shared rule that refuses the detail read also filters the list.
    let (status, body) = harness
        .client
        .get_json_auth("/api/anlagen", &mitarbeiter)
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    assert_eq!(
        body,
        json!([]),
        "AC-11: eine archivierte Anlage ist für ihren Mitarbeiter nicht gelistet"
    );
}

#[test_log::test(pollster::test)]
async fn ac11_und_ac12_ein_ziel_ohne_historie_wird_geloescht_mit_historie_archiviert() {
    let harness = E2eHarness::new().await;
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;

    // AC-12: nothing references this Ziel, so it is really gone.
    let unbenutzt = ziel_hinzufügen(&harness, &admin, &anlage_id).await;
    let (status, body) = harness
        .client
        .delete_auth(
            &format!("/api/anlagen/{anlage_id}/ziele/{unbenutzt}"),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "AC-12: löschen ist erlaubt: {body}");
    assert_eq!(
        body["ziele"].as_array().expect("ziele").len(),
        0,
        "AC-12: die Zeile ist fort"
    );

    // AC-11: a Ziel a captured Runde snapshot references is archived instead, and stays
    // readable. A Parkour placement alone is not history — the snapshot is (AC-12 keeps
    // that case deletable, which the core suite pins down).
    let benutzt = ziel_hinzufügen(&harness, &admin, &anlage_id).await;
    let parkour_id = parkour_hinzufügen(&harness, &admin, &anlage_id, &benutzt).await;
    let (status, _) = harness
        .client
        .post_json_auth("/api/runden", json!({ "parkourId": parkour_id }), &admin)
        .await;
    assert_eq!(status, StatusCode::OK, "eine Runde referenziert das Ziel");

    let (status, body) = harness
        .client
        .delete_auth(&format!("/api/anlagen/{anlage_id}/ziele/{benutzt}"), &admin)
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-11: archivieren statt löschen: {body}"
    );
    let ziel = &body["ziele"][0];
    assert_eq!(ziel["id"], benutzt, "AC-11: die Identität bleibt");
    assert_eq!(ziel["zustand"], "Archiviert", "AC-11: als Wort sichtbar");
}

// ------------------------------------------------------------------------- AC-13

#[test_log::test(pollster::test)]
async fn ac13_die_parkourliste_ist_oeffentlich_und_zeigt_nur_aktive_parkour() {
    let harness = E2eHarness::new().await;
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;

    // One Parkour that stays, one that is removed again (archived, so not public).
    let aktives_ziel = ziel_hinzufügen(&harness, &admin, &anlage_id).await;
    let bleibt = parkour_hinzufügen(&harness, &admin, &anlage_id, &aktives_ziel).await;
    let entfernt = parkour_hinzufügen(&harness, &admin, &anlage_id, &aktives_ziel).await;
    let (status, _) = harness
        .client
        .delete_auth(
            &format!("/api/anlagen/{anlage_id}/parkour/{entfernt}"),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "AC-11: der Parkour wird archiviert");

    // AC-13: no header at all, and no cookie.
    let (status, body) = harness.client.get_json("/api/parkour").await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-13: die Liste ist öffentlich: {body}"
    );
    let parkour = body.as_array().expect("die Antwort ist eine Liste");
    assert_eq!(parkour.len(), 1, "AC-13: nur der aktive Parkour");
    assert_eq!(parkour[0]["parkour"]["id"], bleibt);
    assert_eq!(parkour[0]["anlageName"], "Breitenstein");
    assert_eq!(
        parkour[0]["parkour"]["pflöcke"]
            .as_array()
            .expect("pflöcke")
            .iter()
            .map(|pflock| pflock["farbe"].as_str())
            .collect::<Vec<_>>(),
        vec![Some("#1b4f9c"), Some("#c0392b")],
        "die Pflockfarben sind Nutzerdaten und stehen nah zu fern"
    );
}

// ------------------------------------------------------------------------- AC-6, AC-7

#[test_log::test(pollster::test)]
async fn ac6_und_ac7_ein_parkour_setup_ersetzt_die_ganze_aufstellung() {
    let harness = E2eHarness::new().await;
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;
    let ziel_id = ziel_hinzufügen(&harness, &admin, &anlage_id).await;
    let parkour_id = parkour_hinzufügen(&harness, &admin, &anlage_id, &ziel_id).await;

    let nah = Uuid::new_v4();
    let (status, body) = harness
        .client
        .put_json_auth(
            &format!("/api/anlagen/{anlage_id}/parkour/{parkour_id}/setup"),
            json!({
                "pflöcke": [{ "id": nah, "farbe": "#e8b81c" }],
                "platzierungen": [{
                    "zielId": ziel_id,
                    "abstände": [{ "pflockId": nah, "meter": 15 }],
                }],
            }),
            &admin,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-6: das Setup wird ersetzt: {body}"
    );

    let parkour = &body["parkour"][0];
    assert_eq!(
        parkour["pflöcke"].as_array().expect("pflöcke").len(),
        1,
        "AC-6: der alte Pflock ist fort"
    );
    assert_eq!(parkour["pflöcke"][0]["farbe"], "#e8b81c");
    assert_eq!(
        parkour["pflöcke"][0].get("bezeichnung"),
        None,
        "ein Pflock trägt keine Bezeichnung mehr"
    );
}

// ------------------------------------------------------------------------- AC-5

#[test_log::test(pollster::test)]
async fn ac5_ein_ersatz_ziel_erbt_den_platz_nicht_die_historie() {
    let harness = E2eHarness::new().await;
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;
    let alt = ziel_hinzufügen(&harness, &admin, &anlage_id).await;
    parkour_hinzufügen(&harness, &admin, &anlage_id, &alt).await;

    let (status, body) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/ziele/{alt}/ersatz"),
            json!({ "tiername": "Neuer Keiler" }),
            &admin,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "AC-5: der Ersatz wird angelegt: {body}"
    );

    let neu = body["ziele"]
        .as_array()
        .expect("ziele")
        .iter()
        .find(|ziel| ziel["zustand"] == "Aktiv")
        .and_then(|ziel| ziel["id"].as_str())
        .expect("ein aktives Ziel");
    assert_ne!(neu, alt, "AC-5: eine neue Identität");
    assert_eq!(
        body["parkour"][0]["platzierungen"][0]["zielId"], neu,
        "AC-5: der Platz wandert zum Ersatz"
    );
}

// ------------------------------------------------------------------------- Verzeichnis (2.2)

#[test_log::test(pollster::test)]
async fn das_anlagenverzeichnis_ist_oeffentlich_und_ohne_kontaktdaten() {
    let harness = E2eHarness::new().await;
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;

    let (status, body) = harness
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
                "kontakt": {
                    "name": "Verein Breitenstein",
                    "email": "verein@muster.at",
                },
            }),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "die Anlage entsteht: {body}");
    let anlage_id = body["id"].as_str().expect("id").to_owned();
    let ziel_id = ziel_hinzufügen(&harness, &admin, &anlage_id).await;
    let bleibt = parkour_hinzufügen(&harness, &admin, &anlage_id, &ziel_id).await;
    let entfernt = parkour_hinzufügen(&harness, &admin, &anlage_id, &ziel_id).await;
    let (status, _) = harness
        .client
        .delete_auth(
            &format!("/api/anlagen/{anlage_id}/parkour/{entfernt}"),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "der zweite Parkour wird archiviert");

    // No header at all: the directory is public.
    let (status, body) = harness.client.get_json("/api/anlagen/oeffentlich").await;
    assert_eq!(
        status,
        StatusCode::OK,
        "das Verzeichnis ist öffentlich: {body}"
    );
    let anlagen = body.as_array().expect("die Antwort ist eine Liste");
    assert_eq!(anlagen.len(), 1, "nur die aktive Anlage");
    let anlage = &anlagen[0];
    assert_eq!(anlage["anlageId"], anlage_id);
    assert_eq!(anlage["name"], "Breitenstein");
    assert_eq!(anlage["ort"], "St. Pölten");
    assert_eq!(anlage["ländercode"], "AT");

    let parkour = anlage["parkour"].as_array().expect("parkour");
    assert_eq!(parkour.len(), 1, "nur der aktive Parkour");
    assert_eq!(parkour[0]["id"], bleibt);
    assert_eq!(parkour[0]["name"], "Waldrunde");
    assert_eq!(parkour[0]["anzahlPflöcke"], 2);
    assert_eq!(parkour[0]["anzahlZiele"], 1);

    // No contact data in the public payload.
    let serialisiert = anlage.to_string();
    assert!(
        !serialisiert.contains("verein@muster.at") && !serialisiert.contains("kontakt"),
        "das Verzeichnis darf keine Kontaktdaten tragen: {serialisiert}"
    );
}

#[test_log::test(pollster::test)]
async fn das_anlagenverzeichnis_antwortet_mit_kaputtem_token_wie_anonym() {
    let harness = E2eHarness::new().await;
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;

    let (ohne, erwartet) = harness.client.get_json("/api/anlagen/oeffentlich").await;
    assert_eq!(ohne, StatusCode::OK, "{erwartet}");

    // A stale token must not hide a public route from its own reader.
    let (mit, body) = harness
        .client
        .get_json_auth("/api/anlagen/oeffentlich", "kein-gültiges-token")
        .await;
    assert_eq!(
        mit,
        StatusCode::OK,
        "ein kaputtes Token zählt als anonym: {body}"
    );
    assert_eq!(body, erwartet, "derselbe Inhalt wie ohne Header");
    assert_eq!(body.as_array().expect("liste")[0]["anlageId"], anlage_id);
}

#[test_log::test(pollster::test)]
async fn das_anlagenverzeichnis_listet_keine_archivierte_anlage() {
    let harness = E2eHarness::new().await;
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;
    let (status, _) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/deaktivierung"),
            json!({}),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK, "die Anlage wird deaktiviert");

    let (status, body) = harness.client.get_json("/api/anlagen/oeffentlich").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body,
        json!([]),
        "eine archivierte Anlage ist nicht öffentlich"
    );
}

// ------------------------------------------------------------------------- Lesen (2.1)

#[test_log::test(pollster::test)]
async fn die_anlagenliste_zeigt_je_nach_rolle_die_eigenen_anlagen() {
    let harness = E2eHarness::new().await;
    let (admin, _, mitarbeiter, mitarbeiter_id) = admin_und_benutzer(&harness).await;
    let eine = anlage_erstellen(&harness, &admin, "Breitenstein").await;
    let _zweite = anlage_erstellen(&harness, &admin, "Pregarten").await;
    let (status, _) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{eine}/verwaltungen"),
            json!({ "benutzerId": mitarbeiter_id, "rolle": "Mitarbeiter" }),
            &admin,
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    let (status, body) = harness
        .client
        .get_json_auth("/api/anlagen", &mitarbeiter)
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let zeilen = body.as_array().expect("liste");
    assert_eq!(zeilen.len(), 1, "nur die eigene Anlage");
    assert_eq!(zeilen[0]["anlage"]["id"], eine);
    assert_eq!(zeilen[0]["rolle"], "Mitarbeiter");
    assert_eq!(zeilen[0]["istSystemadministrator"], false);

    let (status, body) = harness.client.get_json_auth("/api/anlagen", &admin).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        body.as_array().expect("liste").len(),
        2,
        "der Systemadministrator sieht alle"
    );
}

#[test_log::test(pollster::test)]
async fn die_anlage_zeigt_wo_ihre_ziele_verwendet_werden() {
    let harness = E2eHarness::new().await;
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;
    let ziel_id = ziel_hinzufügen(&harness, &admin, &anlage_id).await;
    parkour_hinzufügen(&harness, &admin, &anlage_id, &ziel_id).await;

    let (status, body) = harness
        .client
        .get_json_auth(&format!("/api/anlagen/{anlage_id}"), &admin)
        .await;
    assert_eq!(status, StatusCode::OK, "{body}");
    let verwendungen = body["ziele"][0]["verwendungen"]
        .as_array()
        .expect("verwendungen");
    assert_eq!(verwendungen.len(), 1);
    assert_eq!(body["parkour"][0]["name"], "Waldrunde");
    assert_eq!(
        verwendungen[0]["abstände"]
            .as_array()
            .expect("abstände")
            .iter()
            .map(|abstand| abstand["meter"].as_u64())
            .collect::<Vec<_>>(),
        vec![Some(20), Some(40)],
        "die Abstände stehen in Nah-zu-Fern-Reihenfolge"
    );
}

#[test_log::test(pollster::test)]
async fn ein_pflock_verlangt_eine_hex_farbe() {
    let harness = E2eHarness::new().await;
    let (admin, _) = neuer_benutzer(&harness, "admin@muster.at").await;
    befördere_zum_systemadministrator(&harness, "admin@muster.at").await;
    let anlage_id = anlage_erstellen(&harness, &admin, "Breitenstein").await;

    let (status, _) = harness
        .client
        .post_json_auth(
            &format!("/api/anlagen/{anlage_id}/parkour"),
            json!({
                "name": "Waldrunde",
                "pflöcke": [{ "id": Uuid::new_v4(), "farbe": "Rot" }],
            }),
            &admin,
        )
        .await;
    assert_eq!(
        status,
        StatusCode::BAD_REQUEST,
        "eine Wort-Farbe ist keine gültige Hex-Farbe"
    );
}
