//! Persistence ports of the `konto` subdomain.
//!
//! Repositories are dumb: store and load, never enforce a business rule. Aggregates arrive
//! as their `Neu*` creation input from the domain and come back rebuilt through
//! `from_parts`, so a loaded aggregate obeys the same invariants as a fresh one.

use async_trait::async_trait;

use super::RepositoryResult;
use crate::domain::EmailAdresse;
use crate::domain::konto::{
    Benutzer, BenutzerId, BestätigungId, EmailBestätigung, Konto, KontoId, NeueBestätigung,
    NeueSession, NeueWiederherstellung, NeuerBenutzer, NeuesKonto, PasswortWiederherstellung,
    Session, SessionId, SigningKey, SigningKeyId, TokenHash, WiederherstellungId,
};

#[async_trait]
pub trait BenutzerRepository: Send + Sync {
    async fn create(&self, benutzer: NeuerBenutzer) -> RepositoryResult<Benutzer>;

    async fn find_by_id(&self, id: BenutzerId) -> RepositoryResult<Benutzer>;

    /// Insert or update the role. Used by the idempotent superuser bootstrap, which is the
    /// only path that promotes anybody (SPEC-konto rule 9).
    async fn setze_rolle(
        &self,
        id: BenutzerId,
        rolle: crate::domain::konto::Rolle,
    ) -> RepositoryResult<()>;

    async fn delete(&self, id: BenutzerId) -> RepositoryResult<()>;
}

#[async_trait]
pub trait KontoRepository: Send + Sync {
    async fn create(&self, konto: NeuesKonto) -> RepositoryResult<Konto>;

    async fn find_by_id(&self, id: KontoId) -> RepositoryResult<Konto>;

    /// `Ok(None)` when no Konto holds the address: the login path needs that distinction,
    /// not an error.
    async fn find_by_email(&self, email: &EmailAdresse) -> RepositoryResult<Option<Konto>>;

    async fn find_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<Vec<Konto>>;

    /// Persist a replaced credential.
    async fn speichere(&self, konto: &Konto) -> RepositoryResult<()>;

    async fn delete(&self, id: KontoId) -> RepositoryResult<()>;
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: NeueSession) -> RepositoryResult<Session>;

    async fn find_by_id(&self, id: SessionId) -> RepositoryResult<Session>;

    /// `Ok(None)` for an unknown or already rotated refresh token.
    async fn find_by_refresh_hash(&self, hash: &TokenHash) -> RepositoryResult<Option<Session>>;

    async fn markiere_verbraucht(
        &self,
        id: SessionId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()>;

    async fn widerrufe(
        &self,
        id: SessionId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()>;

    async fn widerrufe_alle_fuer_benutzer(
        &self,
        benutzer_id: BenutzerId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()>;

    async fn delete_alle_fuer_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<()>;
}

#[async_trait]
pub trait SigningKeyRepository: Send + Sync {
    /// `Ok(None)` for an unknown `kid`: that is a 401, not a storage failure (AC-17).
    async fn find_by_kid(&self, kid: &SigningKeyId) -> RepositoryResult<Option<SigningKey>>;

    /// Idempotent startup registration of the configured key.
    async fn ensure(&self, schlüssel: &SigningKey) -> RepositoryResult<()>;

    /// Retire a key by row update; no delete, so an old token's `kid` stays recognisable.
    async fn außer_betrieb_nehmen(
        &self,
        kid: &SigningKeyId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()>;
}

#[async_trait]
pub trait EmailBestätigungRepository: Send + Sync {
    async fn create(&self, bestätigung: NeueBestätigung) -> RepositoryResult<EmailBestätigung>;

    /// Keyed by the token hash, because that is what step 2 presents.
    async fn find_by_hash(&self, hash: &TokenHash) -> RepositoryResult<Option<EmailBestätigung>>;

    /// Spend one row. Fails with `Conflict` if another request already spent it.
    async fn consume(
        &self,
        id: BestätigungId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()>;

    /// Invalidate every open token for an address, so a resend leaves exactly one usable.
    async fn invalidate_open_for_email(
        &self,
        email: &EmailAdresse,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()>;

    async fn delete_alle_fuer_email(&self, email: &EmailAdresse) -> RepositoryResult<()>;
}

#[async_trait]
pub trait PasswortWiederherstellungRepository: Send + Sync {
    async fn create(
        &self,
        wiederherstellung: NeueWiederherstellung,
    ) -> RepositoryResult<PasswortWiederherstellung>;

    async fn find_by_hash(
        &self,
        hash: &TokenHash,
    ) -> RepositoryResult<Option<PasswortWiederherstellung>>;

    async fn consume(
        &self,
        id: WiederherstellungId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()>;

    /// Requesting a new token spends every open one for that Konto (rule 7).
    async fn invalidate_open_for_konto(
        &self,
        konto_id: KontoId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()>;

    async fn delete_alle_fuer_konto(&self, konto_id: KontoId) -> RepositoryResult<()>;
}
