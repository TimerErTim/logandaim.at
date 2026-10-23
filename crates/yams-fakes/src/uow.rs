use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use yams_core::{
    ports::RepositoryResult,
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

pub struct FakeUnitOfWorkProvider {
    pub log: Arc<Mutex<Vec<UoWEvent>>>,
}

impl FakeUnitOfWorkProvider {
    pub fn new() -> Self {
        Self {
            log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn empty() -> Self {
        Self::new()
    }
}

impl Default for FakeUnitOfWorkProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UnitOfWorkProvider for FakeUnitOfWorkProvider {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>> {
        self.log.lock().unwrap().push(UoWEvent::Begin);
        Ok(Box::new(FakeUnitOfWork::new(Arc::clone(&self.log))))
    }
}

pub struct FakeUnitOfWork {
    log: Arc<Mutex<Vec<UoWEvent>>>,
}

impl FakeUnitOfWork {
    pub fn new(log: Arc<Mutex<Vec<UoWEvent>>>) -> Self {
        Self { log }
    }
}

#[async_trait]
impl UnitOfWorkImpl for FakeUnitOfWork {
    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        self.log.lock().unwrap().push(UoWEvent::Commit);
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        self.log.lock().unwrap().push(UoWEvent::Rollback);
        Ok(())
    }
}

#[derive(Debug)]
pub enum UoWEvent {
    Begin,
    Commit,
    Rollback,
    Error(String),
}
