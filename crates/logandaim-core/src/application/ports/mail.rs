use crate::application::ResultReport;
use crate::domain::EmailAdresse;

/// What a transaction mail is about.
///
/// The use case names the *kind* and carries the token; it never builds a URL. The
/// frontend base URL is deployment data that only the sending adapter has, and the
/// routes below are the product's, so the adapter is the one place that knows how a
/// token becomes a clickable link (SPEC-konto Q-17).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MailArt {
    /// Confirmation of an address during registration step 1.
    EmailBestätigung { token: String },
    /// A requested password recovery.
    PasswortWiederherstellung { token: String },
}

impl MailArt {
    /// Frontend route the mailed link points at, appended to the frontend base URL.
    fn route(&self) -> &'static str {
        match self {
            Self::EmailBestätigung { .. } => "/registrierung",
            Self::PasswortWiederherstellung { .. } => "/passwort",
        }
    }

    fn token(&self) -> &str {
        match self {
            Self::EmailBestätigung { token } | Self::PasswortWiederherstellung { token } => token,
        }
    }
}

/// One plain-text transaction mail.
///
/// Plain text only: a token-bearing HTML mail invites link rewriting, prefetching, and
/// rendering bugs that a one-click link cannot afford.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mail {
    pub empfänger: EmailAdresse,
    pub art: MailArt,
}

impl Mail {
    pub fn email_bestätigung(empfänger: EmailAdresse, token: impl Into<String>) -> Self {
        Self {
            empfänger,
            art: MailArt::EmailBestätigung {
                token: token.into(),
            },
        }
    }

    pub fn passwort_wiederherstellung(empfänger: EmailAdresse, token: impl Into<String>) -> Self {
        Self {
            empfänger,
            art: MailArt::PasswortWiederherstellung {
                token: token.into(),
            },
        }
    }

    pub fn betreff(&self) -> &'static str {
        match self.art {
            MailArt::EmailBestätigung { .. } => "Bestätige deine E-Mail-Adresse",
            MailArt::PasswortWiederherstellung { .. } => "Setze dein Passwort zurück",
        }
    }

    /// The body, with the link spelled from `frontend_basis_url` and the route this mail
    /// is about. Trailing slashes on the base URL are ignored, so a deployment that
    /// configures `https://logandaim.at/` does not produce a double slash.
    pub fn text(&self, frontend_basis_url: &str) -> String {
        let link = format!(
            "{}{}?token={}",
            frontend_basis_url.trim_end_matches('/'),
            self.art.route(),
            self.art.token()
        );

        match self.art {
            MailArt::EmailBestätigung { .. } => format!(
                "Hallo,\n\n\
                 bitte bestätige deine E-Mail-Adresse für logandaim.at:\n\n\
                 {link}\n\n\
                 Der Link ist 24 Stunden gültig und kann nur einmal verwendet werden.\n\
                 Wenn du dich nicht registriert hast, ignoriere diese Mail.\n"
            ),
            MailArt::PasswortWiederherstellung { .. } => format!(
                "Hallo,\n\n\
                 über diesen Link kannst du ein neues Passwort für logandaim.at setzen:\n\n\
                 {link}\n\n\
                 Der Link ist 60 Minuten gültig und kann nur einmal verwendet werden.\n\
                 Alle bestehenden Anmeldungen werden dabei beendet.\n\
                 Wenn du ihn nicht angefordert hast, ignoriere diese Mail.\n"
            ),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum MailError {
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
    async fn send(&self, mail: Mail) -> ResultReport<(), MailError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn adresse() -> EmailAdresse {
        EmailAdresse::normalisiert("anna@muster.at").expect("static address is valid")
    }

    #[test_log::test]
    fn der_link_wird_aus_der_basis_url_und_der_route_gebaut() {
        let mail = Mail::email_bestätigung(adresse(), "abc");
        assert!(
            mail.text("http://localhost:3000")
                .contains("http://localhost:3000/registrierung?token=abc")
        );

        let mail = Mail::passwort_wiederherstellung(adresse(), "xyz");
        assert!(
            mail.text("http://localhost:3000")
                .contains("http://localhost:3000/passwort?token=xyz")
        );
    }

    #[test_log::test]
    fn ein_schrägstrich_am_ende_erzeugt_keinen_doppelten() {
        let mail = Mail::email_bestätigung(adresse(), "abc");
        assert!(
            mail.text("https://logandaim.at/")
                .contains("https://logandaim.at/registrierung?token=abc")
        );
    }
}
