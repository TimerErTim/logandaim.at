use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use error_stack::{Report, ResultExt};
use logandaim_core::{
    App,
    adapters::{InMemorySigning, InMemoryThrottle},
    application::{AppBuilder, AuthContext, IdentityProviderConfiguration, SetUowProvider},
    domain::{AnlageId, RundeId, konto::AccessTokenClaims},
    ports::{Clock, Signing},
    service::{AnlageAnsicht, RundeAnsicht, RundeAnzeigenFehler, SessionPrüfen, StammdatenFehler},
};
use logandaim_fakes::{FakeMailSender, FixedClock};

/// Key every shared case signs and verifies with. A test-only constant seed.
pub fn test_signing_key() -> InMemorySigning {
    InMemorySigning::from_secret_bytes(&[7u8; 32])
}

/// A key that is valid on its own but is not the one a case's sessions were issued
/// under. Used by AC-17 to prove that the stored public key — not the configured key
/// alone — decides whether a token verifies.
pub fn fremder_schluessel() -> InMemorySigning {
    InMemorySigning::from_secret_bytes(&[9u8; 32])
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
pub fn test_konfiguration() -> IdentityProviderConfiguration {
    IdentityProviderConfiguration::neu("logandaim.at", "logandaim-api")
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
        let mail = self.mail.latest();
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

/// Read one Anlage as its admin view, straight from the repositories.
///
/// The read itself is the API facade's job now; a core case that only needs the aggregate
/// reads it here instead of going through a use case that no longer exists.
pub async fn anlage_ansicht(harness: &Harness, anlage_id: AnlageId) -> AnlageAnsicht {
    harness
        .app()
        .execute_fn(AuthContext::Anonymous, async |ctx| {
            let uow = ctx
                .enter()
                .await
                .change_context(StammdatenFehler::AnlageLaden)?;
            let anlage = uow
                .anlagen()
                .find_by_id(anlage_id)
                .await
                .map_err(error_stack::IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?
                .into_data();
            uow.finish(
                Ok::<_, Report<StammdatenFehler>>(AnlageAnsicht::neu(anlage)),
                StammdatenFehler::AnlageLaden,
            )
            .await
        })
        .await
        .expect("die Anlage ist lesbar")
}

/// Read one Runde with its folded Wertung, straight from the repositories.
pub async fn runde_ansicht(harness: &Harness, runde_id: RundeId) -> RundeAnsicht {
    harness
        .app()
        .execute_fn(AuthContext::Anonymous, async |ctx| {
            let uow = ctx
                .enter()
                .await
                .change_context(RundeAnzeigenFehler::RundeLaden)?;
            let runde = uow
                .runden()
                .find_by_id(runde_id)
                .await
                .map_err(error_stack::IntoReport::into_report)
                .change_context(RundeAnzeigenFehler::RundeLaden)?
                .into_data();
            let schüsse = uow
                .schüsse()
                .find_by_runde(runde_id)
                .await
                .map_err(error_stack::IntoReport::into_report)
                .change_context(RundeAnzeigenFehler::SchüsseLaden)?
                .into_iter()
                .map(|schuss| schuss.into_data())
                .collect();
            uow.finish(
                Ok::<_, Report<RundeAnzeigenFehler>>(RundeAnsicht::neu(runde, schüsse)),
                RundeAnzeigenFehler::RundeLaden,
            )
            .await
        })
        .await
        .expect("die Runde ist lesbar")
}
