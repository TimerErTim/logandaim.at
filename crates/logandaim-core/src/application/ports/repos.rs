use async_trait::async_trait;

use crate::application::ResultReport;
use crate::application::uow::Versioned;
use crate::domain::konto::{
    Benutzer, BenutzerId, BestätigungId, EmailBestätigung, Konto, KontoId, NeueBestätigung,
    NeuePasswortWiederherstellung, NeueSession, NeuerBenutzer, NeuesKonto,
    PasswortWiederherstellung, PasswortWiederherstellungId, SecretTokenHash, Session, SessionId,
};
use crate::domain::{
    Anlage, AnlageId, EmailAdresse, NeueAnlage, NeueRunde, NeuerSchuss, ParcoursId, Runde, RundeId,
    Schuss, SchussId, ZielId,
};

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

/// How every repository looks.
///
/// Repositories are dumb: store and load, never compute points, never enforce a business
/// rule. The shape is the same for every aggregate, and each trait repeats it rather than
/// sharing one generic interface, because the extra lookup methods (`find_by_email`,
/// `find_by_refresh_hash`, …) are the point of a hand-written port.
///
/// - `create(Neues*)` — insert a fresh row, return it with its version.
/// - `find_by_id` — load one row with its version, or `NotFound`.
/// - `update(&mut Versioned<T>)` — write the *whole* row back and bump its version in
///   place. A stale version is `RepositoryError::VersionMismatch`.
/// - `delete(Versioned<T>)` — remove the row, checking that the version the caller holds is
///   still the stored one.
///
/// The load → domain mutation → write-back loop is the caller's:
///
/// ```ignore
/// let mut konto = uow.konten().find_by_id(id).await?;
/// konto.setze_password_hash(hash);
/// uow.konten().update(&mut konto).await?;
/// ```
///
/// A missing mutator is a sign the rule belongs to the use case; a purpose-built method
/// such as `revoke_all` is a sign the logic belongs to a use case loop over `find_by_*`.
/// The `Anlage` aggregate's repository. `Ziel` and `Parcours` are children, so they have
/// no repository of their own: one `update` writes the root and replaces its child rows in
/// the same transaction.
#[async_trait]
pub trait AnlageRepository: Send + Sync {
    async fn create(&self, anlage: NeueAnlage) -> RepositoryResult<Versioned<Anlage>>;

    /// Load either lifecycle state, so a caller can answer "is this archived?" without a
    /// second method.
    async fn find_by_id(&self, id: AnlageId) -> RepositoryResult<Versioned<Anlage>>;

    /// Every active Anlage, as full aggregates. Fine for two pilot sites; a read-model
    /// port is the escape hatch if the list grows.
    async fn find_aktiv(&self) -> RepositoryResult<Vec<Versioned<Anlage>>>;

    /// Every Anlage this Benutzer holds a `AnlageVerwaltung` in, active or archived.
    async fn find_by_verwaltung(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Anlage>>>;

    /// Write the whole aggregate back: the root row and every child row, guarded by the
    /// root `_version`.
    async fn update(&self, anlage: &mut Versioned<Anlage>) -> RepositoryResult<()>;

    async fn delete(&self, anlage: Versioned<Anlage>) -> RepositoryResult<()>;
}

#[async_trait]
pub trait RundeRepository: Send + Sync {
    async fn create(&self, runde: NeueRunde) -> RepositoryResult<Versioned<Runde>>;

    async fn find_by_id(&self, id: RundeId) -> RepositoryResult<Versioned<Runde>>;

    /// Every Runde this Benutzer logged, including anonymised rows are excluded because
    /// they no longer match.
    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Runde>>>;

    async fn update(&self, runde: &mut Versioned<Runde>) -> RepositoryResult<()>;

    async fn delete(&self, runde: Versioned<Runde>) -> RepositoryResult<()>;

    /// Whether any logged or in-progress Runde still refers to this Anlage (AC-11/AC-12).
    ///
    /// The history-reference lookups live here, not on `AnlageRepository`, because the
    /// references are written by `Runde` and read as an index: a master-data removal is then
    /// one index read, never a scan of the aggregate it is about to change.
    async fn exists_referencing_anlage(&self, id: AnlageId) -> RepositoryResult<bool>;

    /// Whether any Runde refers to this Parcours.
    async fn exists_referencing_parcours(&self, id: ParcoursId) -> RepositoryResult<bool>;

