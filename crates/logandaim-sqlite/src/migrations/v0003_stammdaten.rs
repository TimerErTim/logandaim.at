use async_trait::async_trait;
use molting::UpMigration;

/// Master data: the `Anlage` aggregate and the `Runde` that captures its snapshot.
///
/// Shapes that mirror the domain on purpose:
///
/// - `Ziel` and `Parcours` are **children** of `anlagen`: no repository loads them alone, so a
///   child write is a replace inside the root's transaction, guarded by the root `_version`.
/// - `anlagen.zustand` is the lifecycle column of the typestate root. Archiving a Ziel or a
///   Parcours is its own `zustand`, because children use a plain enum.
/// - `runden.snapshot` is the frozen `ParcoursSnapshot` as a JSON document. The domain stays
///   serde-free; the DTO lives in the adapter.
/// - `runden_referenzen` is the history index: one row per referenced master-data identity, so
///   "may this be deleted?" is an index read rather than a scan of every snapshot.
/// - The inherited `schüsse` table is **dropped and recreated**: this is pre-release, no user
///   ever ran the old shape, and it lacked `runde_id` and the `Zuordnung`.
pub struct Migration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        3
    }

    fn description(&self) -> Option<&'static str> {
        Some("Anlagen, Parcours, Ziele, and the Runde snapshot")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                r#"
            DROP TABLE IF EXISTS "schüsse";

            CREATE TABLE IF NOT EXISTS "anlagen" (
                id TEXT PRIMARY KEY,
                zustand TEXT NOT NULL,
                name TEXT NOT NULL,
                strasse TEXT NOT NULL,
                postleitzahl TEXT NOT NULL,
                ort TEXT NOT NULL,
                laendercode TEXT NOT NULL,
                kontakt_name TEXT,
                kontakt_email TEXT,
                kontakt_telefon TEXT,
                kontakt_website TEXT,
                deaktiviert_am TEXT,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS "idx_anlagen_zustand" ON "anlagen" (zustand);

            CREATE TABLE IF NOT EXISTS "anlage_verwaltungen" (
                anlage_id TEXT NOT NULL REFERENCES "anlagen" (id) ON DELETE CASCADE,
                benutzer_id TEXT NOT NULL,
                rolle TEXT NOT NULL,
                PRIMARY KEY (anlage_id, benutzer_id)
            );

            CREATE INDEX IF NOT EXISTS "idx_anlage_verwaltungen_benutzer_id"
                ON "anlage_verwaltungen" (benutzer_id);

            -- A Ziel is a physical target, not a route slot: its identity survives every
            -- arrangement edit (SPEC-stammdaten rule 5, AC-5).
            CREATE TABLE IF NOT EXISTS "ziele" (
                id TEXT PRIMARY KEY,
                anlage_id TEXT NOT NULL REFERENCES "anlagen" (id) ON DELETE CASCADE,
                tiername TEXT NOT NULL,
                zielgruppe INTEGER NOT NULL,
                zustand TEXT NOT NULL,
                ordnung INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS "idx_ziele_anlage_id" ON "ziele" (anlage_id);

            CREATE TABLE IF NOT EXISTS "parcours" (
                id TEXT PRIMARY KEY,
                anlage_id TEXT NOT NULL REFERENCES "anlagen" (id) ON DELETE CASCADE,
                name TEXT NOT NULL,
                zustand TEXT NOT NULL,
                ordnung INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS "idx_parcours_anlage_id" ON "parcours" (anlage_id);

            -- Pflöcke are per Parcours, ordered far to near by `ordnung`. A Pflock is its
            -- colour: no label exists, the colour identifies the stake and is unique here.
            CREATE TABLE IF NOT EXISTS "pflock_setups" (
                id TEXT PRIMARY KEY,
                parcours_id TEXT NOT NULL REFERENCES "parcours" (id) ON DELETE CASCADE,
                farbe TEXT NOT NULL,
                ordnung INTEGER NOT NULL,
                UNIQUE (parcours_id, farbe),
                UNIQUE (parcours_id, ordnung)
            );

            CREATE INDEX IF NOT EXISTS "idx_pflock_setups_parcours_id"
                ON "pflock_setups" (parcours_id);

            CREATE TABLE IF NOT EXISTS "zielplatzierungen" (
                parcours_id TEXT NOT NULL REFERENCES "parcours" (id) ON DELETE CASCADE,
                ziel_id TEXT NOT NULL,
                ordnung INTEGER NOT NULL,
                PRIMARY KEY (parcours_id, ziel_id),
                UNIQUE (parcours_id, ordnung)
            );

            CREATE INDEX IF NOT EXISTS "idx_zielplatzierungen_ziel_id"
                ON "zielplatzierungen" (ziel_id);

            CREATE TABLE IF NOT EXISTS "zielplatzierung_abstaende" (
                parcours_id TEXT NOT NULL,
                ziel_id TEXT NOT NULL,
                pflock_id TEXT NOT NULL,
                meter INTEGER NOT NULL,
                PRIMARY KEY (parcours_id, ziel_id, pflock_id),
                FOREIGN KEY (parcours_id, ziel_id)
                    REFERENCES "zielplatzierungen" (parcours_id, ziel_id) ON DELETE CASCADE
            );

            -- The frozen setup of one route at the instant a Runde started. `snapshot` is the
            -- adapter-local JSON document, so the domain needs no serde.
            CREATE TABLE IF NOT EXISTS "runden" (
                id TEXT PRIMARY KEY,
                anlage_id TEXT NOT NULL,
                parcours_id TEXT NOT NULL,
                snapshot TEXT NOT NULL,
                benutzer_id TEXT,
                anonymisiert_am TEXT,
                zustand TEXT NOT NULL,
                begonnen_am TEXT NOT NULL,
                abgeschlossen_am TEXT,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS "idx_runden_benutzer_id" ON "runden" (benutzer_id);
            CREATE INDEX IF NOT EXISTS "idx_runden_anlage_id" ON "runden" (anlage_id);
            CREATE INDEX IF NOT EXISTS "idx_runden_parcours_id" ON "runden" (parcours_id);

            -- The history index: one row per master-data identity a Runde still leans on, so
            -- AC-11/AC-12 ("archive vs delete") is an index read.
            CREATE TABLE IF NOT EXISTS "runden_referenzen" (
                runde_id TEXT NOT NULL REFERENCES "runden" (id) ON DELETE CASCADE,
                art TEXT NOT NULL,
                ref_id TEXT NOT NULL,
                PRIMARY KEY (runde_id, art, ref_id)
            );

            CREATE INDEX IF NOT EXISTS "idx_runden_referenzen_ref_id"
                ON "runden_referenzen" (ref_id);

            -- Recreated, not migrated: pre-release, and the shape now carries the Runde and the
            -- attribution the retention rule needs.
            CREATE TABLE IF NOT EXISTS "schüsse" (
                id TEXT PRIMARY KEY,
                runde_id TEXT NOT NULL,
                benutzer_id TEXT,
                anonymisiert_am TEXT,
                ziel_id TEXT NOT NULL,
                pfeil INTEGER NOT NULL,
                "wertungszone" TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS "idx_schüsse_runde_id" ON "schüsse" (runde_id);
            CREATE INDEX IF NOT EXISTS "idx_schüsse_ziel_id" ON "schüsse" (ziel_id);
            CREATE INDEX IF NOT EXISTS "idx_schüsse_benutzer_id" ON "schüsse" (benutzer_id);
        "#,
            )
            .await?;
        Ok(())
    }
}
