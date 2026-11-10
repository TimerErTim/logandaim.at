use std::future::Future;
use std::pin::Pin;
use std::sync::LazyLock;

use async_trait::async_trait;
use molting::{AppliableMigration, MigrationRegistry, MigrationTarget, UpMigration};

use crate::SQLiteConnection;

mod v0001_schuss;
mod v0002_konto;
mod v0003_stammdaten;
mod v0004_erfassung;
mod v0005_ziel_pfloecke;
mod v0006_freie_runde;

type Registry = MigrationRegistry<dyn UpMigration<libsql::Transaction, libsql::Error>>;

/// Baseline plus one migration per schema change, registered in ascending version order.
pub static MIGRATIONS: LazyLock<Registry> = LazyLock::new(|| {
    let mut registry: Registry = MigrationRegistry::new();
    registry.add(v0001_schuss::Migration);
    registry.add(v0002_konto::Migration);
    registry.add(v0003_stammdaten::Migration);
    registry.add(v0004_erfassung::Migration);
    registry.add(v0005_ziel_pfloecke::Migration);
    registry.add(v0006_freie_runde::Migration);
    registry
});

#[async_trait]
impl MigrationTarget<libsql::Transaction, libsql::Error> for SQLiteConnection {
    fn get_current_version(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Option<usize>, libsql::Error>> + '_>> {
        Box::pin(async move {
            let tx = self
                .transaction_with_behavior(libsql::TransactionBehavior::Exclusive)
                .await?;
            // Create migrations table if it doesn't exist
            tx.execute(
                "CREATE TABLE IF NOT EXISTS _migration_history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    version INTEGER,
                    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )",
                (),
            )
            .await?;

            // Get the latest version based on the most recent application time
            let mut rows = tx
                .query(
                    "SELECT version FROM _migration_history ORDER BY applied_at DESC, version DESC LIMIT 1",
                    (),
                )
                .await?;

            tx.commit().await?;
            let Some(row) = rows.next().await? else {
                return Ok(None);
            };
            let version: Option<u64> = row.get(0)?;
            Ok(version.map(|v| v as usize))
        })
    }

    async fn apply_migration(
        &mut self,
        new_version: Option<usize>,
        implementation: impl AppliableMigration<libsql::Transaction, libsql::Error> + Send,
    ) -> Result<(), libsql::Error> {
        let mut tx = self
            .transaction_with_behavior(libsql::TransactionBehavior::Exclusive)
            .await?;

        implementation.run(&mut tx).await?;

        tx.execute(
            "INSERT INTO _migration_history (version) VALUES (?1)",
            [new_version.map(|v| v as i64)],
        )
        .await?;
        tx.query(
            &format!(
                "PRAGMA user_version = {}",
                new_version.map(|v| v as i64).unwrap_or(0)
            ),
            (),
        )
        .await?;

        tx.commit().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SQLiteInstance;

    #[test_log::test(pollster::test)]
    async fn v0006_rewrites_legacy_round_snapshots_before_repository_reads() {
        let instance = SQLiteInstance::in_memory().await.unwrap();
        let connection = instance.create_connection().await.unwrap();
        let anlage_id = "00000000-0000-0000-0000-000000000001";
        let parkour_id = "00000000-0000-0000-0000-000000000002";
        let legacy_snapshot = r#"{
            "parkour_id":"00000000-0000-0000-0000-000000000002",
            "parkour_name":"Wald",
            "pflöcke":[],
            "ziele":[]
        }"#;

        connection
            .execute_batch(
                r#"
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
        "#,
            )
            .await
            .unwrap();
        let runde_id = "00000000-0000-0000-0000-000000000003";
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
                   VALUES (?1, 'anlage', ?2), (?1, 'parkour', ?3)"#,
                (runde_id, anlage_id, parkour_id),
            )
            .await
            .unwrap();
        let mut migration_tx = connection.transaction().await.unwrap();
        v0006_freie_runde::Migration
            .up(&mut migration_tx)
            .await
            .unwrap();
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
        assert_eq!(version, 2);
        let mut foreign_keys = connection
            .query("PRAGMA foreign_key_check", ())
            .await
            .unwrap();
        assert!(foreign_keys.next().await.unwrap().is_none());
    }
}
