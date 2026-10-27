use std::sync::{Arc, Mutex, MutexGuard};

use async_trait::async_trait;
use error_stack::Report;
use rustc_hash::FxHashMap;
use uuid::Uuid;

use logandaim_core::{
    ResultReport,
    application::uow::Versioned,
    domain::{
        EmailAdresse, NeuerSchuss, Schuss, SchussId,
        konto::{
            Benutzer, BenutzerId, BestätigungId, EmailBestätigung, Konto, KontoId, NeueBestätigung,
            NeuePasswortWiederherstellung, NeueSession, NeuerBenutzer, NeuesKonto,
            PasswortWiederherstellung, PasswortWiederherstellungId, RenewableSession,
            SecretTokenHash, Session, SessionId,
        },
    },
    ports::{
        BenutzerRepository, EmailBestätigungRepository, KontoRepository, Mail, MailError,
        MailSender, PasswortWiederherstellungRepository, RepositoryError, RepositoryResult,
        SchussRepository, SessionRepository,
    },
};

/// In-memory backing store: one map per aggregate, all on the same snapshot.
///
/// `Clone` copies the maps, which is what the fake unit of work uses to get transaction
/// semantics without a database. There is no fake hasher and no fake signer: tests use the
/// real domain crypto, so a test can never pass against an algorithm the product does not
/// ship.
pub struct FakeDatastore {
    pub schüsse: Mutex<FxHashMap<SchussId, Versioned<Schuss>>>,
    pub benutzer: Mutex<FxHashMap<BenutzerId, Versioned<Benutzer>>>,
    pub konten: Mutex<FxHashMap<KontoId, Versioned<Konto>>>,
    pub sessions: Mutex<FxHashMap<SessionId, Versioned<Session>>>,
    pub bestätigungen: Mutex<FxHashMap<BestätigungId, Versioned<EmailBestätigung>>>,
    pub wiederherstellungen:
        Mutex<FxHashMap<PasswortWiederherstellungId, Versioned<PasswortWiederherstellung>>>,
}

impl Clone for FakeDatastore {
    fn clone(&self) -> Self {
        Self {
            schüsse: Mutex::new(self.schüsse().clone()),
            benutzer: Mutex::new(self.benutzer().clone()),
            konten: Mutex::new(self.konten().clone()),
            sessions: Mutex::new(self.sessions().clone()),
            bestätigungen: Mutex::new(self.bestätigungen().clone()),
            wiederherstellungen: Mutex::new(self.wiederherstellungen().clone()),
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
            benutzer: Mutex::new(FxHashMap::default()),
            konten: Mutex::new(FxHashMap::default()),
            sessions: Mutex::new(FxHashMap::default()),
            bestätigungen: Mutex::new(FxHashMap::default()),
            wiederherstellungen: Mutex::new(FxHashMap::default()),
        }
    }

    pub fn schüsse(&self) -> MutexGuard<'_, FxHashMap<SchussId, Versioned<Schuss>>> {
        self.schüsse.lock().unwrap()
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

    /// Replace this datastore's contents with `other` (commit of a snapshot).
    pub fn replace_with(&self, other: &FakeDatastore) {
        *self.schüsse() = other.schüsse().clone();
        *self.benutzer() = other.benutzer().clone();
        *self.konten() = other.konten().clone();
        *self.sessions() = other.sessions().clone();
        *self.bestätigungen() = other.bestätigungen().clone();
        *self.wiederherstellungen() = other.wiederherstellungen().clone();
    }
}

/// Write `neu` into `map` only when the stored row is still at `neu.v()`, mirroring the
/// conditional `UPDATE ... WHERE _version = ?`, then bump `neu`'s version in place.
fn store_update<K, T>(
    map: &mut FxHashMap<K, Versioned<T>>,
    id: K,
    neu: &mut Versioned<T>,
) -> RepositoryResult<()>
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
    neu.increment();
    map.insert(id, neu.clone());
    Ok(())
}

