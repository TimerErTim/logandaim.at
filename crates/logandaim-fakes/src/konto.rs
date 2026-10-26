use std::sync::MutexGuard;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use error_stack::Report;
use logandaim_core::{
    ResultReport,
    domain::{
        EmailAdresse,
        konto::{
            Benutzer, BenutzerId, BestaetigungId, EmailBestaetigung, Konto, KontoId,
            NeueBestaetigung, NeueSession, NeueWiederherstellung, NeuerBenutzer, NeuesKonto,
            PasswortWiederherstellung, Rolle, Session, SessionId, SigningKey, SigningKeyId,
            TokenHash, WiederherstellungId,
        },
    },
    ports::{
        BenutzerRepository, EmailBestaetigungRepository, KontoRepository, Mail, MailFehler,
        MailSender, PasswortWiederherstellungRepository, RepositoryError, RepositoryResult,
        SessionRepository, SigningKeyRepository,
    },
};
use rustc_hash::FxHashMap;
use uuid::Uuid;

use crate::repository::FakeDatastore;

/// In-memory maps of the `konto` subdomain, one per aggregate, nested inside
/// [`FakeDatastore`](crate::repository::FakeDatastore).
///
/// There is no fake hasher and no fake signer. Tests use the real domain crypto, so a
/// test can never pass against an algorithm the product does not ship.
pub struct KontoDatastore {
    pub benutzer: Mutex<FxHashMap<BenutzerId, Benutzer>>,
    pub konten: Mutex<FxHashMap<KontoId, Konto>>,
    pub sessions: Mutex<FxHashMap<SessionId, Session>>,
    pub signing_keys: Mutex<FxHashMap<SigningKeyId, SigningKey>>,
    pub bestaetigungen: Mutex<FxHashMap<BestaetigungId, EmailBestaetigung>>,
    pub wiederherstellungen: Mutex<FxHashMap<WiederherstellungId, PasswortWiederherstellung>>,
}

impl KontoDatastore {
    pub fn new() -> Self {
        Self {
            benutzer: Mutex::new(FxHashMap::default()),
            konten: Mutex::new(FxHashMap::default()),
            sessions: Mutex::new(FxHashMap::default()),
            signing_keys: Mutex::new(FxHashMap::default()),
            bestaetigungen: Mutex::new(FxHashMap::default()),
            wiederherstellungen: Mutex::new(FxHashMap::default()),
        }
    }

    pub fn benutzer(&self) -> MutexGuard<'_, FxHashMap<BenutzerId, Benutzer>> {
        self.benutzer.lock().unwrap()
    }

    pub fn konten(&self) -> MutexGuard<'_, FxHashMap<KontoId, Konto>> {
        self.konten.lock().unwrap()
    }

    pub fn sessions(&self) -> MutexGuard<'_, FxHashMap<SessionId, Session>> {
        self.sessions.lock().unwrap()
    }

    pub fn signing_keys(&self) -> MutexGuard<'_, FxHashMap<SigningKeyId, SigningKey>> {
        self.signing_keys.lock().unwrap()
    }

    pub fn bestaetigungen(&self) -> MutexGuard<'_, FxHashMap<BestaetigungId, EmailBestaetigung>> {
        self.bestaetigungen.lock().unwrap()
    }

    pub fn wiederherstellungen(
        &self,
    ) -> MutexGuard<'_, FxHashMap<WiederherstellungId, PasswortWiederherstellung>> {
        self.wiederherstellungen.lock().unwrap()
    }

    /// Replace this store's contents with `other` (commit of a snapshot).
    pub fn replace_with(&self, other: &KontoDatastore) {
        *self.benutzer() = other.benutzer().clone();
        *self.konten() = other.konten().clone();
        *self.sessions() = other.sessions().clone();
        *self.signing_keys() = other.signing_keys().clone();
        *self.bestaetigungen() = other.bestaetigungen().clone();
        *self.wiederherstellungen() = other.wiederherstellungen().clone();
    }
}

impl Clone for KontoDatastore {
    fn clone(&self) -> Self {
        Self {
            benutzer: Mutex::new(self.benutzer().clone()),
            konten: Mutex::new(self.konten().clone()),
            sessions: Mutex::new(self.sessions().clone()),
            signing_keys: Mutex::new(self.signing_keys().clone()),
            bestaetigungen: Mutex::new(self.bestaetigungen().clone()),
            wiederherstellungen: Mutex::new(self.wiederherstellungen().clone()),
        }
    }
}

