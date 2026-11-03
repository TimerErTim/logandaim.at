use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use logandaim_core::{
    ports::{
        AnlageRepository, BenutzerRepository, EmailBestätigungRepository, KontoRepository,
        PasswortWiederherstellungRepository, RepositoryResult, RundeRepository, SchussRepository,
        SessionRepository,
    },
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

use crate::repository::{
    FakeAnlageRepository, FakeBenutzerRepository, FakeDatastore, FakeEmailBestätigungRepository,
    FakeKontoRepository, FakePasswortWiederherstellungRepository, FakeRundeRepository,
    FakeSchussRepository, FakeSessionRepository,
};

/// Fake UoW with real transaction semantics: `begin` snapshots the backing datastore,
/// repositories write into the snapshot, `commit` publishes the snapshot back.
pub struct FakeUnitOfWorkProvider {
    pub log: Arc<Mutex<Vec<UoWEvent>>>,
    pub datastore: Arc<FakeDatastore>,
}

impl FakeUnitOfWorkProvider {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self {
            log: Arc::new(Mutex::new(Vec::new())),
            datastore,
        }
    }

    pub fn empty() -> Self {
        Self::new(Arc::new(FakeDatastore::new()))
    }
}

impl Default for FakeUnitOfWorkProvider {
    fn default() -> Self {
        Self::empty()
    }
}

#[async_trait]
impl UnitOfWorkProvider for FakeUnitOfWorkProvider {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>> {
        self.log.lock().unwrap().push(UoWEvent::Begin);
        Ok(Box::new(FakeUnitOfWork::new(
            Arc::clone(&self.log),
            Arc::clone(&self.datastore),
        )))
    }
}

pub struct FakeUnitOfWork {
    log: Arc<Mutex<Vec<UoWEvent>>>,
    /// The data visible after commit. Writes only reach it via `commit`.
    backing_datastore: Arc<FakeDatastore>,
    /// Snapshot the repositories write into; discarded on rollback.
    transaction_datastore: Arc<FakeDatastore>,
    anlagen: FakeAnlageRepository,
    runden: FakeRundeRepository,
    schüsse: FakeSchussRepository,
    benutzer: FakeBenutzerRepository,
    konten: FakeKontoRepository,
    sessions: FakeSessionRepository,
    bestätigungen: FakeEmailBestätigungRepository,
    wiederherstellungen: FakePasswortWiederherstellungRepository,
}

impl FakeUnitOfWork {
    pub fn new(log: Arc<Mutex<Vec<UoWEvent>>>, datastore: Arc<FakeDatastore>) -> Self {
        let transaction_datastore = Arc::new(FakeDatastore::clone(datastore.as_ref()));
        Self {
            log,
            backing_datastore: datastore,
            anlagen: FakeAnlageRepository::new(Arc::clone(&transaction_datastore)),
            runden: FakeRundeRepository::new(Arc::clone(&transaction_datastore)),
            schüsse: FakeSchussRepository::new(Arc::clone(&transaction_datastore)),
            benutzer: FakeBenutzerRepository::new(Arc::clone(&transaction_datastore)),
            konten: FakeKontoRepository::new(Arc::clone(&transaction_datastore)),
            sessions: FakeSessionRepository::new(Arc::clone(&transaction_datastore)),
            bestätigungen: FakeEmailBestätigungRepository::new(
                Arc::clone(&transaction_datastore),
            ),
            wiederherstellungen: FakePasswortWiederherstellungRepository::new(
                transaction_datastore.clone(),
            ),
            transaction_datastore,
        }
    }
}

#[async_trait]
impl UnitOfWorkImpl for FakeUnitOfWork {
    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        self.log.lock().unwrap().push(UoWEvent::Commit);
        self.backing_datastore
            .replace_with(self.transaction_datastore.as_ref());
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        self.log.lock().unwrap().push(UoWEvent::Rollback);
        Ok(())
    }

    fn anlagen(&self) -> &dyn AnlageRepository {
        &self.anlagen
    }

    fn runden(&self) -> &dyn RundeRepository {
        &self.runden
    }

    fn schüsse(&self) -> &dyn SchussRepository {
        &self.schüsse
    }

    fn benutzer(&self) -> &dyn BenutzerRepository {
        &self.benutzer
    }

    fn konten(&self) -> &dyn KontoRepository {
        &self.konten
    }

    fn sessions(&self) -> &dyn SessionRepository {
        &self.sessions
    }

    fn bestätigungen(&self) -> &dyn EmailBestätigungRepository {
        &self.bestätigungen
    }

    fn wiederherstellungen(&self) -> &dyn PasswortWiederherstellungRepository {
        &self.wiederherstellungen
    }
}

#[derive(Debug)]
pub enum UoWEvent {
    Begin,
    Commit,
    Rollback,
    Error(String),
}
