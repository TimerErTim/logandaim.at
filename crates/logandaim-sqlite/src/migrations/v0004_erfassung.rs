use async_trait::async_trait;
use molting::UpMigration;

/// Round capture: dates, shot positions, and uniqueness for active rounds and arrows.
pub struct Migration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        4
    }

    fn description(&self) -> Option<&'static str> {
        Some("Runde capture fields and constraints")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                r#"
            CREATE UNIQUE INDEX "idx_runden_offen_pro_benutzer"
                ON "runden" (benutzer_id)
                WHERE zustand = 'InBearbeitung' AND benutzer_id IS NOT NULL;

            ALTER TABLE "schüsse" ADD COLUMN schussdistanz INTEGER;
            ALTER TABLE "schüsse" ADD COLUMN pflock_farbe TEXT;

            CREATE UNIQUE INDEX "idx_schüsse_runde_ziel_pfeil"
                ON "schüsse" (runde_id, ziel_id, pfeil);
        "#,
            )
            .await?;
        Ok(())
    }
}
