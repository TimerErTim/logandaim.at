use async_trait::async_trait;

use crate::application::ResultReport;
use crate::application::uow::Versioned;
use crate::domain::konto::{
    Benutzer, BenutzerId, BestätigungId, EmailBestätigung, Konto, KontoId, NeueBestätigung,
    NeuePasswortWiederherstellung, NeueSession, NeuerBenutzer, NeuesKonto,
    PasswortWiederherstellung, PasswortWiederherstellungId, SecretTokenHash, Session, SessionId,
};
use crate::domain::{
    Anlage, AnlageId, EmailAdresse, NeueAnlage, NeueRunde, NeuerSchuss, ParkourId, Runde, RundeId,
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

/// Store and load one aggregate; never compute or enforce a business rule.
#[async_trait]
pub trait AnlageRepository: Send + Sync {
    async fn create(&self, anlage: NeueAnlage) -> RepositoryResult<Versioned<Anlage>>;

    /// Load either lifecycle state.
    async fn find_by_id(&self, id: AnlageId) -> RepositoryResult<Versioned<Anlage>>;

    async fn find_aktiv(&self) -> RepositoryResult<Vec<Versioned<Anlage>>>;

    async fn find_by_verwaltung(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Anlage>>>;

    /// Write the root and its child rows back, guarded by the root version.
    async fn update(&self, anlage: &mut Versioned<Anlage>) -> RepositoryResult<()>;

    async fn delete(&self, anlage: Versioned<Anlage>) -> RepositoryResult<()>;
}

#[async_trait]
pub trait RundeRepository: Send + Sync {
    async fn create(&self, runde: NeueRunde) -> RepositoryResult<Versioned<Runde>>;

    async fn find_by_id(&self, id: RundeId) -> RepositoryResult<Versioned<Runde>>;

    /// Every Runde of one Benutzer; anonymised rows no longer match.
    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Runde>>>;

    async fn update(&self, runde: &mut Versioned<Runde>) -> RepositoryResult<()>;

    async fn delete(&self, runde: Versioned<Runde>) -> RepositoryResult<()>;

    /// Whether any Runde still refers to this Anlage.
    async fn exists_referencing_anlage(&self, id: AnlageId) -> RepositoryResult<bool>;

    /// Whether any Runde refers to this Parkour.
    async fn exists_referencing_parkour(&self, id: ParkourId) -> RepositoryResult<bool>;

    /// Whether any Runde's captured route contains this Ziel identity.
    async fn exists_referencing_ziel(&self, id: ZielId) -> RepositoryResult<bool>;
}

#[async_trait]
pub trait SchussRepository: Send + Sync {
    async fn create(&self, schuss: NeuerSchuss) -> RepositoryResult<Versioned<Schuss>>;

    async fn find_by_id(&self, id: SchussId) -> RepositoryResult<Versioned<Schuss>>;

    async fn find_by_runde(&self, runde_id: RundeId) -> RepositoryResult<Vec<Versioned<Schuss>>>;

    /// Per-physical-Ziel history, independent of the Runde that produced it.
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

    /// `NotFound` when no Konto holds the address.
    async fn find_by_email(&self, email: &EmailAdresse) -> RepositoryResult<Versioned<Konto>>;

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Konto>>>;

    async fn update(&self, konto: &mut Versioned<Konto>) -> RepositoryResult<()>;

    async fn delete(&self, konto: Versioned<Konto>) -> RepositoryResult<()>;
}

/// Rows are retained when spent, revoked, or expired.
#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: NeueSession) -> RepositoryResult<Versioned<Session>>;

    async fn find_by_id(&self, id: SessionId) -> RepositoryResult<Versioned<Session>>;

    /// `NotFound` for an unknown or already rotated refresh token.
    async fn find_by_refresh_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Versioned<Session>>;

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Session>>>;

    async fn update(&self, session: &mut Versioned<Session>) -> RepositoryResult<()>;

    async fn delete(&self, session: Versioned<Session>) -> RepositoryResult<()>;

    /// Remove every session of one Benutzer, when the identity itself goes away.
    async fn delete_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<()>;
}

#[async_trait]
pub trait EmailBestätigungRepository: Send + Sync {
    async fn create(
        &self,
        bestätigung: NeueBestätigung,
    ) -> RepositoryResult<Versioned<EmailBestätigung>>;

    /// Keyed by the token hash, which is what confirmation step 2 presents.
    async fn find_by_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Versioned<EmailBestätigung>>;

    async fn find_by_id(
        &self, id: BestätigungId
    ) -> RepositoryResult<Versioned<EmailBestätigung>>;

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
