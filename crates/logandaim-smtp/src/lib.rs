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
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
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
    /// Host the TCP connection is opened to.
    pub host: String,
    pub port: u16,
    pub verschlüsselung: Verschlüsselung,
    /// Name the server certificate is verified against, when it differs from [`Self::host`].
    /// `None` means the host is also the certified name.
    pub tls_hostname: Option<String>,
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
            tls_hostname: None,
            benutzername: None,
            passwort: None,
            absender,
            absender_name: Some("logandaim".to_owned()),
            frontend_basis_url: "http://localhost:3000".to_owned(),
        }
    }
}

/// TLS parameters for the configured hop: verified against the certified name, not the
/// host the connection is opened to.
fn tls_parameter(konfiguration: &SmtpKonfiguration) -> ResultReport<TlsParameters, MailError> {
    let name = konfiguration
        .tls_hostname
        .as_deref()
        .unwrap_or(&konfiguration.host)
        .to_owned();
    TlsParameters::new(name).change_context(MailError::NichtKonfiguriert)
}

#[derive(Debug, Clone)]
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
                // `starttls_relay` would verify the certificate against the host string,
                // which is a container name on the Compose network and never what a CA
                // issues for. Build the parameters from the certified name instead, and
                // keep the connection pointed at the host that resolves.
                let builder = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
                    konfiguration.host.clone(),
                )
                .tls(Tls::Required(tls_parameter(&konfiguration)?));
                builder.port(konfiguration.port)
            }
            Verschlüsselung::Implizit => {
                let builder = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
                    konfiguration.host.clone(),
                )
                .tls(Tls::Wrapper(tls_parameter(&konfiguration)?));
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

    /// Open one connection, complete the TLS upgrade and authentication, then close it.
    ///
    /// A send would prove the same hop but deliver a message; NOOP proves it without a
    /// recipient, so a deployment can report whether mail works before anyone registers.
    pub async fn prüfe_verbindung(&self) -> ResultReport<bool, MailError> {
        self.transport
            .test_connection()
            .await
            .map_err(IntoReport::into_report)
            .change_context(MailError::Zustellung)
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

#[cfg(test)]
mod tests {
    use super::*;

    fn konfiguration() -> SmtpKonfiguration {
        let mut konfiguration =
            SmtpKonfiguration::mailpit(EmailAdresse::normalisiert("noreply@logandaim.at").unwrap());
        konfiguration.host = "stalwart".to_owned();
        konfiguration.port = 587;
        konfiguration.verschlüsselung = Verschlüsselung::StartTls;
        konfiguration
    }

    /// The defect this exists for: a container name reaches the relay but is never what a
    /// CA issues for. Verification has to follow `tls_hostname`, not `host`.
    #[test_log::test]
    fn tls_wird_gegen_den_zertifizierten_namen_geprüft() {
        let mut konfiguration = konfiguration();
        konfiguration.tls_hostname = Some("mail.logandaim.at".to_owned());

        let parameter = tls_parameter(&konfiguration).expect("TLS-Parameter bauen");

        assert_eq!(parameter.domain(), "mail.logandaim.at");
    }

    /// Without an override the host is also the certified name, as before.
    #[test_log::test]
    fn ohne_überschreibung_ist_der_host_der_zertifizierte_name() {
        let konfiguration = konfiguration();

        let parameter = tls_parameter(&konfiguration).expect("TLS-Parameter bauen");

        assert_eq!(parameter.domain(), "stalwart");
    }
}
