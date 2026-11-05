//! The mail relay's health, shared between a background probe and the health routes.
//!
//! Mail is part of the MVP, but a request must never wait on the relay (NFR-5). The two
//! facts combine into the shape here: an independent loop probes the relay on its own
//! schedule and records what it found, and a route only reads that record. A registration
//! therefore succeeds or fails on its own merits, while an operator can still ask whether
//! mail works.
//!
//! The probe is a trait so the server keeps its concrete SMTP transport and the API crate
//! stays free of one. Tests implement it with a fake.

use std::sync::{Arc, RwLock};
use std::time::Duration;

use async_trait::async_trait;
use chrono::Utc;
use tracing::{debug, error, info, warn};

use crate::schema::{MailHealth, MailHealthStatus};

/// One check of the relay: connect, upgrade TLS, authenticate, and hang up.
///
/// `Ok(false)` means the relay answered but did not confirm the connection; `Err` carries
/// the reason. Both mean sending would fail and are reported the same way, because an
/// operator acts on them the same way.
///
/// The error is a rendered sentence rather than the port's error type on purpose: the probe
/// is a status check, not a delivery, and its only reader is a person reading the endpoint.
#[async_trait]
pub trait MailProbe: Send + Sync {
    async fn check_connection(&self) -> Result<bool, String>;
}

/// The last verdict, readable from any handler and written only by the probe loop.
#[derive(Debug, Default)]
pub struct MailHealthState(RwLock<MailHealth>);

impl MailHealthState {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// The current verdict, as of the last probe.
    pub fn read(&self) -> MailHealth {
        self.0
            .read()
            .expect("the mail health lock is never poisoned: no panic holds it")
            .clone()
    }

    /// Record what a probe found.
    pub fn write_verdict(&self, health: MailHealth) {
        *self
            .0
            .write()
            .expect("the mail health lock is never poisoned: no panic holds it") = health;
    }
}

/// How long between probes. Long enough that a restart's first failure is not re-tried
/// every few seconds forever, short enough that a relay that dies after boot is noticed
/// within a minute.
pub const PROBE_INTERVAL: Duration = Duration::from_secs(60);

/// The first probe is more patient than later ones: a relay that is still bootstrapping
/// (writing its config, restarting once) must not be reported as broken while it starts.
const FIRST_ATTEMPTS: u32 = 12;
const FIRST_RETRY_DELAY: Duration = Duration::from_secs(5);

/// Probe the relay forever, recording each verdict in `state`.
///
/// Runs as its own task: it must never delay a request, and a failing relay must never stop
/// the API from serving (NFR-5). The first cycle keeps retrying so a bootstrapping relay is
/// not reported broken; after that each cycle is one probe and the loop waits.
pub async fn monitor_mail(probe: Arc<dyn MailProbe>, state: Arc<MailHealthState>) -> ! {
    // First cycle: the relay may still be starting.
    let mut last_error = String::new();
    let mut reachable = false;
    for attempt in 1..=FIRST_ATTEMPTS {
        match probe.check_connection().await {
            Ok(true) => {
                reachable = true;
                break;
            }
            Ok(false) => last_error = "Relay antwortet nicht auf NOOP".to_owned(),
            Err(error) => last_error = error,
        }
        if attempt == FIRST_ATTEMPTS {
            break;
        }
        debug!(attempt, "SMTP-Relay noch nicht erreichbar: {last_error}");
        tokio::time::sleep(FIRST_RETRY_DELAY).await;
    }
    record(&state, reachable, last_error);

    loop {
        tokio::time::sleep(PROBE_INTERVAL).await;
        let (reachable, detail) = match probe.check_connection().await {
            Ok(true) => (true, String::new()),
            Ok(false) => (false, "Relay antwortet nicht auf NOOP".to_owned()),
            Err(error) => (false, error),
        };
        record(&state, reachable, detail);
    }
}

