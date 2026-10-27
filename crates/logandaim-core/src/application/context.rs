use std::sync::Arc;

use crate::{
    application::instrumented::{
        InstrumentedMailSender, InstrumentedObjectStore, InstrumentedPdfRenderer,
        InstrumentedThrottle, InstrumentedUnitOfWork,
    },
    application::uow::{UnitOfWork, UnitOfWorkImpl, UnitOfWorkProvider},
    ports::{Clock, MailSender, ObjectStore, PdfRenderer, RepositoryResult, Signing, Throttle},
};
use tracing::instrument;

use super::auth::AuthContext;
use super::konfiguration::IdentityProviderConfiguration;

#[derive(Clone, Copy)]
enum ExecutionSource<'a> {
    Root(&'a dyn UnitOfWorkProvider),
    Nested(&'a dyn UnitOfWorkImpl),
}

/// Fresh per `App::execute` / `execute_fn`. Holds ports, the executing identity, and a UoW
/// source (root provider or nested impl). Does not hold an active transaction — call
/// [`ExecutionContext::enter`].
pub struct ExecutionContext<'a> {
    source: ExecutionSource<'a>,
    auth: AuthContext,
    signing: Arc<dyn Signing>,
    clock: Arc<dyn Clock>,
    object_store: Arc<dyn ObjectStore>,
    pdf_renderer: Arc<dyn PdfRenderer>,
    throttle: Arc<dyn Throttle>,
    mail_sender: Arc<dyn MailSender>,
    konto: IdentityProviderConfiguration,
}

/// Everything a context carries besides its UoW source, so `root`, `nested`, and `sub`
/// cannot drift apart.
pub(crate) struct ContextPorts {
    pub(crate) auth: AuthContext,
    pub(crate) signing: Arc<dyn Signing>,
    pub(crate) clock: Arc<dyn Clock>,
    pub(crate) object_store: Arc<dyn ObjectStore>,
    pub(crate) pdf_renderer: Arc<dyn PdfRenderer>,
    pub(crate) throttle: Arc<dyn Throttle>,
    pub(crate) mail_sender: Arc<dyn MailSender>,
    pub(crate) konto: IdentityProviderConfiguration,
}

impl<'a> ExecutionContext<'a> {
    pub(super) fn root(
        provider: &'a dyn UnitOfWorkProvider,
        ports: ContextPorts,
    ) -> ExecutionContext<'a> {
        Self {
            source: ExecutionSource::Root(provider),
            auth: ports.auth,
            signing: ports.signing,
            clock: ports.clock,
            object_store: InstrumentedObjectStore::new(ports.object_store),
            pdf_renderer: InstrumentedPdfRenderer::new(ports.pdf_renderer),
            throttle: InstrumentedThrottle::new(ports.throttle),
            mail_sender: InstrumentedMailSender::new(ports.mail_sender),
            konto: ports.konto,
        }
    }

    pub(super) fn nested(uow: &'a dyn UnitOfWorkImpl, ports: ContextPorts) -> ExecutionContext<'a> {
        Self {
            source: ExecutionSource::Nested(uow),
            auth: ports.auth,
            signing: ports.signing,
            clock: ports.clock,
            object_store: InstrumentedObjectStore::new(ports.object_store),
            pdf_renderer: InstrumentedPdfRenderer::new(ports.pdf_renderer),
            throttle: InstrumentedThrottle::new(ports.throttle),
            mail_sender: InstrumentedMailSender::new(ports.mail_sender),
            konto: ports.konto,
        }
    }

    pub fn clock(&self) -> &dyn Clock {
        self.clock.as_ref()
    }

    pub fn object_store(&self) -> &dyn ObjectStore {
        self.object_store.as_ref()
    }

    pub fn pdf_renderer(&self) -> &dyn PdfRenderer {
        self.pdf_renderer.as_ref()
    }

    /// Client-scoped rate limiting. Use cases call it directly and map a refusal to their
    /// own `Throttled` variant.
    pub fn throttle(&self) -> &dyn Throttle {
        self.throttle.as_ref()
    }

    pub fn mail_sender(&self) -> &dyn MailSender {
        self.mail_sender.as_ref()
    }

    /// Mailed link targets and token identity of this deployment.
    pub fn konto_konfiguration(&self) -> &IdentityProviderConfiguration {
        &self.konto
    }

    /// Who is executing this use case. Injected by the API layer, never by a use case.
    pub fn auth(&self) -> AuthContext {
        self.auth
    }

    /// The configured signing key as a port. The algorithm below it is domain code; this
    /// accessor is how the application composes and checks a token.
    pub fn signing(&self) -> &dyn Signing {
        self.signing.as_ref()
    }

    /// Nested context that joins `uow`'s transaction and copies this context's ports and
    /// identity.
    pub fn sub<'b>(&self, uow: &'b UnitOfWork<'b>) -> ExecutionContext<'b> {
        ExecutionContext::nested(
            uow.as_impl(),
            ContextPorts {
                auth: self.auth,
                signing: Arc::clone(&self.signing),
                clock: Arc::clone(&self.clock),
                object_store: Arc::clone(&self.object_store),
                pdf_renderer: Arc::clone(&self.pdf_renderer),
                throttle: Arc::clone(&self.throttle),
                mail_sender: Arc::clone(&self.mail_sender),
                konto: self.konto.clone(),
            },
        )
    }

    /// Start a unit of work. Returned UoW borrows `self`, so this context cannot be
    /// moved into a nested `perform` until the UoW is committed or rolled back.
    /// Nested contexts (from [`ExecutionContext::sub`]) join the outer transaction.
    #[instrument(skip(self), level = "debug", err(Debug))]
    pub async fn enter(&self) -> RepositoryResult<UnitOfWork<'_>> {
        match self.source {
            ExecutionSource::Root(provider) => {
                let implementation = InstrumentedUnitOfWork::wrap(provider.begin().await?);
                Ok(UnitOfWork::owned(implementation))
            }
            ExecutionSource::Nested(inner) => Ok(UnitOfWork::locked(inner)),
        }
    }
}
