use async_trait::async_trait;
use molting::UpMigration;

/// A `Runde` may now be a `Freie Runde` with no `Anlage` and no `Parkour` (SPEC-runde rule 17).
///
/// - `anlage_id` and `parkour_id` become nullable read projections only. The stored setup
///   JSON (`snapshot`) is the truth: both ids are written from it and read back from it.
/// - The table is recreated rather than altered, because SQLite cannot drop a `NOT NULL`
///   constraint in place; the pre-release policy allows the copy-drop-rename.
/// - `runden_referenzen` names `runden`, so it is recreated against the new table. Its rows are
///   first copied into a plain backup table, because dropping the old `runden` would otherwise
///   cascade-delete the history index through the child's own `ON DELETE CASCADE`.
/// - Existing Parkour snapshots are rewritten into the current DTO shape by Rust serde before
///   installing the table. This migration covers every Parkour snapshot format produced in this
///   WIP feature; new reads therefore need no compatibility branches.
pub struct Migration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        6
    }

    fn description(&self) -> Option<&'static str> {
        Some("Freie Runde: nullable Anlage and Parkour projections")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        rewrite_legacy_snapshots(transaction).await?;

        transaction
            .execute_batch(
                r#"
            CREATE TABLE "runden_neu" (
                id TEXT PRIMARY KEY,
                -- Read projections of the setup JSON below. NULL means "not a Parkour round".
                anlage_id TEXT,
                parkour_id TEXT,
                snapshot TEXT NOT NULL,
                benutzer_id TEXT,
                anonymisiert_am TEXT,
                zustand TEXT NOT NULL,
                begonnen_am TEXT NOT NULL,
                abgeschlossen_am TEXT,
                _version INTEGER NOT NULL DEFAULT 0
            );

            INSERT INTO "runden_neu" (id, anlage_id, parkour_id, snapshot, benutzer_id,
                                      anonymisiert_am, zustand, begonnen_am, abgeschlossen_am,
                                      _version)
                SELECT r.id, r.anlage_id, r.parkour_id,
                       r.snapshot, r.benutzer_id, r.anonymisiert_am,
                       r.zustand, r.begonnen_am, r.abgeschlossen_am, r._version
                  FROM "runden" AS r;

            -- The history index names `runden` in its foreign key, so it is recreated against
            -- the rebuilt table. Its rows are the AC-11/AC-12 answer, not derived data the
            -- migration may recompute, so they are carried over verbatim.
            --
            -- They must be parked in a table without a foreign key first. `ALTER TABLE ... RENAME
            -- TO` leaves the renamed child's foreign key pointing at `runden`, and `PRAGMA
            -- foreign_keys` is ON for every connection, so `DROP TABLE "runden"` would
            -- cascade-delete every row of the renamed child. An empty child then makes the copy
            -- below silently write nothing, and `PRAGMA foreign_key_check` cannot notice, because
            -- it has no rows left to check.
            CREATE TABLE "referenzen_sicherung" AS
                SELECT runde_id, art, ref_id FROM "runden_referenzen";

            ALTER TABLE "runden_referenzen" RENAME TO "runden_referenzen_alt";

            -- The child goes first: dropping the parent while a cascade-bearing child still
            -- points at it is exactly the delete this migration must not perform.
            DROP TABLE "runden_referenzen_alt";
            DROP TABLE "runden";
            ALTER TABLE "runden_neu" RENAME TO "runden";

            CREATE INDEX IF NOT EXISTS "idx_runden_benutzer_id" ON "runden" (benutzer_id);
            CREATE INDEX IF NOT EXISTS "idx_runden_anlage_id" ON "runden" (anlage_id);
            CREATE INDEX IF NOT EXISTS "idx_runden_parkour_id" ON "runden" (parkour_id);
            CREATE UNIQUE INDEX IF NOT EXISTS "idx_runden_offen_pro_benutzer"
                ON "runden" (benutzer_id)
                WHERE zustand = 'InBearbeitung' AND benutzer_id IS NOT NULL;

            CREATE TABLE "runden_referenzen" (
                runde_id TEXT NOT NULL REFERENCES "runden" (id) ON DELETE CASCADE,
                art TEXT NOT NULL,
                ref_id TEXT NOT NULL,
                PRIMARY KEY (runde_id, art, ref_id)
            );

            INSERT INTO "runden_referenzen" (runde_id, art, ref_id)
                SELECT runde_id, art, ref_id FROM "referenzen_sicherung";

            DROP TABLE "referenzen_sicherung";

            CREATE INDEX IF NOT EXISTS "idx_runden_referenzen_ref_id"
                ON "runden_referenzen" (ref_id);
        "#,
            )
            .await?;
        Ok(())
    }
}

fn corrupt() -> libsql::Error {
    libsql::Error::SqliteFailure(
        libsql::ffi::SQLITE_CORRUPT,
        "corrupt legacy Runde snapshot".into(),
    )
}

fn pflock_dtos(
    source: &[serde_json::Value],
    distances: &serde_json::Map<String, serde_json::Value>,
) -> Result<Vec<serde_json::Value>, libsql::Error> {
    source
        .iter()
        .map(|pflock| {
            let farbe = pflock
                .get("farbe")
                .and_then(serde_json::Value::as_str)
                .ok_or_else(corrupt)?;
            let distance = pflock
                .get("abstand")
                .cloned()
                .or_else(|| distances.get(farbe).cloned());
            Ok(serde_json::json!({ "farbe": farbe, "abstand": distance }))
        })
        .collect()
}

fn target_ziele(
    source: &[serde_json::Value],
    parkour_pflöcke: &[serde_json::Value],
    has_anlage: bool,
) -> Result<Vec<serde_json::Value>, libsql::Error> {
    source
        .iter()
        .map(|ziel| {
            let id = ziel.get("ziel_id").cloned().ok_or_else(corrupt)?;
            let name = ziel
                .get("tiername")
                .or_else(|| ziel.get("name"))
                .cloned()
                .ok_or_else(corrupt)?;
            let number = ziel
                .get("zielnummer")
                .or_else(|| ziel.get("ordinal"))
                .cloned()
                .ok_or_else(corrupt)?;
            let own_distances = ziel
                .get("abstände")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|pair| Some((pair.get(0)?.as_str()?.to_owned(), pair.get(1)?.clone())))
                .collect::<serde_json::Map<_, _>>();
            let stored = ziel
                .get("pflöcke")
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            let source_pflöcke = if let Some(array) = stored.as_array() {
                array.clone()
            } else if stored == "InheritFromParkour" || stored.is_null() {
                parkour_pflöcke.to_vec()
            } else if let Some(explicit) =
                stored.get("Explicit").and_then(serde_json::Value::as_array)
            {
                explicit.clone()
            } else {
                return Err(corrupt());
            };
            let mut result = serde_json::json!({
                "ziel_id": id,
                "tiername": name,
                "zielnummer": number,
                "pflöcke": pflock_dtos(&source_pflöcke, &own_distances)?,
            });
            if let Some(anlage_ziel_id) = ziel.get("anlage_ziel_id") {
                result["anlage_ziel_id"] = anlage_ziel_id.clone();
            } else if !has_anlage {
                result["anlage_ziel_id"] = serde_json::Value::Null;
            }
            Ok(result)
        })
        .collect()
}

fn rewrite_document(
    value: serde_json::Value,
    anlage_column: Option<&str>,
    parkour_column: Option<&str>,
) -> Result<serde_json::Value, libsql::Error> {
    let object = value.as_object().ok_or_else(corrupt)?;
    let anlage = object
        .get("anlage_id")
        .and_then(serde_json::Value::as_str)
        .or(anlage_column);
    let parkour = object
        .get("parkour_id")
        .and_then(serde_json::Value::as_str)
        .or(parkour_column);
    let goals = object
        .get("ziele")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(corrupt)?;
    let parkour_pflöcke = object
        .get("pflöcke")
        .and_then(serde_json::Value::as_array)
        .cloned()
        .unwrap_or_default();

    if let Some(parkour_id) = parkour {
        let anlage_id = anlage.ok_or_else(corrupt)?;
        if let Some(column) = parkour_column {
            if column != parkour_id {
                return Err(corrupt());
            }
        }
        if let Some(column) = anlage_column {
            if column != anlage_id {
                return Err(corrupt());
            }
        }
        let name = object
            .get("name")
            .or_else(|| object.get("parkour_name"))
            .cloned()
            .ok_or_else(corrupt)?;
        Ok(serde_json::json!({
            "anlage_id": anlage_id,
            "parkour_id": parkour_id,
            "name": name,
            "ziele": target_ziele(goals, &parkour_pflöcke, true)?,
        }))
    } else {
        if anlage.is_some() || parkour_column.is_some() || anlage_column.is_some() {
            return Err(libsql::Error::SqliteFailure(
                libsql::ffi::SQLITE_CORRUPT,
                "free Runde has Parkour references".into(),
            ));
        }
        let name = object.get("name").cloned().ok_or_else(corrupt)?;
        Ok(serde_json::json!({
            "name": name,
            "ziele": target_ziele(goals, &[], false)?,
        }))
    }
}

pub(super) async fn rewrite_legacy_snapshots(
    transaction: &mut libsql::Transaction,
) -> Result<(), libsql::Error> {
    let mut rows = transaction
        .query(
            r#"SELECT id, anlage_id, parkour_id, snapshot FROM "runden""#,
            (),
        )
        .await?;

    let mut updates = Vec::new();
    while let Some(row) = rows.next().await? {
        let id: String = row.get(0)?;
        let anlage_id: Option<String> = row.get(1)?;
        let parkour_id: Option<String> = row.get(2)?;
        let snapshot: String = row.get(3)?;
        let value: serde_json::Value = serde_json::from_str(&snapshot).map_err(|_| {
            libsql::Error::SqliteFailure(
                libsql::ffi::SQLITE_CORRUPT,
                "corrupt legacy Runde snapshot".into(),
            )
        })?;
        let rewritten =
            rewrite_document(value, anlage_id.as_deref(), parkour_id.as_deref())?.to_string();
        updates.push((id, rewritten));
    }
    drop(rows);

    for (id, snapshot) in updates {
        transaction
            .execute(
                r#"UPDATE "runden" SET snapshot = ?1 WHERE id = ?2"#,
                (snapshot, id),
            )
            .await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_lock::Mutex;
    use logandaim_core::{
        domain::{AnlageId, ParkourId},
        ports::RundeRepository,
    };
    use uuid::Uuid;

    use super::*;
    use crate::{SQLiteInstance, repos::SQLiteRundeRepository};

    /// The pre-v0006 schema: `runden` with both ids `NOT NULL`, and the history index whose
    /// `ON DELETE CASCADE` is what the migration has to survive.
    const LEGACY_SCHEMA: &str = r#"
        CREATE TABLE "runden" (
            id TEXT PRIMARY KEY,
            anlage_id TEXT NOT NULL,
            parkour_id TEXT NOT NULL,
            snapshot TEXT NOT NULL,
            benutzer_id TEXT,
            anonymisiert_am TEXT,
            zustand TEXT NOT NULL,
            begonnen_am TEXT NOT NULL,
            abgeschlossen_am TEXT,
            _version INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE "runden_referenzen" (
            runde_id TEXT NOT NULL REFERENCES "runden" (id) ON DELETE CASCADE,
            art TEXT NOT NULL,
            ref_id TEXT NOT NULL,
            PRIMARY KEY (runde_id, art, ref_id)
        );
    "#;

    #[test_log::test(pollster::test)]
    async fn v0006_rewrites_legacy_round_snapshots_before_repository_reads() {
        let instance = SQLiteInstance::in_memory().await.unwrap();
        let connection = instance.create_connection().await.unwrap();
        let anlage_id = "00000000-0000-0000-0000-000000000001";
        let parkour_id = "00000000-0000-0000-0000-000000000002";
        let legacy_snapshot = r##"{
            "parkour_id":"00000000-0000-0000-0000-000000000002",
            "parkour_name":"Wald",
            "pflöcke":[{"farbe":"#123456","position":0}],
            "ziele":[
                {
                    "ziel_id":"00000000-0000-0000-0000-000000000004",
                    "tiername":"Ohne eigenen Pflocksatz",
                    "zielnummer":1,
                    "abstände":[["#123456",35]]
                },
                {
                    "ziel_id":"00000000-0000-0000-0000-000000000005",
                    "tiername":"Mit eigenem Pflocksatz",
                    "zielnummer":2,
                    "pflöcke":{"Explicit":[{"farbe":"#123456","position":0}]},
                    "abstände":[]
                }
            ]
        }"##;

        connection.execute_batch(LEGACY_SCHEMA).await.unwrap();
        let runde_id = "00000000-0000-0000-0000-000000000003";
        let ziel_id = "00000000-0000-0000-0000-000000000004";
        connection
            .execute(
                r#"INSERT INTO "runden" (id, anlage_id, parkour_id, snapshot, benutzer_id,
                    anonymisiert_am, zustand, begonnen_am, abgeschlossen_am, _version)
                    VALUES (?1, ?2, ?3, ?4, NULL, '2026-11-10T20:00:00Z', 'Abgeschlossen',
                    '2026-11-10T19:00:00Z', '2026-11-10T20:00:00Z', 2)"#,
                (runde_id, anlage_id, parkour_id, legacy_snapshot),
            )
            .await
            .unwrap();
        connection
            .execute(
                r#"INSERT INTO "runden_referenzen" (runde_id, art, ref_id)
                   VALUES (?1, 'anlage', ?2), (?1, 'parkour', ?3), (?1, 'ziel', ?4)"#,
                (runde_id, anlage_id, parkour_id, ziel_id),
            )
            .await
            .unwrap();
        let mut migration_tx = connection.transaction().await.unwrap();
        Migration.up(&mut migration_tx).await.unwrap();
        migration_tx.commit().await.unwrap();

        let mut rows = connection
            .query(
                r#"SELECT snapshot, _version FROM "runden" WHERE id = ?1"#,
                [runde_id],
            )
            .await
            .unwrap();
        let row = rows.next().await.unwrap().unwrap();
        let snapshot: String = row.get(0).unwrap();
        let version: i64 = row.get(1).unwrap();
        let json: serde_json::Value = serde_json::from_str(&snapshot).unwrap();
        assert_eq!(json["anlage_id"], anlage_id);
        assert_eq!(json["parkour_id"], parkour_id);
        assert_eq!(json["ziele"][0]["pflöcke"][0]["farbe"], "#123456");
        assert_eq!(json["ziele"][0]["pflöcke"][0]["abstand"], 35);
        assert_eq!(json["ziele"][1]["pflöcke"][0]["farbe"], "#123456");
        assert!(json["ziele"][1]["pflöcke"][0]["abstand"].is_null());
        assert!(json.get("parkour_name").is_none());
        assert!(json.get("pflöcke").is_none());
        assert_eq!(version, 2);
        let mut foreign_keys = connection
            .query("PRAGMA foreign_key_check", ())
            .await
            .unwrap();
        assert!(foreign_keys.next().await.unwrap().is_none());

        // The history index must survive the rebuild intact. `foreign_key_check` above proves
        // nothing here: it is trivially clean over an empty table, and an empty table is exactly
        // the failure mode — the renamed child's `ON DELETE CASCADE` fires when the old `runden`
        // is dropped, so the copy that follows silently writes zero rows.
        let mut referenzen = connection
            .query(
                r#"SELECT art, ref_id FROM "runden_referenzen" WHERE runde_id = ?1 ORDER BY art"#,
                [runde_id],
            )
            .await
            .unwrap();
        let mut gelesen = Vec::new();
        while let Some(row) = referenzen.next().await.unwrap() {
            gelesen.push((row.get::<String>(0).unwrap(), row.get::<String>(1).unwrap()));
        }
        assert_eq!(
            gelesen,
            vec![
                ("anlage".to_owned(), anlage_id.to_owned()),
                ("parkour".to_owned(), parkour_id.to_owned()),
                ("ziel".to_owned(), ziel_id.to_owned()),
            ],
            "v0006 must carry the history index over, not cascade-delete it"
        );
    }

    /// AC-11/AC-12 read the history index through the repository, so a migration that empties
    /// `runden_referenzen` makes a Parkour look unreferenced and therefore deletable. This drives
    /// the real adapter against a migrated legacy database.
    #[test_log::test(pollster::test)]
    async fn v0006_keeps_a_migrated_round_visible_as_a_parkour_reference() {
        let instance = SQLiteInstance::in_memory().await.unwrap();
        let connection = instance.create_connection().await.unwrap();

        let anlage_id = Uuid::from_u128(1);
        let parkour_id = Uuid::from_u128(2);
        let runde_id = Uuid::from_u128(3).to_string();
        let legacy_snapshot = format!(
            r##"{{
                "parkour_id":"{parkour_id}",
                "parkour_name":"Wald",
                "pflöcke":[],
                "ziele":[]
            }}"##
        );

        connection.execute_batch(LEGACY_SCHEMA).await.unwrap();
        connection
            .execute(
                r#"INSERT INTO "runden" (id, anlage_id, parkour_id, snapshot, benutzer_id,
                    anonymisiert_am, zustand, begonnen_am, abgeschlossen_am, _version)
                    VALUES (?1, ?2, ?3, ?4, NULL, '2026-11-10T20:00:00Z', 'Abgeschlossen',
                    '2026-11-10T19:00:00Z', '2026-11-10T20:00:00Z', 2)"#,
                (
                    runde_id.clone(),
                    anlage_id.to_string(),
                    parkour_id.to_string(),
                    legacy_snapshot,
                ),
            )
            .await
            .unwrap();
        connection
            .execute(
                r#"INSERT INTO "runden_referenzen" (runde_id, art, ref_id)
                   VALUES (?1, 'anlage', ?2), (?1, 'parkour', ?3)"#,
                (runde_id, anlage_id.to_string(), parkour_id.to_string()),
            )
            .await
            .unwrap();

        let mut migration_tx = connection.transaction().await.unwrap();
        Migration.up(&mut migration_tx).await.unwrap();
        migration_tx.commit().await.unwrap();
        drop(connection);

        // A fresh connection, because the repository reads through the UoW's own transaction.
        let tx = instance
            .create_connection()
            .await
            .unwrap()
            .transaction()
            .await
            .unwrap();
        let repo = SQLiteRundeRepository {
            tx: Arc::new(Mutex::new(Some(tx))),
        };

        assert!(
            repo.exists_referencing_parkour(ParkourId(parkour_id))
                .await
                .unwrap(),
            "AC-11/AC-12: a recorded Runde still references this Parkour after v0006"
        );
        assert!(
            repo.exists_referencing_anlage(AnlageId(anlage_id))
                .await
                .unwrap(),
            "AC-11/AC-12: a recorded Runde still references this Anlage after v0006"
        );
        assert!(
            !repo
                .exists_referencing_parkour(ParkourId(Uuid::from_u128(9)))
                .await
                .unwrap(),
            "an unreferenced Parkour must still read as unreferenced"
        );
    }
}
