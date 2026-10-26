use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use logandaim_core::{
    App,
    application::{AppBuilder, AuthContext, KontoKonfiguration, SetUowProvider},
    domain::konto::{SigningKeyId, SigningKeypair},
    ports::Clock,
};
use logandaim_fakes::{FakeMailSender, FixedClock, InMemoryDrossel};

/// Key every shared case signs and verifies with. A test-only constant seed.
pub fn test_signing_key() -> SigningKeypair {
    SigningKeypair::from_secret_bytes(
        SigningKeyId::new("test-key").expect("static test kid is valid"),
        &[7u8; 32],
    )
}

/// Token identity every shared case runs with.
pub fn test_konfiguration() -> KontoKonfiguration {
    KontoKonfiguration::neu("logandaim.at", "logandaim-api")
}

/// The frontend origin a case's mailed links are spelled from.
///
/// The core never builds a URL, so a case that wants to read a link renders the mail body
/// with the same base URL the real adapter would use.
pub const FRONTEND_BASIS_URL: &str = "https://logandaim.at";

/// A built app plus the infrastructure fakes a case has to reach into.
///
/// The ports are *shared* Arcs, so a case can read the mails that were "sent" and move the
/// clock forward without guessing at elapsed real time.
pub struct Harness {
    app: App,
    clock: Arc<FixedClock>,
    mail: Arc<FakeMailSender>,
}

impl Harness {
    pub fn app(&self) -> &App {
        &self.app
    }
    pub fn mail(&self) -> &FakeMailSender {
        self.mail.as_ref()
    }

    pub fn jetzt(&self) -> DateTime<Utc> {
        self.clock.now()
    }

    /// Move time forward for every subsequent use case run.
    pub fn vorspulen(&self, dauer: Duration) {
        self.clock.advance_by(dauer);
    }

    /// The token of the single most recent mail, for a link that was just mailed.
    pub fn letztes_token(&self) -> String {
        let mail = self.mail.letzte();
        let text = mail.text(FRONTEND_BASIS_URL);
        let marker = "token=";
        let start = text
            .find(marker)
            .unwrap_or_else(|| panic!("mail enthält keinen token-link: {text}"))
            + marker.len();
        text[start..]
            .chars()
            .take_while(|zeichen| {
                zeichen.is_ascii_alphanumeric() || *zeichen == '-' || *zeichen == '_'
            })
            .collect()
    }
}

/// Finish a harness builder with the test signing key and the in-memory infrastructure
/// ports: no case talks SMTP, and every throttle runs on its real thresholds.
pub fn build(builder: AppBuilder<SetUowProvider>) -> Harness {
    let clock = Arc::new(FixedClock::new(jetzt()));
    let mail = Arc::new(FakeMailSender::new());
    let app = builder
        .signing_key(test_signing_key())
        .drossel(Arc::new(InMemoryDrossel::new()))
        .mail_sender(mail.clone())
        .konto(test_konfiguration())
        .clock(clock.clone())
        .build();
    Harness { app, clock, mail }
}

/// Fixed instant shared by every case, so lifetimes are compared against one clock.
pub fn jetzt() -> DateTime<Utc> {
    DateTime::from_timestamp(1_700_000_000, 0).expect("static timestamp is valid")
}

/// Anonymous caller, for cases that do not authenticate.
pub fn anonym() -> AuthContext {
    AuthContext::Anonymous
}
