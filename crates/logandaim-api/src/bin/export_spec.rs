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
    application::KontoKonfiguration,
    domain::konto::{SigningKeyId, SigningKeypair},
    ports::{
        Drossel, DrosselAktion, DrosselFehler, KundeSchluessel, Mail, MailFehler, MailSender,
        RepositoryResult,
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
struct KeineDrossel;

#[async_trait]
impl Drossel for KeineDrossel {
    async fn pruefe_und_zaehle(
        &self,
        _kunde: &KundeSchluessel,
        _aktion: DrosselAktion,
        _jetzt: DateTime<Utc>,
    ) -> Result<(), Report<DrosselFehler>> {
        Err(Report::new(DrosselFehler::NichtVerfuegbar))
    }

    async fn zuruecksetzen(
        &self,
        _kunde: &KundeSchluessel,
        _aktion: DrosselAktion,
    ) -> Result<(), Report<DrosselFehler>> {
        Err(Report::new(DrosselFehler::NichtVerfuegbar))
    }
}

/// Accepts nothing: the document is built from types, never by sending mail.
struct KeinVersand;

#[async_trait]
impl MailSender for KeinVersand {
    async fn senden(&self, _mail: Mail) -> Result<(), Report<MailFehler>> {
        Err(Report::new(MailFehler::NichtKonfiguriert))
    }
}

fn main() {
    let dummy_app = App::builder()
        .uow_provider(Box::new(Unimplemented))
        .signing_key(SigningKeypair::from_secret_bytes(
            SigningKeyId::new("spec-export").expect("static kid is valid"),
            &[0u8; 32],
        ))
        .drossel(Arc::new(KeineDrossel))
        .mail_sender(Arc::new(KeinVersand))
        .konto(KontoKonfiguration::neu(
            "https://logandaim.at/registrierung",
            "https://logandaim.at/passwort",
            "logandaim.at",
            "logandaim-api",
        ))
        .build();

    let api_service = openapi_service(dummy_app, std::iter::empty::<String>());
    println!("{}", api_service.spec());
}
