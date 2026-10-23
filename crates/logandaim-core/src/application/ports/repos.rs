use std::fmt::Debug;

pub type RepositoryResult<T> = ResultReport<T, RepositoryError>;

use async_trait::async_trait;

use crate::application::ResultReport;
use crate::domain::{NeuerSchuss, Schuss, SchussId};

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
