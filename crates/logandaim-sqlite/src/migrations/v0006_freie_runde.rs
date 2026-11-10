use async_trait::async_trait;
use molting::UpMigration;

/// A `Runde` may now be a `Freie Runde` with no `Anlage` and no `Parkour` (SPEC-runde rule 17).
///
/// - `anlage_id` and `parkour_id` become nullable read projections only. The stored setup
///   JSON (`snapshot`) is the truth: both ids are written from it and read back from it.
/// - The table is recreated rather than altered, because SQLite cannot drop a `NOT NULL`
///   constraint in place; the pre-release policy allows the copy-drop-rename.
/// - `runden_referenzen` names `runden`, so it is recreated against the new table.
/// - Existing Parkour snapshots are rewritten into the current DTO shape by Rust serde before
///   installing the table. New reads therefore need no compatibility branches.
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
            -- the rebuilt table. Its rows are carried over: they are the AC-11/AC-12 answer,
            -- not derived data the migration may recompute.
            ALTER TABLE "runden_referenzen" RENAME TO "runden_referenzen_alt";

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
                SELECT runde_id, art, ref_id FROM "runden_referenzen_alt";

            DROP TABLE "runden_referenzen_alt";

            CREATE INDEX IF NOT EXISTS "idx_runden_referenzen_ref_id"
                ON "runden_referenzen" (ref_id);
        "#,
            )
            .await?;
        Ok(())
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct LegacyParkourSnapshot {
    parkour_id: uuid::Uuid,
    parkour_name: String,
    pflöcke: Vec<LegacyPflock>,
    ziele: Vec<LegacyZiel>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct LegacyPflock {
    farbe: String,
    position: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct LegacyZiel {
    ziel_id: uuid::Uuid,
    tiername: String,
    zielnummer: u32,
    #[serde(default)]
    pflöcke: LegacyZielPflöcke,
    abstände: Vec<(String, u32)>,
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
enum LegacyZielPflöcke {
    #[default]
    InheritFromParkour,
    Explicit(Vec<LegacyPflock>),
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
        if value.get("anlage_id").is_some() {
            continue;
        }

        let (Some(anlage_id), Some(parkour_id)) = (anlage_id, parkour_id) else {
            // Before v0006 every round was Parkour-backed and both columns were NOT NULL.
            return Err(libsql::Error::SqliteFailure(
                libsql::ffi::SQLITE_CORRUPT,
                "corrupt legacy Runde".into(),
            ));
        };
        let legacy: LegacyParkourSnapshot = serde_json::from_value(value).map_err(|_| {
            libsql::Error::SqliteFailure(
                libsql::ffi::SQLITE_CORRUPT,
                "corrupt legacy Runde snapshot".into(),
            )
        })?;
        if legacy.parkour_id.to_string() != parkour_id {
            return Err(libsql::Error::SqliteFailure(
                libsql::ffi::SQLITE_CORRUPT,
                "legacy Runde Parkour id mismatch".into(),
            ));
        }

        let rewritten = serde_json::json!({
            "anlage_id": anlage_id,
            "parkour_id": legacy.parkour_id,
            "parkour_name": legacy.parkour_name,
            "pflöcke": legacy.pflöcke,
            "ziele": legacy.ziele,
        })
        .to_string();
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
