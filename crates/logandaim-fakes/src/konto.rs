use std::sync::MutexGuard;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use error_stack::Report;
use logandaim_core::{
    ResultReport,
    application::uow::Versioned,
    domain::{
        EmailAdresse,
        konto::{
            Benutzer, BenutzerId, BestätigungId, EmailBestätigung, Konto, KontoId, NeueBestätigung,
            NeuePasswortWiederherstellung, NeueSession, NeuerBenutzer, NeuesKonto,
            PasswortWiederherstellung, PasswortWiederherstellungId, SecretTokenHash, Session,
            SessionId, SessionIn,
        },
    },
    ports::{
        BenutzerRepository, EmailBestätigungRepository, KontoRepository, Mail, MailError,
        MailSender, PasswortWiederherstellungRepository, RepositoryError, RepositoryResult,
        SessionRepository,
    },
};
use rustc_hash::FxHashMap;
use uuid::Uuid;

use crate::repository::FakeDatastore;

/// In-memory maps of the `konto` subdomain, one per aggregate, nested inside
/// [`FakeDatastore`](crate::repository::FakeDatastore).
///
/// Every map holds a [`Versioned`] row, so the fake enforces the same optimistic
/// concurrency contract as SQLite: an `update` with a stale version is a
/// `VersionMismatch`, never a silent overwrite.
///
/// There is no fake hasher and no fake signer. Tests use the real domain crypto, so a
/// test can never pass against an algorithm the product does not ship.
pub struct KontoDatastore {
    pub benutzer: Mutex<FxHashMap<BenutzerId, Versioned<Benutzer>>>,
    pub konten: Mutex<FxHashMap<KontoId, Versioned<Konto>>>,
    pub sessions: Mutex<FxHashMap<SessionId, Versioned<Session>>>,
    pub bestätigungen: Mutex<FxHashMap<BestätigungId, Versioned<EmailBestätigung>>>,
    pub wiederherstellungen:
        Mutex<FxHashMap<PasswortWiederherstellungId, Versioned<PasswortWiederherstellung>>>,
}

impl KontoDatastore {
    pub fn new() -> Self {
        Self {
            benutzer: Mutex::new(FxHashMap::default()),
            konten: Mutex::new(FxHashMap::default()),
            sessions: Mutex::new(FxHashMap::default()),
            bestätigungen: Mutex::new(FxHashMap::default()),
            wiederherstellungen: Mutex::new(FxHashMap::default()),
        }
    }

    pub fn benutzer(&self) -> MutexGuard<'_, FxHashMap<BenutzerId, Versioned<Benutzer>>> {
        self.benutzer.lock().unwrap()
    }

    pub fn konten(&self) -> MutexGuard<'_, FxHashMap<KontoId, Versioned<Konto>>> {
        self.konten.lock().unwrap()
    }

    pub fn sessions(&self) -> MutexGuard<'_, FxHashMap<SessionId, Versioned<Session>>> {
        self.sessions.lock().unwrap()
    }

    pub fn bestätigungen(
        &self,
    ) -> MutexGuard<'_, FxHashMap<BestätigungId, Versioned<EmailBestätigung>>> {
        self.bestätigungen.lock().unwrap()
    }

    pub fn wiederherstellungen(
        &self,
    ) -> MutexGuard<'_, FxHashMap<PasswortWiederherstellungId, Versioned<PasswortWiederherstellung>>>
    {
        self.wiederherstellungen.lock().unwrap()
    }

    /// Replace this store's contents with `other` (commit of a snapshot).
    pub fn replace_with(&self, other: &KontoDatastore) {
        *self.benutzer() = other.benutzer().clone();
        *self.konten() = other.konten().clone();
        *self.sessions() = other.sessions().clone();
        *self.bestätigungen() = other.bestätigungen().clone();
        *self.wiederherstellungen() = other.wiederherstellungen().clone();
    }
}

