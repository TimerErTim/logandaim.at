//! SMTP adapter for the `MailSender` port.
//!
//! Transaction mail is plain text: a token-bearing HTML mail invites link rewriting,
//! prefetching, and rendering bugs that a one-click link cannot afford, so this adapter
//! only ever builds a `text/plain; charset=utf-8` body.
//!
//! Configuration is data, not behaviour: credentials arrive from deployment configuration
//! and the relay is either Mailpit on loopback (development, tests) or the production
//! relay. A send failure is returned as [`MailError::Zustellung`]; the calling use case
//! logs it instead of failing the request that triggered it (SPEC-konto NFR-5).

use async_trait::async_trait;
use error_stack::{IntoReport, ResultExt};
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use logandaim_core::{
    ResultReport,
    domain::EmailAdresse,
    ports::{Mail, MailError, MailSender},
};

/// How the adapter talks to the relay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verschlüsselung {
    /// Plain SMTP: Mailpit on loopback in development and tests.
    Klar,
    /// `STARTTLS` upgrade, requiring a valid certificate.
    StartTls,
    /// TLS from the first byte (submission port 465).
    Implizit,
}

/// Where mail goes and who it comes from.
#[derive(Debug, Clone)]
pub struct SmtpKonfiguration {
    pub host: String,
    pub port: u16,
    pub verschlüsselung: Verschlüsselung,
    pub benutzername: Option<String>,
    pub passwort: Option<String>,
    pub absender: EmailAdresse,
    /// Display name shown in the mail client, e.g. "logandaim".
    pub absender_name: Option<String>,
    /// Frontend origin the mailed links point at. The routes are the product's; only the
    /// origin is deployment data (see [`logandaim_core::ports::Mail`]).
    pub frontend_basis_url: String,
}

impl SmtpKonfiguration {
    // TODO: improve ergonomics of having a hardcoded dev config in the codebase
    /// A local Mailpit relay: port 1025, plain SMTP, no credentials.
    pub fn mailpit(absender: EmailAdresse) -> Self {
        Self {
            host: "127.0.0.1".to_owned(),
            port: 1025,
            verschlüsselung: Verschlüsselung::Klar,
            benutzername: None,
            passwort: None,
            absender,
            absender_name: Some("logandaim".to_owned()),
            frontend_basis_url: "http://localhost:3000".to_owned(),
        }
    }
}

pub struct SmtpMailSender {
    konfiguration: SmtpKonfiguration,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpMailSender {
    /// Build the transport once and reuse its connection pool across sends.
    pub fn neu(konfiguration: SmtpKonfiguration) -> ResultReport<Self, MailError> {
        let mut builder = match konfiguration.verschlüsselung {
            Verschlüsselung::Klar => {
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(konfiguration.host.clone())
                    .port(konfiguration.port)
            }
            Verschlüsselung::StartTls => {
                let builder =
                    AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&konfiguration.host)
                        .change_context(MailError::NichtKonfiguriert)?;
                builder.port(konfiguration.port)
            }
            Verschlüsselung::Implizit => {
                let builder = AsyncSmtpTransport::<Tokio1Executor>::relay(&konfiguration.host)
                    .change_context(MailError::NichtKonfiguriert)?;
                builder.port(konfiguration.port)
            }
        };

        if let (Some(benutzer), Some(passwort)) = (
            konfiguration.benutzername.clone(),
            konfiguration.passwort.clone(),
        ) {
            builder = builder.credentials(Credentials::new(benutzer, passwort));
        }

        Ok(Self {
            transport: builder.build(),
            konfiguration,
        })
    }

    /// Build the wire message. Separate from sending so a unit test can assert recipient,
    /// subject, and body without a relay. This is where a token becomes a link: the
    /// use case does not know the frontend origin and must not know the route table.
    pub fn baue_nachricht(&self, mail: &Mail) -> ResultReport<Message, MailError> {
        let von = Mailbox::new(
            self.konfiguration.absender_name.clone(),
            self.konfiguration
                .absender
                .as_ref()
                .parse()
                .change_context(MailError::Zustellung)?,
        );
        let an = mail
            .empfänger
            .as_ref()
            .parse()
            .change_context(MailError::Zustellung)?;

        Message::builder()
            .from(von)
            .to(Mailbox::new(None, an))
            .subject(mail.betreff())
            .header(ContentType::TEXT_PLAIN)
            .body(mail.text(&self.konfiguration.frontend_basis_url))
            .map_err(IntoReport::into_report)
            .change_context(MailError::Zustellung)
    }
}

#[async_trait]
impl MailSender for SmtpMailSender {
    async fn send(&self, mail: Mail) -> ResultReport<(), MailError> {
        let nachricht = self.baue_nachricht(&mail)?;
        self.transport
            .send(nachricht)
            .await
            .map_err(IntoReport::into_report)
            .change_context(MailError::Zustellung)?;
        Ok(())
    }
}

/// A `MailSender` for a deployment with no relay configured.
///
/// It fails every send, which the calling use case logs as a warning (NFR-5). That is
/// deliberately louder than a fake that swallows mail: an operator who forgot to configure
/// SMTP gets one warning per attempted mail, not silent success.
#[derive(Debug, Default)]
pub struct KeinVersand;

#[async_trait]
impl MailSender for KeinVersand {
    async fn send(&self, _mail: Mail) -> ResultReport<(), MailError> {
        Err(error_stack::Report::new(MailError::NichtKonfiguriert))
    }
}