    /// Whether any Runde's captured route contains this Ziel identity.
    async fn exists_referencing_ziel(&self, id: ZielId) -> RepositoryResult<bool>;
}

/// A `Schuss` is a standalone aggregate root: it carries `runde_id`, `ziel_id`, and its
/// `Zuordnung`, so it can be listed per Runde, per Ziel, or per Benutzer without loading
/// the aggregates it references.
#[async_trait]
pub trait SchussRepository: Send + Sync {
    async fn create(&self, schuss: NeuerSchuss) -> RepositoryResult<Versioned<Schuss>>;

    async fn find_by_id(&self, id: SchussId) -> RepositoryResult<Versioned<Schuss>>;

    async fn find_by_runde(&self, runde_id: RundeId) -> RepositoryResult<Vec<Versioned<Schuss>>>;

    /// Per-physical-Ziel history, independent of which Runde or Parcours produced it
    /// (AC-8).
    async fn find_by_ziel(&self, ziel_id: ZielId) -> RepositoryResult<Vec<Versioned<Schuss>>>;

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Schuss>>>;

    async fn update(&self, schuss: &mut Versioned<Schuss>) -> RepositoryResult<()>;

    async fn delete(&self, schuss: Versioned<Schuss>) -> RepositoryResult<()>;
}

#[async_trait]
pub trait BenutzerRepository: Send + Sync {
    async fn create(&self, benutzer: NeuerBenutzer) -> RepositoryResult<Versioned<Benutzer>>;

    async fn find_by_id(&self, id: BenutzerId) -> RepositoryResult<Versioned<Benutzer>>;

    async fn update(&self, benutzer: &mut Versioned<Benutzer>) -> RepositoryResult<()>;

    async fn delete(&self, benutzer: Versioned<Benutzer>) -> RepositoryResult<()>;
}

#[async_trait]
pub trait KontoRepository: Send + Sync {
    async fn create(&self, konto: NeuesKonto) -> RepositoryResult<Versioned<Konto>>;

    async fn find_by_id(&self, id: KontoId) -> RepositoryResult<Versioned<Konto>>;

    /// `NotFound` when no Konto holds the address; callers match on the error.
    async fn find_by_email(&self, email: &EmailAdresse) -> RepositoryResult<Versioned<Konto>>;

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Konto>>>;

    async fn update(&self, konto: &mut Versioned<Konto>) -> RepositoryResult<()>;

    async fn delete(&self, konto: Versioned<Konto>) -> RepositoryResult<()>;
}

/// Rows are **retained** when spent, revoked, or expired (see
/// [`SessionIn`](crate::domain::konto::SessionIn)): replay detection needs to find the row
/// a spent refresh token names, and a revocation timestamp is the record of what happened.
#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: NeueSession) -> RepositoryResult<Versioned<Session>>;

    async fn find_by_id(&self, id: SessionId) -> RepositoryResult<Versioned<Session>>;

    /// `Ok(None)` for an unknown or already rotated refresh token.
    async fn find_by_refresh_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Versioned<Session>>;

    /// Every session of one Benutzer, for the load-then-update revoke loop a use case runs
    /// (logout everywhere, password recovery). Retention means this must not delete.
    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Session>>>;

    async fn update(&self, session: &mut Versioned<Session>) -> RepositoryResult<()>;

    async fn delete(&self, session: Versioned<Session>) -> RepositoryResult<()>;

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
    ) -> RepositoryResult<Versioned<EmailBestätigung>>;

    async fn find_by_id(
        &self, id: BestätigungId
    ) -> RepositoryResult<Versioned<EmailBestätigung>>;

    /// Every outstanding row for one address, so a resend can spend them one by one.
    async fn find_by_email(
        &self,
        email: &EmailAdresse,
    ) -> RepositoryResult<Vec<Versioned<EmailBestätigung>>>;

    async fn update(&self, bestätigung: &mut Versioned<EmailBestätigung>)
    -> RepositoryResult<()>;

    async fn delete(&self, bestätigung: Versioned<EmailBestätigung>) -> RepositoryResult<()>;

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
    ) -> RepositoryResult<Versioned<PasswortWiederherstellung>>;

    /// Every outstanding row for one Konto, so a new request can spend them one by one
    /// (rule 7).
    async fn find_by_konto(
        &self,
        konto_id: KontoId,
    ) -> RepositoryResult<Vec<Versioned<PasswortWiederherstellung>>>;

    async fn update(
        &self,
        wiederherstellung: &mut Versioned<PasswortWiederherstellung>,
    ) -> RepositoryResult<()>;

    async fn delete(
        &self,
        wiederherstellung: Versioned<PasswortWiederherstellung>,
    ) -> RepositoryResult<()>;

    async fn delete_by_konto(&self, konto_id: KontoId) -> RepositoryResult<()>;
}
