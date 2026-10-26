use crate::application::ResultReport;
use crate::domain::EmailAdresse;

/// One plain-text transaction mail.
///
/// Plain text only: a token-bearing HTML mail invites link rewriting, prefetching, and
/// rendering bugs that a one-click link cannot afford.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mail {
    pub empfaenger: EmailAdresse,
    pub betreff: String,
    pub text: String,
}

impl Mail {
    pub fn neu(
        empfaenger: EmailAdresse,
        betreff: impl Into<String>,
        text: impl Into<String>,
    ) -> Self {
        Self {
            empfaenger,
            betreff: betreff.into(),
            text: text.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MailFehler {
    #[error("die Mail konnte nicht zugestellt werden")]
    Zustellung,
    #[error("der Mailversand ist nicht konfiguriert")]
    NichtKonfiguriert,
}

/// Outbound transaction mail. One adapter per deployment; Mailpit in development and
/// tests, the configured relay in production (SPEC-konto FR-11, AC-19).
///
/// A failing send never fails the request that caused it: use cases call this after their
/// unit of work commits and log the failure instead of propagating it (NFR-5).
#[async_trait::async_trait]
pub trait MailSender: Send + Sync {
    async fn senden(&self, mail: Mail) -> ResultReport<(), MailFehler>;
}