impl Clone for KontoDatastore {
    fn clone(&self) -> Self {
        Self {
            benutzer: Mutex::new(self.benutzer().clone()),
            konten: Mutex::new(self.konten().clone()),
            sessions: Mutex::new(self.sessions().clone()),
            bestätigungen: Mutex::new(self.bestätigungen().clone()),
            wiederherstellungen: Mutex::new(self.wiederherstellungen().clone()),
        }
    }
}

impl Default for KontoDatastore {
    fn default() -> Self {
        Self::new()
    }
}

/// Write `neu` into `map` only when it was loaded at `neu.v()`, mirroring the conditional
/// `UPDATE ... WHERE _version = ?`. The caller's copy is bumped, as a real adapter does.
fn store_update<K, T>(
    map: &mut FxHashMap<K, Versioned<T>>,
    id: K,
    neu: &Versioned<T>,
) -> RepositoryResult<Versioned<T>>
where
    K: std::hash::Hash + Eq + Copy,
    T: Clone,
{
    let vorhanden = map
        .get(&id)
        .ok_or_else(|| Report::new(RepositoryError::NotFound))?;
    if vorhanden.v() != neu.v() {
        return Err(Report::new(RepositoryError::VersionMismatch {
            expected: neu.v(),
            actual: Some(vorhanden.v()),
        }));
    }
    let geschrieben = neu.clone().incremented();
    map.insert(id, geschrieben.clone());
    Ok(geschrieben)
}