/// Store a verdict and log it, but only when it changes: a relay that stays broken would
/// otherwise write the same error line every minute.
fn record(state: &MailHealthState, reachable: bool, detail: String) {
    let previous = state.read();
    let health = if reachable {
        MailHealth {
            status: MailHealthStatus::Ok,
            checked_at: Some(Utc::now()),
            detail: String::new(),
        }
    } else {
        MailHealth {
            status: MailHealthStatus::Failed,
            checked_at: Some(Utc::now()),
            detail,
        }
    };

    let unchanged = previous.status == health.status;
    state.write_verdict(health.clone());

    match (health.status, unchanged) {
        (MailHealthStatus::Ok, false) => {
            info!("SMTP-Relay erreichbar; Mailversand ist verfügbar")
        }
        (MailHealthStatus::Failed, false) => error!(
            "SMTP-Relay nicht erreichbar; Transaktionsmails schlagen fehl: {}",
            health.detail
        ),
        // Still the same answer: one line at debug keeps the log useful without noise.
        (MailHealthStatus::Ok, true) => debug!("SMTP-Relay weiterhin erreichbar"),
        (MailHealthStatus::Failed, true) => warn!("SMTP-Relay weiterhin nicht erreichbar"),
        (MailHealthStatus::Unknown, _) => {}
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    /// A probe that answers a fixed sequence and then repeats the last answer.
    struct Scripted {
        answers: Vec<Result<bool, String>>,
        calls: AtomicUsize,
    }

    impl Scripted {
        fn new(answers: Vec<Result<bool, String>>) -> Arc<Self> {
            Arc::new(Self {
                answers,
                calls: AtomicUsize::new(0),
            })
        }
    }

    #[async_trait]
    impl MailProbe for Scripted {
        async fn check_connection(&self) -> Result<bool, String> {
            let index = self.calls.fetch_add(1, Ordering::SeqCst);
            self.answers
                .get(index)
                .or_else(|| self.answers.last())
                .cloned()
                .unwrap_or(Ok(true))
        }
    }

    /// A relay that is broken on the first cycle and healthy on the next must end up
    /// reported as `ok`. Driven through the real loop rather than by calling `record`, so
    /// the retry-then-recover path the server runs is what the test covers.
    #[test_log::test(tokio::test)]
    async fn the_monitor_reports_a_relay_that_recovers() {
        let state = MailHealthState::new();
        let probe = Scripted::new(vec![Err("Verbindung abgelehnt".to_owned()), Ok(true)]);

        let monitor = tokio::spawn(monitor_mail(probe.clone(), state.clone()));
        // Two cycles: the first attempt, then one interval later the recovery.
        for _ in 0..200 {
            if state.read().status == MailHealthStatus::Ok {
                break;
            }
            tokio::time::sleep(PROBE_INTERVAL / 20).await;
        }
        monitor.abort();

        let health = state.read();
        assert_eq!(health.status, MailHealthStatus::Ok);
        assert!(health.detail.is_empty());
        assert!(probe.calls.load(Ordering::SeqCst) >= 2);
    }

    #[test_log::test]
    fn the_initial_verdict_is_unknown() {
        let state = MailHealthState::new();

        let health = state.read();

        assert_eq!(health.status, MailHealthStatus::Unknown);
        assert_eq!(health.checked_at, None);
    }

    /// The whole point of the shared state: a verdict can move `unknown -> ok -> failed ->
    /// ok` and the last write is what a route reads. A stale `ok` after a failure would be
    /// worse than no verdict at all, because a deploy gate would pass.
    #[test_log::test]
    fn a_verdict_replaces_the_previous_one() {
        let state = MailHealthState::new();

        record(&state, true, String::new());
        assert_eq!(state.read().status, MailHealthStatus::Ok);

        record(&state, false, "Verbindung abgelehnt".to_owned());
        let failed = state.read();
        assert_eq!(failed.status, MailHealthStatus::Failed);
        assert_eq!(failed.detail, "Verbindung abgelehnt");
        assert!(failed.checked_at.is_some());

        record(&state, true, String::new());
        let recovered = state.read();
        assert_eq!(recovered.status, MailHealthStatus::Ok);
        assert!(recovered.detail.is_empty());
    }

    /// A success must clear the previous reason, otherwise the endpoint keeps serving a
    /// sentence about a failure that no longer applies.
    #[test_log::test]
    fn a_recovered_relay_reports_no_reason() {
        let state = MailHealthState::new();
        record(&state, false, "TLS-Fehler".to_owned());

        record(&state, true, String::new());

        assert!(state.read().detail.is_empty());
    }
}
