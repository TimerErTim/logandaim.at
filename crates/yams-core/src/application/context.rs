use std::sync::Arc;

use crate::{
    application::instrumented::InstrumentedUnitOfWork,
    application::uow::{UnitOfWork, UnitOfWorkImpl, UnitOfWorkProvider},
    ports::{Clock, RepositoryResult},
};
use tracing::instrument;

#[derive(Clone, Copy)]
enum ExecutionSource<'a> {
    Root(&'a dyn UnitOfWorkProvider),
    Nested(&'a dyn UnitOfWorkImpl),
}

/// Fresh per `App::execute` / `execute_fn`. Holds ports + a UoW source (root provider or nested impl).
/// Does not hold an active transaction — call [`ExecutionContext::enter`].
pub struct ExecutionContext<'a> {
    source: ExecutionSource<'a>,
    clock: Arc<dyn Clock>,
}

impl<'a> ExecutionContext<'a> {
    pub(super) fn root(provider: &'a dyn UnitOfWorkProvider, clock: Arc<dyn Clock>) -> Self {
        Self {
            source: ExecutionSource::Root(provider),
            clock,
        }
    }

    pub(super) fn nested(uow: &'a dyn UnitOfWorkImpl, clock: Arc<dyn Clock>) -> Self {
        Self {
            source: ExecutionSource::Nested(uow),
            clock,
        }
    }

    pub fn clock(&self) -> &dyn Clock {
        self.clock.as_ref()
    }

    /// Nested context that joins `uow`'s transaction and copies this context's ports.
    /// Nested `enter` returns a locked UoW; `commit` / `rollback` are no-ops.
    pub fn sub<'u>(&self, uow: &'u UnitOfWork<'_>) -> ExecutionContext<'u> {
        ExecutionContext::nested(uow.as_impl(), Arc::clone(&self.clock))
    }

    /// Start a unit of work. Returned UoW borrows `self`, so this context cannot be
    /// moved into a nested `perform` until the UoW is committed or rolled back.
    /// Nested contexts (from [`ExecutionContext::sub`]) join the outer transaction.
    #[instrument(skip(self), level = "debug", err(Debug))]
    pub async fn enter(&self) -> RepositoryResult<UnitOfWork<'_>> {
        match self.source {
            ExecutionSource::Root(provider) => {
                let implementation = InstrumentedUnitOfWork::wrap(provider.begin().await?);
                Ok(UnitOfWork::owned(implementation))
            }
            ExecutionSource::Nested(inner) => Ok(UnitOfWork::locked(inner)),
        }
    }
}
