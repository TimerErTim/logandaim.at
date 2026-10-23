use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use rustc_hash::FxHashMap;
use uuid::Uuid;

use logandaim_core::{
    domain::{Schuss, SchussId, schuss::Schuss as DomainSchuss},
    ports::{NeuerSchuss, RepositoryError, RepositoryResult, SchussRepository},
};

/// In-memory backing store. One map per aggregate; feature slices add a field.
///
/// `Clone` snapshots the maps, which is what the fake unit of work uses to get
/// transaction semantics without a database.
pub struct FakeDatastore {
    pub schüsse: Mutex<FxHashMap<Uuid, Schuss>>,
}

impl Clone for FakeDatastore {
    fn clone(&self) -> Self {
        Self {
            schüsse: Mutex::new(self.schüsse.lock().unwrap().clone()),
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
        }
    }

    /// Replace this datastore's contents with `other` (commit of a snapshot).
    pub fn replace_with(&self, other: &FakeDatastore) {
        *self.schüsse.lock().unwrap() = other.schüsse.lock().unwrap().clone();
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
        let created = DomainSchuss::neu(id, schuss.ziel_id, schuss.pfeil, schuss.wertungszone)
            .map_err(|_| RepositoryError::Data)?;
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
            .ok_or(RepositoryError::NotFound.into())
    }

    async fn find_by_ids(&self, ids: &[SchussId]) -> RepositoryResult<Vec<Schuss>> {
        let data = self.datastore.schüsse.lock().unwrap();
        Ok(ids
            .iter()
            .filter_map(|id| data.get(&id.0).cloned())
            .collect())
    }
}
