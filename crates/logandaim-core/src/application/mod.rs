use std::sync::Arc;

use crate::adapters::{BlankPdfRenderer, InMemoryObjectStore, SystemClock};
use crate::application::context::ContextPorts;
use crate::ports::{Clock, MailSender, ObjectStore, PdfRenderer, Signing, Throttle};
use crate::service::UseCase;
use crate::uow::UnitOfWorkProvider;
use error_stack::IntoReport;
use tracing::instrument;

pub mod uow;

pub mod ports;

pub mod auth;
pub use auth::{AccessError, AuthContext};

mod context;
pub use context::ExecutionContext;

mod konfiguration;
pub use konfiguration::KontoKonfiguration;

mod instrumented;
mod instrumented_repos;

mod errors;
pub use errors::ErrorReportExt;
pub use errors::ResultReport;
pub use errors::ThreadSafeError;

#[derive(bon::Builder)]
pub struct App {
    pub uow_provider: Box<dyn UnitOfWorkProvider>,
    /// The configured signing key behind an application port. The algorithm beneath it is
    /// domain code; startup only supplies key material.
    pub signing: Arc<dyn Signing>,
    /// Client-scoped rate limiting. No default: a deployment must choose a store rather
    /// than silently run without throttles.
    pub throttle: Arc<dyn Throttle>,
    /// Transaction mail. No default, for the same reason.
    pub mail_sender: Arc<dyn MailSender>,
    /// Mailed link targets and token identity. Deployment data, not behaviour.
    pub konto: KontoKonfiguration,
    #[builder(default = Arc::new(SystemClock))]
    pub clock: Arc<dyn Clock>,
    #[builder(default = Arc::new(InMemoryObjectStore::new()))]
    pub object_store: Arc<dyn ObjectStore>,
    #[builder(default = Arc::new(BlankPdfRenderer))]
    pub pdf_renderer: Arc<dyn PdfRenderer>,
}

pub use app_builder::{SetClock, SetUowProvider};

impl App {
    /// The configured signing key port, so the HTTP edge can assemble and verify a JOSE
    /// envelope around the bytes core signs.
    pub fn signing(&self) -> &dyn Signing {
        self.signing.as_ref()
    }

    #[instrument(
        skip(self, use_case),
        fields(use_case = std::any::type_name::<U>(), auth = ?auth),
        err(Debug)
    )]
    pub async fn execute<U: UseCase<O> + Send, O>(
        &self,
        auth: AuthContext,
        use_case: U,
    ) -> ResultReport<O, <U::Error as IntoReport>::Context> {
        use_case.perform(self.new_execution_context(auth)).await
    }

    #[instrument(skip(self, f), fields(execute_fn = true, auth = ?auth), err(Debug))]
    pub async fn execute_fn<F, O, E: ThreadSafeError>(
        &self,
        auth: AuthContext,
        f: F,
    ) -> ResultReport<O, E>
    where
        for<'a> F: AsyncFnOnce(ExecutionContext<'a>) -> ResultReport<O, E> + Send,
        E: Send,
    {
        f(self.new_execution_context(auth)).await
    }

    fn new_execution_context(&self, auth: AuthContext) -> ExecutionContext<'_> {
        ExecutionContext::root(
            self.uow_provider.as_ref(),
            ContextPorts {
                auth,
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
}
