use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use logandaim_core::{
    App,
    adapters::InMemorySigning,
    application::{AppBuilder, AuthContext, KontoKonfiguration, SetUowProvider},
    domain::konto::{AccessTokenClaims, SigningKeyId},
    ports::{Clock, Signing},
    service::SessionPrüfen,
};
use logandaim_fakes::{FakeMailSender, FixedClock, InMemoryThrottle};

/// Key every shared case signs and verifies with. A test-only constant seed.
pub fn test_signing_key() -> InMemorySigning {
    InMemorySigning::from_secret_bytes(
        SigningKeyId::new("test-key").expect("static test kid is valid"),
        &[7u8; 32],
    )
}

/// A key that is valid on its own but is not the one a case's sessions were issued
/// under. Used by AC-17 to prove that the stored public key — not the configured key
/// alone — decides whether a token verifies.
pub fn fremder_schluessel() -> InMemorySigning {
    InMemorySigning::from_secret_bytes(
        SigningKeyId::new("test-key").expect("static test kid is valid"),
        &[9u8; 32],
    )
}

/// The ASCII `header.payload` every case signs. Its content is irrelevant in core: only
/// that the bytes were signed by some key, and that the session row carries that key's
/// public half.
const SIGNIERTE_NACHRICHT: &str = "eyJhbGciOiJFZERTQSJ9.eyJzdWIiOiJ0ZXN0In0";

/// A `SessionPrüfen` whose signature was made by `signing`, over the same message every
/// case uses.
pub fn pruefen_mit(signing: &dyn Signing, claims: AccessTokenClaims) -> SessionPrüfen {
    SessionPrüfen {
        claims,
        signed_message: SIGNIERTE_NACHRICHT.to_owned(),
        signature: signing.sign(SIGNIERTE_NACHRICHT.as_bytes()),
    }
}

/// The common case: signed by the key the harness was built with.
pub fn pruefen(_harness: &Harness, claims: AccessTokenClaims) -> SessionPrüfen {
    pruefen_mit(&test_signing_key(), claims)
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
        .signing(Arc::new(test_signing_key()))
        .throttle(Arc::new(InMemoryThrottle::new()))
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
