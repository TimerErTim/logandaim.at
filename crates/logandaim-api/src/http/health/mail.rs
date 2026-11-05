//! The verdict behind `/health/mail`, refreshed on read.
//!
//! A read answers from the stored verdict and, if that is stale or not `Ok`, starts one probe
//! in the background. The route therefore never waits on the relay (NFR-5), and a polling
//! healthcheck is what keeps the verdict current.

use std::sync::{Arc, atomic::Ordering};

use chrono::{TimeDelta, Utc};
use logandaim_core::ports::MailSender;
use tracing::{error, info};

use crate::http::LogandaimApiSpec;
use crate::schema::{MailHealth, MailHealthFailed, MailHealthOk};

/// How long an `Ok` is trusted before the next read probes again.
const PROBE_INTERVAL: TimeDelta = TimeDelta::seconds(60);

impl LogandaimApiSpec {
    pub(in crate::http) fn mail_health(&self) -> MailHealth {
        let health = self
            .mail_health
            .read()
            .expect("no panic holds the mail health lock")
            .clone();

        let fresh =
            matches!(&health, MailHealth::Ok(ok) if Utc::now() - ok.checked_at < PROBE_INTERVAL);
        if !fresh && !self.mail_probe_running.swap(true, Ordering::AcqRel) {
            let sender = Arc::clone(&self.app_api.inner_app().mail_sender);
            let state = Arc::clone(&self.mail_health);
            let running = Arc::clone(&self.mail_probe_running);
            tokio::spawn(async move {
                let next = probe(sender.as_ref()).await;
                let previous = std::mem::replace(
                    &mut *state.write().expect("no panic holds the mail health lock"),
                    next.clone(),
                );
                log_change(&previous, &next);
                running.store(false, Ordering::Release);
            });
        }

        health
    }
}

async fn probe(sender: &dyn MailSender) -> MailHealth {
    let checked_at = Utc::now();
    match sender.prüfe_verbindung().await {
        Ok(()) => MailHealth::Ok(MailHealthOk { checked_at }),
        Err(fehler) => MailHealth::Failed(MailHealthFailed {
            checked_at,
            detail: format!("{fehler:#}"),
        }),
    }
}

/// Log only a change of verdict: a relay that stays broken would otherwise repeat its line
/// on every healthcheck.
fn log_change(previous: &MailHealth, next: &MailHealth) {
    match (previous, next) {
        (MailHealth::Ok(_), MailHealth::Ok(_))
        | (MailHealth::Failed(_), MailHealth::Failed(_))
        | (_, MailHealth::Unknown(_)) => {}
        (_, MailHealth::Ok(_)) => info!("SMTP-Relay erreichbar; Mailversand ist verfügbar"),
        (_, MailHealth::Failed(failed)) => error!(
            "SMTP-Relay nicht erreichbar; Transaktionsmails schlagen fehl: {}",
            failed.detail
        ),
    }
}
