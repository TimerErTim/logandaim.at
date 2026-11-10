use async_trait::async_trait;
use molting::UpMigration;

/// A Ziel's own Pflock set: the `Explicit` half of `ZielPflöcke`.
///
/// Zero rows for a placement mean `InheritFromParkour`, which is why `ordnung` carries the
/// near-to-far order rather than an ordinal on the placement: the set *is* the rows.
///
/// - `UNIQUE (parkour_id, ziel_id, ordnung)` keeps one Pflock per position.
/// - `UNIQUE (parkour_id, ziel_id, farbe)` keeps a colour unique **within one Ziel's set**.
///   That is deliberately narrower than `pflock_setups`' `UNIQUE (parkour_id, farbe)`: a
///   Ziel's own set may reuse a colour the Parkour-wide set already has.
/// - The foreign key names the placement, so a dropped Ziel or Parkour takes its own set
///   with it. The repository deletes the rows explicitly before dropping a `parkour`, because
///   a bulk `DELETE` with `ON DELETE CASCADE` is not something a reader should have to trust.
pub struct Migration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        5
    }

    fn description(&self) -> Option<&'static str> {
        Some("Per-Ziel Pflock sets")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                r#"
            CREATE TABLE IF NOT EXISTS "ziel_pflock_setups" (
                id TEXT PRIMARY KEY,
                parkour_id TEXT NOT NULL,
                ziel_id TEXT NOT NULL,
                farbe TEXT NOT NULL,
                ordnung INTEGER NOT NULL,
                UNIQUE (parkour_id, ziel_id, ordnung),
                UNIQUE (parkour_id, ziel_id, farbe),
                FOREIGN KEY (parkour_id, ziel_id)
                    REFERENCES "zielplatzierungen" (parkour_id, ziel_id) ON DELETE CASCADE
            );

            CREATE INDEX IF NOT EXISTS "idx_ziel_pflock_setups_ziel"
                ON "ziel_pflock_setups" (parkour_id, ziel_id);
        "#,
            )
            .await?;
        Ok(())
    }
}
