use async_trait::async_trait;
use tracing::instrument;

use crate::application::uow::UnitOfWorkImpl;
use crate::ports::RepositoryResult;

pub struct InstrumentedUnitOfWork {
    uow: Box<dyn UnitOfWorkImpl>,
}

impl InstrumentedUnitOfWork {
    pub fn wrap(inner: Box<dyn UnitOfWorkImpl>) -> Box<dyn UnitOfWorkImpl> {
        Box::new(Self { uow: inner })
    }
}

#[async_trait]
impl UnitOfWorkImpl for InstrumentedUnitOfWork {
    #[instrument(skip(self), level = "debug", err(Debug))]
    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        self.uow.commit().await
    }

    #[instrument(skip(self), level = "debug", err(Debug))]
    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        self.uow.rollback().await
    }
}
