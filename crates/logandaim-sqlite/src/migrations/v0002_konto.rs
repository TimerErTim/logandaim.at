use async_trait::async_trait;
use molting::UpMigration;

/// The `konto` subdomain: Benutzer, Konten, sessions, signing keys, and the two
/// single-use token tables.
///
/// Two shapes are deliberate and mirror the domain:
///
/// - There is **no unconfirmed Benutzer state** (SPEC-konto rule 3, AC-20): a `benutzer`
///   row only ever appears together with its confirmed `konten` row, in the same
///   transaction. `email_bestätigungen` is keyed by the address, not by a `benutzer_id`,
///   because no Benutzer exists while the confirmation is outstanding.
/// - Revocation is a **row state** (rule 15): `sessions.verhindert_am` and
///   `signing_keys.aktiv` decide acceptance, so a logout is immediate even while an
///   access token is still inside its 24 hours.
pub struct Migration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        2
    }

    fn description(&self) -> Option<&'static str> {
        Some("Benutzer, Konten, sessions, signing keys, and single-use tokens")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                r#"
            CREATE TABLE IF NOT EXISTS "benutzer" (
                id TEXT PRIMARY KEY,
                rolle TEXT NOT NULL,
                erstellt_am TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS "konten" (
                id TEXT PRIMARY KEY,
                benutzer_id TEXT NOT NULL REFERENCES "benutzer" (id) ON DELETE CASCADE,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                bestätigt_am TEXT NOT NULL,
                erstellt_am TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS "idx_konten_benutzer_id" ON "konten" (benutzer_id);

            CREATE TABLE IF NOT EXISTS "sessions" (
                id TEXT PRIMARY KEY,
                benutzer_id TEXT NOT NULL REFERENCES "benutzer" (id) ON DELETE CASCADE,
                refresh_token_hash TEXT NOT NULL UNIQUE,
                signing_key_id TEXT NOT NULL,
                erstellt_am TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                spent_at TEXT,
                revoked_at TEXT,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS "idx_sessions_benutzer_id" ON "sessions" (benutzer_id);
            CREATE INDEX IF NOT EXISTS "idx_sessions_revoked_at" ON "sessions" (revoked_at);
            CREATE INDEX IF NOT EXISTS "idx_sessions_expires_at" ON "sessions" (expires_at);

            CREATE TABLE IF NOT EXISTS "signing_keys" (
                kid TEXT PRIMARY KEY,
                öffentlicher_schlüssel TEXT NOT NULL,
                aktiv INTEGER NOT NULL,
                erstellt_am TEXT NOT NULL,
                außer_betrieb_am TEXT,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS "email_bestätigungen" (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL,
                token_hash TEXT NOT NULL UNIQUE,
                expires_at TEXT NOT NULL,
                spent_at TEXT,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS "idx_email_bestätigungen_email"
                ON "email_bestätigungen" (email);

            CREATE TABLE IF NOT EXISTS "passwort_wiederherstellungen" (
                id TEXT PRIMARY KEY,
                konto_id TEXT NOT NULL REFERENCES "konten" (id) ON DELETE CASCADE,
                token_hash TEXT NOT NULL UNIQUE,
                expires_at TEXT NOT NULL,
                spent_at TEXT,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS "idx_passwort_wiederherstellungen_konto_id"
                ON "passwort_wiederherstellungen" (konto_id);
        "#,
            )
            .await?;
        Ok(())
    }
}
