//! SMTP adapter for the `MailSender` port.
//!
//! Transaction mail is plain text: a token-bearing HTML mail invites link rewriting,
//! prefetching, and rendering bugs that a one-click link cannot afford, so this adapter
//! only ever builds a `text/plain; charset=utf-8` body.
//!
//! Configuration is data, not behaviour: credentials arrive from deployment configuration
//! and the relay is either Mailpit on loopback (development, tests) or the production
//! relay. A send failure is returned as [`MailFehler::Zustellung`]; the calling use case
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
    ports::{Mail, MailFehler, MailSender},
};

/// How the adapter talks to the relay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verschluesselung {
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
    pub verschluesselung: Verschluesselung,
    pub benutzername: Option<String>,
    pub passwort: Option<String>,
    pub absender: EmailAdresse,
    /// Display name shown in the mail client, e.g. "logandaim".
    pub absender_name: Option<String>,
}

impl SmtpKonfiguration {
    /// A local Mailpit relay: port 1025, plain SMTP, no credentials.
    pub fn mailpit(absender: EmailAdresse) -> Self {
        Self {
            host: "127.0.0.1".to_owned(),
            port: 1025,
            verschluesselung: Verschluesselung::Klar,
            benutzername: None,
            passwort: None,
            absender,
            absender_name: Some("logandaim".to_owned()),
        }
    }
}

pub struct SmtpMailSender {
    konfiguration: SmtpKonfiguration,
    transport: AsyncSmtpTransport<Tokio1Executor>,
}

impl SmtpMailSender {
    /// Build the transport once and reuse its connection pool across sends.
    pub fn neu(konfiguration: SmtpKonfiguration) -> ResultReport<Self, MailFehler> {
        let mut builder = match konfiguration.verschluesselung {
            Verschluesselung::Klar => {
                AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(konfiguration.host.clone())
                    .port(konfiguration.port)
            }
            Verschluesselung::StartTls => {
                let builder =
                    AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&konfiguration.host)
                        .change_context(MailFehler::NichtKonfiguriert)?;
                builder.port(konfiguration.port)
            }
            Verschluesselung::Implizit => {
                let builder = AsyncSmtpTransport::<Tokio1Executor>::relay(&konfiguration.host)
                    .change_context(MailFehler::NichtKonfiguriert)?;
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
    /// subject, and body without a relay.
    pub fn baue_nachricht(&self, mail: &Mail) -> ResultReport<Message, MailFehler> {
        let von = Mailbox::new(
            self.konfiguration.absender_name.clone(),
            self.konfiguration
                .absender
                .as_ref()
                .parse()
                .change_context(MailFehler::Zustellung)?,
        );
        let an = mail
            .empfaenger
            .as_ref()
            .parse()
            .change_context(MailFehler::Zustellung)?;

        Message::builder()
            .from(von)
            .to(Mailbox::new(None, an))
            .subject(mail.betreff.clone())
            .header(ContentType::TEXT_PLAIN)
            .body(mail.text.clone())
            .map_err(IntoReport::into_report)
            .change_context(MailFehler::Zustellung)
    }
}

#[async_trait]
impl MailSender for SmtpMailSender {
    async fn senden(&self, mail: Mail) -> ResultReport<(), MailFehler> {
        let nachricht = self.baue_nachricht(&mail)?;
        self.transport
            .send(nachricht)
            .await
            .map_err(IntoReport::into_report)
            .change_context(MailFehler::Zustellung)?;
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
    async fn senden(&self, _mail: Mail) -> ResultReport<(), MailFehler> {
        Err(error_stack::Report::new(MailFehler::NichtKonfiguriert))
    }
}
