use std::sync::Arc;

use crate::adapters::{BlankPdfRenderer, InMemoryObjectStore, SystemClock};
use crate::application::context::ContextPorts;
use crate::domain::konto::SigningKeypair;
use crate::ports::{Clock, Drossel, MailSender, ObjectStore, PdfRenderer};
use crate::service::UseCase;
use crate::uow::UnitOfWorkProvider;
use error_stack::IntoReport;
use tracing::instrument;

pub mod uow;

pub mod ports;

pub mod auth;
pub use auth::{AuthContext, ZugriffFehler};

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
    /// Concrete domain key, built from deployment configuration at startup. There is no
    /// signer port: the signing algorithm is part of the `konto` subdomain's contract.
    pub signing_key: SigningKeypair,
    /// Client-scoped rate limiting. No default: a deployment must choose a store rather
    /// than silently run without throttles.
    pub drossel: Arc<dyn Drossel>,
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
    /// The configured signing key, so the HTTP edge can assemble and verify a JOSE
    /// envelope around the bytes core signs.
    pub fn signing_key(&self) -> &SigningKeypair {
        &self.signing_key
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

    /// Register the configured signing key in the key table, or verify the stored row.
    ///
    /// Runs once at startup. Idempotent across restarts, and *refuses* a stored row with
    /// the same `kid` but different material: silently replacing the public key would
    /// invalidate every access token already in circulation (AC-17).
    #[instrument(skip(self), err(Debug))]
    pub async fn signing_key_registrieren(
        &self,
    ) -> ResultReport<(), crate::ports::RepositoryError> {
        use crate::domain::konto::SigningKey;

        let ctx = self.new_execution_context(AuthContext::Anonymous);
        let schlüssel = SigningKey::neu(
            self.signing_key.kid().clone(),
            self.signing_key.public_key_hex(),
            self.clock.now(),
        );

        let uow = ctx.enter().await?;
        let ergebnis = uow.signing_keys().ensure(&schlüssel).await;
        uow.finish(ergebnis, crate::ports::RepositoryError::Unknown)
            .await?;
        tracing::info!(kid = %self.signing_key.kid().as_str(), "signing key registriert");
        Ok(())
    }

    fn new_execution_context(&self, auth: AuthContext) -> ExecutionContext<'_> {
        ExecutionContext::root(
            self.uow_provider.as_ref(),
            ContextPorts {
                auth,
                signing_key: &self.signing_key,
                clock: Arc::clone(&self.clock),
                object_store: Arc::clone(&self.object_store),
                pdf_renderer: Arc::clone(&self.pdf_renderer),
                drossel: Arc::clone(&self.drossel),
                mail_sender: Arc::clone(&self.mail_sender),
                konto: self.konto.clone(),
            },
        )
    }
}
