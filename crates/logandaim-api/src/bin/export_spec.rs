//! Emit the OpenAPI document without a database.
//!
//! The spec only needs the *shape* of the app, not a live one, so the ports are stubs that
//! are never called. That is deliberate: exporting the spec in CI must not require
//! migrations, a signing key file, or an SMTP relay.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use error_stack::Report;
use logandaim_api::openapi_service;
use logandaim_core::{
    App,
    adapters::InMemorySigning,
    application::{IdentityProviderConfiguration, ResultReport},
    domain::konto::SigningKeyId,
    ports::{
        ClientKey, Mail, MailError, MailSender, RepositoryResult, Throttle, ThrottleAction,
        ThrottleError,
    },
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

struct Unimplemented;

#[async_trait]
impl UnitOfWorkProvider for Unimplemented {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>> {
        unimplemented!()
    }
}

/// Refuses every throttle check, so an accidental use during export fails loudly instead of
/// silently pretending the port works.
struct NoThrottle;

#[async_trait]
impl Throttle for NoThrottle {
    async fn check_and_count(
        &self,
        _client: &ClientKey,
        _action: ThrottleAction,
        _now: DateTime<Utc>,
    ) -> ResultReport<(), ThrottleError> {
        Err(Report::new(ThrottleError::Unavailable))
    }

    async fn reset(
        &self,
        _client: &ClientKey,
        _action: ThrottleAction,
    ) -> ResultReport<(), ThrottleError> {
        Err(Report::new(ThrottleError::Unavailable))
    }
}

/// Accepts nothing: the document is built from types, never by sending mail.
struct NoMailSender;

#[async_trait]
impl MailSender for NoMailSender {
    async fn send(&self, _mail: Mail) -> ResultReport<(), MailError> {
        Err(Report::new(MailError::NichtKonfiguriert))
    }
}

fn main() {
    let dummy_app = App::builder()
        .uow_provider(Box::new(Unimplemented))
        .signing(Arc::new(InMemorySigning::from_secret_bytes(
            SigningKeyId::new("spec-export").expect("static kid is valid"),
            &[0u8; 32],
        )))
        .throttle(Arc::new(NoThrottle))
        .mail_sender(Arc::new(NoMailSender))
        .konto(IdentityProviderConfiguration::neu(
            "logandaim.at",
            "logandaim-api",
        ))
        .build();

    let api_service = openapi_service(dummy_app, std::iter::empty::<String>());
    println!("{}", api_service.spec());
}