impl Default for KontoDatastore {
    fn default() -> Self {
        Self::new()
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
    async fn create(&self, benutzer: NeuerBenutzer) -> RepositoryResult<Benutzer> {
        let id = BenutzerId(Uuid::new_v4());
        let erstellt = Benutzer::neu(id, benutzer);
        self.datastore.konto.benutzer().insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_id(&self, id: BenutzerId) -> RepositoryResult<Benutzer> {
        self.datastore
            .konto
            .benutzer()
            .get(&id)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn setze_rolle(&self, id: BenutzerId, rolle: Rolle) -> RepositoryResult<()> {
        let mut benutzer = self.datastore.konto.benutzer();
        let vorhanden = benutzer
            .get(&id)
            .ok_or_else(|| Report::new(RepositoryError::NotFound))?;
        let neu = Benutzer::from_parts(id, rolle, vorhanden.erstellt_am());
        benutzer.insert(id, neu);
        Ok(())
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
    async fn create(&self, konto: NeuesKonto) -> RepositoryResult<Konto> {
        let mut konten = self.datastore.konto.konten();
        if konten
            .values()
            .any(|vorhanden| vorhanden.email() == konto.email())
        {
            return Err(Report::new(RepositoryError::Conflict));
        }
        let id = KontoId(Uuid::new_v4());
        let erstellt = Konto::neu(id, konto);
        konten.insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_id(&self, id: KontoId) -> RepositoryResult<Konto> {
        self.datastore
            .konto
            .konten()
            .get(&id)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn find_by_email(&self, email: &EmailAdresse) -> RepositoryResult<Option<Konto>> {
        Ok(self
            .datastore
            .konto
            .konten()
            .values()
            .find(|konto| konto.email() == email)
            .cloned())
    }

    async fn find_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<Vec<Konto>> {
        Ok(self
            .datastore
            .konto
            .konten()
            .values()
            .filter(|konto| konto.benutzer_id() == benutzer_id)
            .cloned()
            .collect())
    }

    async fn speichere(&self, konto: &Konto) -> RepositoryResult<()> {
        let mut konten = self.datastore.konto.konten();
        if !konten.contains_key(&konto.id()) {
            return Err(Report::new(RepositoryError::NotFound));
        }
        konten.insert(konto.id(), konto.clone());
        Ok(())
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
    async fn create(&self, session: NeueSession) -> RepositoryResult<Session> {
        let id = SessionId(Uuid::new_v4());
        let erstellt = Session::neu(id, session);
        self.datastore.konto.sessions().insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_id(&self, id: SessionId) -> RepositoryResult<Session> {
        self.datastore
            .konto
            .sessions()
            .get(&id)
            .cloned()
            .ok_or_else(|| Report::new(RepositoryError::NotFound))
    }

    async fn find_by_refresh_hash(&self, hash: &TokenHash) -> RepositoryResult<Option<Session>> {
        Ok(self
            .datastore
            .konto
            .sessions()
            .values()
            .find(|session| session.refresh_token_hash() == hash)
            .cloned())
    }

    async fn markiere_verbraucht(
        &self,
        id: SessionId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut sessions = self.datastore.konto.sessions();
        let session = sessions
            .get_mut(&id)
            .ok_or_else(|| Report::new(RepositoryError::NotFound))?;
        session.verbrauchen(jetzt);
        Ok(())
    }

    async fn widerrufe(
        &self,
        id: SessionId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut sessions = self.datastore.konto.sessions();
        let session = sessions
            .get_mut(&id)
            .ok_or_else(|| Report::new(RepositoryError::NotFound))?;
        session.widerrufen(jetzt);
        Ok(())
    }

    async fn widerrufe_alle_fuer_benutzer(
        &self,
        benutzer_id: BenutzerId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut sessions = self.datastore.konto.sessions();
        for session in sessions.values_mut() {
            if session.benutzer_id() == benutzer_id {
                session.widerrufen(jetzt);
            }
        }
        Ok(())
    }

    async fn delete_alle_fuer_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<()> {
        self.datastore
            .konto
            .sessions()
            .retain(|_, session| session.benutzer_id() != benutzer_id);
        Ok(())
    }
}

pub struct FakeSigningKeyRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeSigningKeyRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl SigningKeyRepository for FakeSigningKeyRepository {
    async fn find_by_kid(&self, kid: &SigningKeyId) -> RepositoryResult<Option<SigningKey>> {
        Ok(self.datastore.konto.signing_keys().get(kid).cloned())
    }

    async fn ensure(&self, schluessel: &SigningKey) -> RepositoryResult<()> {
        let mut keys = self.datastore.konto.signing_keys();
        match keys.get(schluessel.kid()) {
            Some(vorhanden) if vorhanden == schluessel => Ok(()),
            Some(_) => Err(Report::new(RepositoryError::Conflict)),
            None => {
                keys.insert(schluessel.kid().clone(), schluessel.clone());
                Ok(())
            }
        }
    }

    async fn ausser_betrieb_nehmen(
        &self,
        kid: &SigningKeyId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut keys = self.datastore.konto.signing_keys();
        let schluessel = keys
            .get_mut(kid)
            .ok_or_else(|| Report::new(RepositoryError::NotFound))?;
        schluessel.ausser_betrieb_nehmen(jetzt);
        Ok(())
    }
}

pub struct FakeEmailBestaetigungRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeEmailBestaetigungRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl EmailBestaetigungRepository for FakeEmailBestaetigungRepository {
    async fn create(&self, bestaetigung: NeueBestaetigung) -> RepositoryResult<EmailBestaetigung> {
        let id = BestaetigungId(Uuid::new_v4());
        let erstellt = EmailBestaetigung::neu(id, bestaetigung);
        self.datastore
            .konto
            .bestaetigungen()
            .insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_hash(&self, hash: &TokenHash) -> RepositoryResult<Option<EmailBestaetigung>> {
        Ok(self
            .datastore
            .konto
            .bestaetigungen()
            .values()
            .find(|bestaetigung| bestaetigung.token_hash() == hash)
            .cloned())
    }

    async fn consume(
        &self,
        id: BestaetigungId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut bestaetigungen = self.datastore.konto.bestaetigungen();
        let bestaetigung = bestaetigungen
            .get_mut(&id)
            .ok_or_else(|| Report::new(RepositoryError::NotFound))?;
        bestaetigung
            .verbrauchen(jetzt)
            .map_err(|_| Report::new(RepositoryError::Conflict))?;
        Ok(())
    }

    async fn invalidate_open_for_email(
        &self,
        email: &EmailAdresse,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut bestaetigungen = self.datastore.konto.bestaetigungen();
        for bestaetigung in bestaetigungen.values_mut() {
            if bestaetigung.email() == email && !bestaetigung.ist_verbraucht() {
                let _ = bestaetigung.verbrauchen(jetzt);
            }
        }
        Ok(())
    }

    async fn delete_alle_fuer_email(&self, email: &EmailAdresse) -> RepositoryResult<()> {
        self.datastore
            .konto
            .bestaetigungen()
            .retain(|_, bestaetigung| bestaetigung.email() != email);
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
        wiederherstellung: NeueWiederherstellung,
    ) -> RepositoryResult<PasswortWiederherstellung> {
        let id = WiederherstellungId(Uuid::new_v4());
        let erstellt = PasswortWiederherstellung::neu(id, wiederherstellung);
        self.datastore
            .konto
            .wiederherstellungen()
            .insert(id, erstellt.clone());
        Ok(erstellt)
    }

    async fn find_by_hash(
        &self,
        hash: &TokenHash,
    ) -> RepositoryResult<Option<PasswortWiederherstellung>> {
        Ok(self
            .datastore
            .konto
            .wiederherstellungen()
            .values()
            .find(|wiederherstellung| wiederherstellung.token_hash() == hash)
            .cloned())
    }

    async fn consume(
        &self,
        id: WiederherstellungId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut wiederherstellungen = self.datastore.konto.wiederherstellungen();
        let wiederherstellung = wiederherstellungen
            .get_mut(&id)
            .ok_or_else(|| Report::new(RepositoryError::NotFound))?;
        wiederherstellung
            .verbrauchen(jetzt)
            .map_err(|_| Report::new(RepositoryError::Conflict))?;
        Ok(())
    }

    async fn invalidate_open_for_konto(
        &self,
        konto_id: KontoId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut wiederherstellungen = self.datastore.konto.wiederherstellungen();
        for wiederherstellung in wiederherstellungen.values_mut() {
            if wiederherstellung.konto_id() == konto_id && !wiederherstellung.ist_verbraucht() {
                let _ = wiederherstellung.verbrauchen(jetzt);
            }
        }
        Ok(())
    }

    async fn delete_alle_fuer_konto(&self, konto_id: KontoId) -> RepositoryResult<()> {
        self.datastore
            .konto
            .wiederherstellungen()
            .retain(|_, wiederherstellung| wiederherstellung.konto_id() != konto_id);
        Ok(())
    }
}

/// Re-exported, not duplicated: the in-memory throttle is the production adapter for a
/// single-process deployment (`logandaim-core::adapters::InMemoryDrossel`), and a second
/// copy here would be a second behaviour to keep in sync with AC-15 and AC-16.
pub use logandaim_core::adapters::InMemoryDrossel;

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
    async fn senden(&self, mail: Mail) -> ResultReport<(), MailFehler> {
        self.gesendet.lock().unwrap().push(mail);
        Ok(())
    }
}
