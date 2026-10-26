use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;

use crate::ResultReport;
use crate::application::instrumented_repos::RepoStorage;
use crate::application::uow::UnitOfWorkImpl;
use crate::ports::{
    BenutzerRepository, ClientKey, EmailBestätigungRepository, KontoRepository, Mail, MailError,
    MailSender, ObjectStore, ObjectStoreError, ObjectStream, PasswortWiederherstellungRepository,
    PdfDokument, PdfRenderError, PdfRenderer, RepositoryResult, SchussRepository,
    SessionRepository, Throttle, ThrottleAction, ThrottleError,
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
pub struct InstrumentedThrottle {
    inner: Arc<dyn Throttle>,
}

impl InstrumentedThrottle {
    pub fn new(inner: Arc<dyn Throttle>) -> Arc<dyn Throttle> {
        Arc::new(Self { inner })
    }
}

#[async_trait]
impl Throttle for InstrumentedThrottle {
    #[instrument(skip(self, client), fields(action = action.as_str(), ip = %client.ip), level = "debug", err(Debug))]
    async fn check_and_count(
        &self,
        client: &ClientKey,
        action: ThrottleAction,
        now: chrono::DateTime<chrono::Utc>,
    ) -> ResultReport<(), ThrottleError> {
        self.inner.check_and_count(client, action, now).await
    }

    #[instrument(skip(self, client), fields(action = action.as_str(), ip = %client.ip), level = "debug", err(Debug))]
    async fn reset(
        &self,
        client: &ClientKey,
        action: ThrottleAction,
    ) -> ResultReport<(), ThrottleError> {
        self.inner.reset(client, action).await
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
    #[instrument(skip(self, mail), fields(betreff = mail.betreff()), level = "debug", err(Debug))]
    async fn send(&self, mail: Mail) -> ResultReport<(), MailError> {
        self.inner.send(mail).await
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

    fn bestätigungen(&self) -> &dyn EmailBestätigungRepository {
        &self.storage.bestätigungen
    }

    fn wiederherstellungen(&self) -> &dyn PasswortWiederherstellungRepository {
        &self.storage.wiederherstellungen
    }
}
