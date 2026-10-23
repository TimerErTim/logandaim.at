use async_trait::async_trait;

use crate::{
    domain::{Schuss, SchussId, Wertungszone, ZielId},
    ports::RepositoryResult,
};

/// Input for creating a Schuss. The repository assigns the identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuerSchuss {
    pub ziel_id: ZielId,
    pub pfeil: usize,
    pub wertungszone: Wertungszone,
}

/// Persistence port for Schüsse.
///
/// Repository traits are `async_trait` so a [`crate::uow::UnitOfWorkImpl`] can hand out
/// `&dyn SchussRepository`. They are dumb: store and load, never compute points or
/// enforce business rules.
#[async_trait]
pub trait SchussRepository: Send + Sync {
    async fn create(&self, schuss: NeuerSchuss) -> RepositoryResult<Schuss>;

    async fn find_by_id(&self, id: SchussId) -> RepositoryResult<Schuss>;

    /// Load the given Schüsse. IDs that do not exist are omitted; callers decide whether
    /// that is an error.
    async fn find_by_ids(&self, ids: &[SchussId]) -> RepositoryResult<Vec<Schuss>>;
}
