use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use error_stack::Report;
use rustc_hash::FxHashMap;
use uuid::Uuid;

use logandaim_core::{
    domain::{NeuerSchuss, Schuss, SchussId},
    ports::{RepositoryError, RepositoryResult, SchussRepository},
};

use crate::konto::KontoDatastore;
use crate::konto::{
    FakeBenutzerRepository, FakeEmailBestätigungRepository, FakeKontoRepository,
    FakePasswortWiederherstellungRepository, FakeSessionRepository, FakeSigningKeyRepository,
};

/// In-memory backing store. One map per aggregate; feature slices add a field — or, when
/// the slice owns several aggregates, one nested store.
///
/// `Clone` snapshots the maps, which is what the fake unit of work uses to get
/// transaction semantics without a database.
pub struct FakeDatastore {
    pub schüsse: Mutex<FxHashMap<Uuid, Schuss>>,
    pub konto: KontoDatastore,
}

impl Clone for FakeDatastore {
    fn clone(&self) -> Self {
        Self {
            schüsse: Mutex::new(self.schüsse.lock().unwrap().clone()),
            konto: self.konto.clone(),
        }
    }
}

impl Default for FakeDatastore {
    fn default() -> Self {
        Self::new()
    }
}

impl FakeDatastore {
    pub fn new() -> Self {
        Self {
            schüsse: Mutex::new(FxHashMap::default()),
            konto: KontoDatastore::new(),
        }
    }

    /// Replace this datastore's contents with `other` (commit of a snapshot).
    pub fn replace_with(&self, other: &FakeDatastore) {
        *self.schüsse.lock().unwrap() = other.schüsse.lock().unwrap().clone();
        self.konto.replace_with(&other.konto);
    }
}

pub struct FakeSchussRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeSchussRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl SchussRepository for FakeSchussRepository {
    async fn create(&self, schuss: NeuerSchuss) -> RepositoryResult<Schuss> {
        let id = SchussId(Uuid::new_v4());
        let created = Schuss::neu(id, schuss);
        self.datastore
            .schüsse
            .lock()
            .unwrap()
            .insert(id.0, created.clone());
        Ok(created)
    }

    async fn find_by_id(&self, id: SchussId) -> RepositoryResult<Schuss> {
        self.datastore
            .schüsse
            .lock()
            .unwrap()
            .get(&id.0)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn find_by_ids(&self, ids: &[SchussId]) -> RepositoryResult<Vec<Schuss>> {
        let data = self.datastore.schüsse.lock().unwrap();
        Ok(ids
            .iter()
            .filter_map(|id| data.get(&id.0).cloned())
            .collect())
    }
}

/// Every `konto` repository over one shared snapshot, ready to hang on a fake UoW.
pub struct KontoRepositories {
    pub benutzer: FakeBenutzerRepository,
    pub konten: FakeKontoRepository,
    pub sessions: FakeSessionRepository,
    pub signing_keys: FakeSigningKeyRepository,
    pub bestätigungen: FakeEmailBestätigungRepository,
    pub wiederherstellungen: FakePasswortWiederherstellungRepository,
}

impl KontoRepositories {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self {
            benutzer: FakeBenutzerRepository::new(Arc::clone(&datastore)),
            konten: FakeKontoRepository::new(Arc::clone(&datastore)),
            sessions: FakeSessionRepository::new(Arc::clone(&datastore)),
            signing_keys: FakeSigningKeyRepository::new(Arc::clone(&datastore)),
            bestätigungen: FakeEmailBestätigungRepository::new(Arc::clone(&datastore)),
            wiederherstellungen: FakePasswortWiederherstellungRepository::new(datastore),
        }
    }
}