/// Remove `geladen` from `map` only when the stored row is still at its version, mirroring
/// the conditional `DELETE ... WHERE id = ? AND _version = ?`.
fn store_delete<K, T>(
    map: &mut FxHashMap<K, Versioned<T>>,
    id: K,
    geladen: &Versioned<T>,
) -> RepositoryResult<()>
where
    K: std::hash::Hash + Eq + Copy,
{
    let vorhanden = map
        .get(&id)
        .ok_or_else(|| Report::new(RepositoryError::NotFound))?;
    if vorhanden.v() != geladen.v() {
        return Err(Report::new(RepositoryError::VersionMismatch {
            expected: geladen.v(),
            actual: Some(vorhanden.v()),
        }));
    }
    map.remove(&id);
    Ok(())
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
    async fn create(&self, schuss: NeuerSchuss) -> RepositoryResult<Versioned<Schuss>> {
        let id = SchussId(Uuid::new_v4());
        let erstellt = Versioned::init(Schuss::neu(id, schuss));
        self.datastore.schüsse().insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_id(&self, id: SchussId) -> RepositoryResult<Versioned<Schuss>> {
        self.datastore
            .schüsse()
            .get(&id)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn find_by_ids(&self, ids: &[SchussId]) -> RepositoryResult<Vec<Versioned<Schuss>>> {
        let data = self.datastore.schüsse();
        Ok(ids.iter().filter_map(|id| data.get(id).cloned()).collect())
    }

    async fn update(&self, schuss: &mut Versioned<Schuss>) -> RepositoryResult<()> {
        let mut map = self.datastore.schüsse();
        store_update(&mut map, schuss.id(), schuss)
    }

    async fn delete(&self, schuss: Versioned<Schuss>) -> RepositoryResult<()> {
        let mut map = self.datastore.schüsse();
        store_delete(&mut map, schuss.id(), &schuss)
    }
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
        self.datastore.benutzer().insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_id(&self, id: BenutzerId) -> RepositoryResult<Versioned<Benutzer>> {
        self.datastore
            .benutzer()
            .get(&id)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn update(&self, benutzer: &mut Versioned<Benutzer>) -> RepositoryResult<()> {
        let mut map = self.datastore.benutzer();
        store_update(&mut map, benutzer.id(), benutzer)
    }

    async fn delete(&self, benutzer: Versioned<Benutzer>) -> RepositoryResult<()> {
        let mut map = self.datastore.benutzer();
        store_delete(&mut map, benutzer.id(), &benutzer)
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
        let mut konten = self.datastore.konten();
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
            .konten()
            .get(&id)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn find_by_email(&self, email: &EmailAdresse) -> RepositoryResult<Versioned<Konto>> {
        self.datastore
            .konten()
            .values()
            .find(|konto| konto.email() == email)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Konto>>> {
        Ok(self
            .datastore
            .konten()
            .values()
            .filter(|konto| konto.benutzer_id() == benutzer_id)
            .cloned()
            .collect())
    }

    async fn update(&self, konto: &mut Versioned<Konto>) -> RepositoryResult<()> {
        let mut map = self.datastore.konten();
        store_update(&mut map, konto.id(), konto)
    }

    async fn delete(&self, konto: Versioned<Konto>) -> RepositoryResult<()> {
        let mut map = self.datastore.konten();
        store_delete(&mut map, konto.id(), &konto)
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
        let erstellt = Versioned::init(Session::Renewable(RenewableSession::aus(id, session)));
        self.datastore.sessions().insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_id(&self, id: SessionId) -> RepositoryResult<Versioned<Session>> {
        self.datastore
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
            .sessions()
            .values()
            .find(|session| session.refresh_token_hash() == hash)
            .cloned())
    }

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Session>>> {
        Ok(self
            .datastore
            .sessions()
            .values()
            .filter(|session| session.benutzer_id() == benutzer_id)
            .cloned()
            .collect())
    }

    async fn update(&self, session: &mut Versioned<Session>) -> RepositoryResult<()> {
        let mut map = self.datastore.sessions();
        store_update(&mut map, session.id(), session)
    }

    async fn delete(&self, session: Versioned<Session>) -> RepositoryResult<()> {
        let mut map = self.datastore.sessions();
        store_delete(&mut map, session.id(), &session)
    }

    async fn delete_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<()> {
        self.datastore
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
        self.datastore.bestätigungen().insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Option<Versioned<EmailBestätigung>>> {
        Ok(self
            .datastore
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
            .bestätigungen()
            .values()
            .filter(|bestätigung| bestätigung.email() == email)
            .cloned()
            .collect())
    }

    async fn update(
        &self, bestätigung: &mut Versioned<EmailBestätigung>
    ) -> RepositoryResult<()> {
        let mut map = self.datastore.bestätigungen();
        store_update(&mut map, bestätigung.id(), bestätigung)
    }

    async fn delete(&self, bestätigung: Versioned<EmailBestätigung>) -> RepositoryResult<()> {
        let mut map = self.datastore.bestätigungen();
        store_delete(&mut map, bestätigung.id(), &bestätigung)
    }

    async fn delete_by_email(&self, email: &EmailAdresse) -> RepositoryResult<()> {
        self.datastore
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
            .wiederherstellungen()
            .insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_id(
        &self,
        id: PasswortWiederherstellungId,
    ) -> RepositoryResult<Versioned<PasswortWiederherstellung>> {
        self.datastore
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
            .wiederherstellungen()
            .values()
            .filter(|wiederherstellung| wiederherstellung.konto_id() == konto_id)
            .cloned()
            .collect())
    }

    async fn update(
        &self,
        wiederherstellung: &mut Versioned<PasswortWiederherstellung>,
    ) -> RepositoryResult<()> {
        let mut map = self.datastore.wiederherstellungen();
        store_update(&mut map, wiederherstellung.id(), wiederherstellung)
    }

    async fn delete(
        &self,
        wiederherstellung: Versioned<PasswortWiederherstellung>,
    ) -> RepositoryResult<()> {
        let mut map = self.datastore.wiederherstellungen();
        store_delete(&mut map, wiederherstellung.id(), &wiederherstellung)
    }

    async fn delete_by_konto(&self, konto_id: KontoId) -> RepositoryResult<()> {
        self.datastore
            .wiederherstellungen()
            .retain(|_, wiederherstellung| wiederherstellung.konto_id() != konto_id);
        Ok(())
    }
}

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

    /// The single mail sent so far. Panics on none or several, which is what a happy-path
    /// case actually asserts.
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