pub struct FakeBenutzerRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeBenutzerRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl BenutzerRepository for FakeBenutzerRepository {
    async fn create(&self, benutzer: NeuerBenutzer) -> RepositoryResult<Versioned<Benutzer>> {
        let id = BenutzerId(Uuid::new_v4());
        let erstellt = Versioned::init(Benutzer::neu(id, benutzer));
        self.datastore.konto.benutzer().insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_id(&self, id: BenutzerId) -> RepositoryResult<Versioned<Benutzer>> {
        self.datastore
            .konto
            .benutzer()
            .get(&id)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn update(
        &self,
        benutzer: &Versioned<Benutzer>,
    ) -> RepositoryResult<Versioned<Benutzer>> {
        let mut map = self.datastore.konto.benutzer();
        store_update(&mut *map, benutzer.id(), benutzer)
    }

    async fn delete(&self, id: BenutzerId) -> RepositoryResult<()> {
        if self.datastore.konto.benutzer().remove(&id).is_none() {
            return Err(Report::new(RepositoryError::NotFound));
        }
        Ok(())
    }
}

pub struct FakeKontoRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeKontoRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl KontoRepository for FakeKontoRepository {
    async fn create(&self, konto: NeuesKonto) -> RepositoryResult<Versioned<Konto>> {
        let mut konten = self.datastore.konto.konten();
        if konten
            .values()
            .any(|vorhanden| vorhanden.email() == konto.email())
        {
            return Err(Report::new(RepositoryError::Conflict));
        }
        let id = KontoId(Uuid::new_v4());
        let erstellt = Versioned::init(Konto::neu(id, konto));
        konten.insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_id(&self, id: KontoId) -> RepositoryResult<Versioned<Konto>> {
        self.datastore
            .konto
            .konten()
            .get(&id)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn find_by_email(
        &self,
        email: &EmailAdresse,
    ) -> RepositoryResult<Option<Versioned<Konto>>> {
        Ok(self
            .datastore
            .konto
            .konten()
            .values()
            .find(|konto| konto.email() == email)
            .cloned())
    }

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Konto>>> {
        Ok(self
            .datastore
            .konto
            .konten()
            .values()
            .filter(|konto| konto.benutzer_id() == benutzer_id)
            .cloned()
            .collect())
    }

    async fn update(&self, konto: &Versioned<Konto>) -> RepositoryResult<Versioned<Konto>> {
        let mut map = self.datastore.konto.konten();
        store_update(&mut *map, konto.id(), konto)
    }

    async fn delete(&self, id: KontoId) -> RepositoryResult<()> {
        if self.datastore.konto.konten().remove(&id).is_none() {
            return Err(Report::new(RepositoryError::NotFound));
        }
        Ok(())
    }
}

pub struct FakeSessionRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeSessionRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl SessionRepository for FakeSessionRepository {
    async fn create(&self, session: NeueSession) -> RepositoryResult<Versioned<Session>> {
        let id = SessionId(Uuid::new_v4());
        let erstellt = Versioned::init(Session::Ok(SessionIn::aus(id, session)));
        self.datastore.konto.sessions().insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_id(&self, id: SessionId) -> RepositoryResult<Versioned<Session>> {
        self.datastore
            .konto
            .sessions()
            .get(&id)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn find_by_refresh_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Option<Versioned<Session>>> {
        Ok(self
            .datastore
            .konto
            .sessions()
            .values()
            .find(|session| session.refresh_token_hash() == hash)
            .cloned())
    }

    async fn update(&self, session: &Versioned<Session>) -> RepositoryResult<Versioned<Session>> {
        let mut map = self.datastore.konto.sessions();
        store_update(&mut *map, session.id(), session)
    }

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Session>>> {
        Ok(self
            .datastore
            .konto
            .sessions()
            .values()
            .filter(|session| session.benutzer_id() == benutzer_id)
            .cloned()
            .collect())
    }

    async fn delete(&self, id: SessionId) -> RepositoryResult<()> {
        if self.datastore.konto.sessions().remove(&id).is_none() {
            return Err(Report::new(RepositoryError::NotFound));
        }
        Ok(())
    }

    async fn delete_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<()> {
        self.datastore
            .konto
            .sessions()
            .retain(|_, session| session.benutzer_id() != benutzer_id);
        Ok(())
    }
}

pub struct FakeEmailBestätigungRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeEmailBestätigungRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl EmailBestätigungRepository for FakeEmailBestätigungRepository {
    async fn create(
        &self,
        bestätigung: NeueBestätigung,
    ) -> RepositoryResult<Versioned<EmailBestätigung>> {
        let id = BestätigungId(Uuid::new_v4());
        let erstellt = Versioned::init(EmailBestätigung::neu(id, bestätigung));
        self.datastore
            .konto
            .bestätigungen()
            .insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Option<Versioned<EmailBestätigung>>> {
        Ok(self
            .datastore
            .konto
            .bestätigungen()
            .values()
            .find(|bestätigung| bestätigung.token_hash() == hash)
            .cloned())
    }

    async fn find_by_id(
        &self,
        id: BestätigungId,
    ) -> RepositoryResult<Versioned<EmailBestätigung>> {
        self.datastore
            .konto
            .bestätigungen()
            .get(&id)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn find_by_email(
        &self,
        email: &EmailAdresse,
    ) -> RepositoryResult<Vec<Versioned<EmailBestätigung>>> {
        Ok(self
            .datastore
            .konto
            .bestätigungen()
            .values()
            .filter(|bestätigung| bestätigung.email() == email)
            .cloned()
            .collect())
    }

    async fn update(
        &self,
        bestätigung: &Versioned<EmailBestätigung>,
    ) -> RepositoryResult<Versioned<EmailBestätigung>> {
        // Spending is a transition guarded by the version, which is what keeps two
        // concurrent completions from both winning.
        let mut map = self.datastore.konto.bestätigungen();
        store_update(&mut *map, bestätigung.id(), bestätigung)
    }

    async fn delete(&self, id: BestätigungId) -> RepositoryResult<()> {
        if self.datastore.konto.bestätigungen().remove(&id).is_none() {
            return Err(Report::new(RepositoryError::NotFound));
        }
        Ok(())
    }

    async fn delete_by_email(&self, email: &EmailAdresse) -> RepositoryResult<()> {
        self.datastore
            .konto
            .bestätigungen()
            .retain(|_, bestätigung| bestätigung.email() != email);
        Ok(())
    }
}

pub struct FakePasswortWiederherstellungRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakePasswortWiederherstellungRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl PasswortWiederherstellungRepository for FakePasswortWiederherstellungRepository {
    async fn create(
        &self,
        wiederherstellung: NeuePasswortWiederherstellung,
    ) -> RepositoryResult<Versioned<PasswortWiederherstellung>> {
        let id = PasswortWiederherstellungId(Uuid::new_v4());
        let erstellt = Versioned::init(PasswortWiederherstellung::neu(id, wiederherstellung));
        self.datastore
            .konto
            .wiederherstellungen()
            .insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_id(
        &self,
        id: PasswortWiederherstellungId,
    ) -> RepositoryResult<Versioned<PasswortWiederherstellung>> {
        self.datastore
            .konto
            .wiederherstellungen()
            .get(&id)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn find_by_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Option<Versioned<PasswortWiederherstellung>>> {
        Ok(self
            .datastore
            .konto
            .wiederherstellungen()
            .values()
            .find(|wiederherstellung| wiederherstellung.token_hash() == hash)
            .cloned())
    }

    async fn find_by_konto(
        &self,
        konto_id: KontoId,
    ) -> RepositoryResult<Vec<Versioned<PasswortWiederherstellung>>> {
        Ok(self
            .datastore
            .konto
            .wiederherstellungen()
            .values()
            .filter(|wiederherstellung| wiederherstellung.konto_id() == konto_id)
            .cloned()
            .collect())
    }

    async fn update(
        &self,
        wiederherstellung: &Versioned<PasswortWiederherstellung>,
    ) -> RepositoryResult<Versioned<PasswortWiederherstellung>> {
        let mut map = self.datastore.konto.wiederherstellungen();
        store_update(&mut *map, wiederherstellung.id(), wiederherstellung)
    }

    async fn delete(&self, id: PasswortWiederherstellungId) -> RepositoryResult<()> {
        if self
            .datastore
            .konto
            .wiederherstellungen()
            .remove(&id)
            .is_none()
        {
            return Err(Report::new(RepositoryError::NotFound));
        }
        Ok(())
    }

    async fn delete_by_konto(&self, konto_id: KontoId) -> RepositoryResult<()> {
        self.datastore
            .konto
            .wiederherstellungen()
            .retain(|_, wiederherstellung| wiederherstellung.konto_id() != konto_id);
        Ok(())
    }
}

/// Re-exported, not duplicated: the in-memory throttle is the production adapter for a
/// single-process deployment (`logandaim_core::adapters::InMemoryThrottle`), and a second
/// copy here would be a second behaviour to keep in sync with AC-15 and AC-16.
pub use logandaim_core::adapters::InMemoryThrottle;

/// Records every mail it is asked to send, so a case can assert on the link it carries.
#[derive(Default)]
pub struct FakeMailSender {
    gesendet: Mutex<Vec<Mail>>,
}

impl FakeMailSender {
    pub fn new() -> Self {
        Self::default()
    }

    /// Every mail sent so far, oldest first.
    pub fn gesendete(&self) -> Vec<Mail> {
        self.gesendet.lock().unwrap().clone()
    }

    /// The single mail sent so far. Panics on none or several, which is the assertion a
    /// happy-path case actually wants.
    pub fn einzige(&self) -> Mail {
        let gesendet = self.gesendete();
        assert_eq!(gesendet.len(), 1, "erwartet genau eine Mail");
        gesendet.into_iter().next().unwrap()
    }

    /// The most recently sent mail, which is the one a case just triggered.
    pub fn letzte(&self) -> Mail {
        self.gesendet
            .lock()
            .unwrap()
            .last()
            .cloned()
            .expect("erwartet mindestens eine Mail")
    }

    pub fn leeren(&self) {
        self.gesendet.lock().unwrap().clear();
    }
}

#[async_trait]
impl MailSender for FakeMailSender {
    async fn send(&self, mail: Mail) -> ResultReport<(), MailError> {
        self.gesendet.lock().unwrap().push(mail);
        Ok(())
    }
}
