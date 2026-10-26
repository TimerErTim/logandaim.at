use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;

use crate::ResultReport;
use crate::application::instrumented_repos::RepoStorage;
use crate::application::uow::UnitOfWorkImpl;
use crate::ports::{
    BenutzerRepository, Drossel, DrosselAktion, DrosselFehler, EmailBestaetigungRepository,
    KontoRepository, KundeSchluessel, Mail, MailFehler, MailSender, ObjectStore, ObjectStoreError,
    ObjectStream, PasswortWiederherstellungRepository, PdfDokument, PdfRenderError, PdfRenderer,
    RepositoryResult, SchussRepository, SessionRepository, SigningKeyRepository,
};

#[derive(Clone)]
pub struct InstrumentedObjectStore {
    inner: Arc<dyn ObjectStore>,
}

impl InstrumentedObjectStore {
    pub fn new(inner: Arc<dyn ObjectStore>) -> Arc<dyn ObjectStore> {
        Arc::new(Self { inner })
    }
}

#[async_trait]
impl ObjectStore for InstrumentedObjectStore {
    #[instrument(skip(self, bytes), fields(key, bytes_len = bytes.len()), level = "debug", err(Debug))]
    async fn put(&self, key: &str, bytes: &[u8]) -> ResultReport<(), ObjectStoreError> {
        self.inner.put(key, bytes).await
    }

    #[instrument(skip(self), fields(key), level = "debug", err(Debug))]
    async fn get(&self, key: &str) -> ResultReport<Option<ObjectStream>, ObjectStoreError> {
        self.inner.get(key).await
    }

    #[instrument(skip(self), fields(key), level = "debug", err(Debug))]
    async fn delete(&self, key: &str) -> ResultReport<(), ObjectStoreError> {
        self.inner.delete(key).await
    }
}

#[derive(Clone)]
pub struct InstrumentedPdfRenderer {
    inner: Arc<dyn PdfRenderer>,
}

impl InstrumentedPdfRenderer {
    pub fn new(inner: Arc<dyn PdfRenderer>) -> Arc<dyn PdfRenderer> {
        Arc::new(Self { inner })
    }
}

#[async_trait]
impl PdfRenderer for InstrumentedPdfRenderer {
    #[instrument(skip(self, dokument), level = "debug", err(Debug))]
    async fn rendern(&self, dokument: &PdfDokument) -> ResultReport<Vec<u8>, PdfRenderError> {
        self.inner.rendern(dokument).await
    }
}

#[derive(Clone)]
pub struct InstrumentedDrossel {
    inner: Arc<dyn Drossel>,
}

impl InstrumentedDrossel {
    pub fn new(inner: Arc<dyn Drossel>) -> Arc<dyn Drossel> {
        Arc::new(Self { inner })
    }
}

#[async_trait]
impl Drossel for InstrumentedDrossel {
    #[instrument(skip(self, kunde), fields(aktion = aktion.as_str(), ip = %kunde.ip), level = "debug", err(Debug))]
    async fn pruefe_und_zaehle(
        &self,
        kunde: &KundeSchluessel,
        aktion: DrosselAktion,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> ResultReport<(), DrosselFehler> {
        self.inner.pruefe_und_zaehle(kunde, aktion, jetzt).await
    }

    #[instrument(skip(self, kunde), fields(aktion = aktion.as_str(), ip = %kunde.ip), level = "debug", err(Debug))]
    async fn zuruecksetzen(
        &self,
        kunde: &KundeSchluessel,
        aktion: DrosselAktion,
    ) -> ResultReport<(), DrosselFehler> {
        self.inner.zuruecksetzen(kunde, aktion).await
    }
}

#[derive(Clone)]
pub struct InstrumentedMailSender {
    inner: Arc<dyn MailSender>,
}

impl InstrumentedMailSender {
    pub fn new(inner: Arc<dyn MailSender>) -> Arc<dyn MailSender> {
        Arc::new(Self { inner })
    }
}

#[async_trait]
impl MailSender for InstrumentedMailSender {
    // The body carries a single-use token, so it is never recorded in a span.
    #[instrument(skip(self, mail), fields(betreff = %mail.betreff), level = "debug", err(Debug))]
    async fn senden(&self, mail: Mail) -> ResultReport<(), MailFehler> {
        self.inner.senden(mail).await
    }
}

pub struct InstrumentedUnitOfWork {
    storage: Pin<Box<RepoStorage>>,
}

impl InstrumentedUnitOfWork {
    pub fn wrap(inner: Box<dyn UnitOfWorkImpl>) -> Box<dyn UnitOfWorkImpl> {
        Box::new(Self {
            storage: RepoStorage::new(inner),
        })
    }
}

#[async_trait]
impl UnitOfWorkImpl for InstrumentedUnitOfWork {
    #[instrument(skip(self), level = "debug", err(Debug))]
    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        let RepoStorage { uow, .. } = *Pin::into_inner(self.storage);
        uow.commit().await
    }

    #[instrument(skip(self), level = "debug", err(Debug))]
    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        let RepoStorage { uow, .. } = *Pin::into_inner(self.storage);
        uow.rollback().await
    }

    fn schüsse(&self) -> &dyn SchussRepository {
        &self.storage.schüsse
    }

    fn benutzer(&self) -> &dyn BenutzerRepository {
        &self.storage.benutzer
    }

    fn konten(&self) -> &dyn KontoRepository {
        &self.storage.konten
    }

    fn sessions(&self) -> &dyn SessionRepository {
        &self.storage.sessions
    }

    fn signing_keys(&self) -> &dyn SigningKeyRepository {
        &self.storage.signing_keys
    }

    fn bestaetigungen(&self) -> &dyn EmailBestaetigungRepository {
        &self.storage.bestaetigungen
    }

    fn wiederherstellungen(&self) -> &dyn PasswortWiederherstellungRepository {
        &self.storage.wiederherstellungen
    }
}
