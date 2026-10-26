use async_trait::async_trait;

use crate::application::ResultReport;
use crate::application::uow::Versioned;
use crate::domain::konto::{
    Benutzer, BenutzerId, BestätigungId, EmailBestätigung, Konto, KontoId, NeueBestätigung,
    NeuePasswortWiederherstellung, NeueSession, NeuerBenutzer, NeuesKonto,
    PasswortWiederherstellung, PasswortWiederherstellungId, SecretTokenHash, Session, SessionId,
};
use crate::domain::{EmailAdresse, NeuerSchuss, Schuss, SchussId};

pub type RepositoryResult<T> = ResultReport<T, RepositoryError>;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("entity not found")]
    NotFound,
    #[error("version mismatch - entity was modified by another process {expected} != {actual:?}")]
    VersionMismatch { expected: u64, actual: Option<u64> },
    #[error("conflict occurred")]
    Conflict,
    #[error("connection failed")]
    Connection,
    #[error("operation failed")]
    OperationFailed,
    #[error("permissions error")]
    Permission,
    #[error("storage error")]
    Storage,
    #[error("data error")]
    Data,
    #[error("unknown repository error")]
    Unknown,
}

/// Persistence port for Schüsse.
///
/// Repository traits are dumb: store and load, never compute points or enforce business
/// rules. Aggregates arrive as their `Neues*` creation input from the domain.
#[async_trait]
pub trait SchussRepository: Send + Sync {
    async fn create(&self, schuss: NeuerSchuss) -> RepositoryResult<Schuss>;

    async fn find_by_id(&self, id: SchussId) -> RepositoryResult<Schuss>;

    /// Load the given Schüsse. IDs that do not exist are omitted; callers decide whether
    /// that is an error.
    async fn find_by_ids(&self, ids: &[SchussId]) -> RepositoryResult<Vec<Schuss>>;
}

/// One `_version`-carrying row per aggregate.
///
/// Every konto repository has the same four operations:
///
/// - `create(Neues*)` inserts a fresh row and returns it with its version,
/// - `find_by_id` loads one row with its version, or `NotFound`,
/// - `update(&Versioned<...>)` writes the *whole* row back and bumps `_version`; a stale
///   version is `RepositoryError::VersionMismatch`,
/// - `delete(id)` removes the row.
///
/// No repository enforces a rule: a use case loads the aggregate, calls a domain method,
/// and writes the result back with `update`. A missing mutator is a sign the rule belonged
/// to the use case, not the adapter.
#[async_trait]
pub trait BenutzerRepository: Send + Sync {
    async fn create(&self, benutzer: NeuerBenutzer) -> RepositoryResult<Versioned<Benutzer>>;

    async fn find_by_id(&self, id: BenutzerId) -> RepositoryResult<Versioned<Benutzer>>;

    async fn update(&self, benutzer: &Versioned<Benutzer>)
    -> RepositoryResult<Versioned<Benutzer>>;

    async fn delete(&self, id: BenutzerId) -> RepositoryResult<()>;
}

#[async_trait]
pub trait KontoRepository: Send + Sync {
    async fn create(&self, konto: NeuesKonto) -> RepositoryResult<Versioned<Konto>>;

    async fn find_by_id(&self, id: KontoId) -> RepositoryResult<Versioned<Konto>>;

    /// `Ok(None)` when no Konto holds the address: the login path needs that distinction,
    /// not an error.
    async fn find_by_email(
        &self,
        email: &EmailAdresse,
    ) -> RepositoryResult<Option<Versioned<Konto>>>;

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Konto>>>;

    async fn update(&self, konto: &Versioned<Konto>) -> RepositoryResult<Versioned<Konto>>;

    async fn delete(&self, id: KontoId) -> RepositoryResult<()>;
}

/// Rows are **retained** when spent, revoked, or expired (see
/// [`SessionIn`](crate::domain::konto::SessionIn)): replay detection needs to find the row
/// a spent refresh token names.
#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: NeueSession) -> RepositoryResult<Versioned<Session>>;

    async fn find_by_id(&self, id: SessionId) -> RepositoryResult<Versioned<Session>>;

    /// `Ok(None)` for an unknown or already rotated refresh token.
    async fn find_by_refresh_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Option<Versioned<Session>>>;

    async fn update(&self, session: &Versioned<Session>) -> RepositoryResult<Versioned<Session>>;

    /// Every session of one Benutzer, for the load-then-update revoke loop a use case runs
    /// (logout everywhere, password recovery). Retention means this must not delete.
    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Session>>>;

    async fn delete(&self, id: SessionId) -> RepositoryResult<()>;

    /// Remove every session of a Benutzer. Only `BenutzerLöschen` uses this: the identity
    /// itself is going away, so there is nothing left to replay.
    async fn delete_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<()>;
}

#[async_trait]
pub trait EmailBestätigungRepository: Send + Sync {
    async fn create(
        &self,
        bestätigung: NeueBestätigung,
    ) -> RepositoryResult<Versioned<EmailBestätigung>>;

    /// Keyed by the token hash, because that is what step 2 presents.
    async fn find_by_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Option<Versioned<EmailBestätigung>>>;

    async fn find_by_id(
        &self, id: BestätigungId
    ) -> RepositoryResult<Versioned<EmailBestätigung>>;

    /// Every outstanding row for one address, so a resend can spend them one by one.
    async fn find_by_email(
        &self,
        email: &EmailAdresse,
    ) -> RepositoryResult<Vec<Versioned<EmailBestätigung>>>;

    async fn update(
        &self,
        bestätigung: &Versioned<EmailBestätigung>,
    ) -> RepositoryResult<Versioned<EmailBestätigung>>;

    async fn delete(&self, id: BestätigungId) -> RepositoryResult<()>;

    async fn delete_by_email(&self, email: &EmailAdresse) -> RepositoryResult<()>;
}

#[async_trait]
pub trait PasswortWiederherstellungRepository: Send + Sync {
    async fn create(
        &self,
        wiederherstellung: NeuePasswortWiederherstellung,
    ) -> RepositoryResult<Versioned<PasswortWiederherstellung>>;

    async fn find_by_id(
        &self,
        id: PasswortWiederherstellungId,
    ) -> RepositoryResult<Versioned<PasswortWiederherstellung>>;

    async fn find_by_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Option<Versioned<PasswortWiederherstellung>>>;

    /// Every outstanding row for one Konto, so a new request can spend them one by one
    /// (rule 7).
    async fn find_by_konto(
        &self,
        konto_id: KontoId,
    ) -> RepositoryResult<Vec<Versioned<PasswortWiederherstellung>>>;

    async fn update(
        &self,
        wiederherstellung: &Versioned<PasswortWiederherstellung>,
    ) -> RepositoryResult<Versioned<PasswortWiederherstellung>>;

    async fn delete(&self, id: PasswortWiederherstellungId) -> RepositoryResult<()>;

    async fn delete_by_konto(&self, konto_id: KontoId) -> RepositoryResult<()>;
}
