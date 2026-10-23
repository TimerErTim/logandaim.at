use async_trait::async_trait;
use molting::UpMigration;

/// Baseline schema for the reference slice: one table for logged Schüsse.
pub struct Migration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        1
    }

    fn description(&self) -> Option<&'static str> {
        Some("Schüsse an Zielen")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                r#"
            CREATE TABLE IF NOT EXISTS "schüsse" (
                id TEXT PRIMARY KEY,
                ziel_id TEXT NOT NULL,
                pfeil INTEGER NOT NULL,
                "wertungszone" TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS "idx_schüsse_ziel_id" ON "schüsse" (ziel_id);
        "#,
            )
            .await?;
        Ok(())
    }
}
