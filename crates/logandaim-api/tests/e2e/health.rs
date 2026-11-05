//! The two health routes, and the contract that keeps them apart.
//!
//! `GET /api/health` answers the literal `OK`, and three separate consumers parse exactly
//! that: the Compose healthcheck, `tasks/verify.toml`, and `deploy-vps-remote.sh`. A richer
//! body would break them silently, so the plain route stays plain and the mail verdict
//! lives on its own route.
//!
//! NFR-5 is the other half: a user request never waits on the relay. That is why
//! `/health/mail` reads a recorded verdict instead of probing, and why the probe loop lives
//! outside the request path.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::DateTime;
use logandaim_api::{
    health::MailHealthState,
    schema::{MailHealth, MailHealthStatus},
};
use logandaim_core::{
    App,
    application::IdentityProviderConfiguration,
    ports::RepositoryResult,
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};
use poem::http::StatusCode;

use super::LogandaimApiTestClient;

/// The document and the health routes need only the shape of the app: no port is called.
struct Unimplemented;

#[async_trait]
impl UnitOfWorkProvider for Unimplemented {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>> {
        unimplemented!()
    }
}

fn client(state: Arc<MailHealthState>) -> LogandaimApiTestClient {
    let app = App::builder()
        .uow_provider(Box::new(Unimplemented))
        .konto(IdentityProviderConfiguration::neu(
            super::ISSUER,
            super::AUDIENCE,
        ))
        .build();
    LogandaimApiTestClient::from_arc_with_mail_health(Arc::new(app), state)
}

fn verdict(status: MailHealthStatus, detail: &str) -> MailHealth {
    MailHealth {
        status,
        checked_at: Some(DateTime::from_timestamp(1_700_000_000, 0).unwrap_or_default()),
        detail: detail.to_owned(),
    }
}

/// The literal body the three external parsers match on. Changing this is a deployment
/// change, not a cosmetic one.
#[test_log::test(tokio::test)]
async fn gesundheit_antwortet_den_literal_ok() {
    let client = client(MailHealthState::new());

    let (status, body) = client.get_json("/api/health").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, serde_json::json!("OK"));
}

/// Before any probe reports, the honest answer is `Unknown`, not `Ok`. A default of `Ok`
/// would make a deploy gate pass on a relay nobody has tested.
#[test_log::test(tokio::test)]
async fn ohne_prüfergebnis_meldet_die_mailroute_unbekannt() {
    let client = client(MailHealthState::new());

    let (status, body) = client.get_json("/api/health/mail").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "Unknown");
    assert_eq!(body["checkedAt"], serde_json::Value::Null);
}

/// A recorded success is served as-is, with the moment it was observed.
#[test_log::test(tokio::test)]
async fn ein_aufgezeichnetes_ergebnis_wird_ausgeliefert() {
    let state = MailHealthState::new();
    state.write_verdict(verdict(MailHealthStatus::Ok, ""));
    let client = client(state);

    let (status, body) = client.get_json("/api/health/mail").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "Ok");
    assert!(body["checkedAt"].is_string());
    assert_eq!(body["detail"], "");
}

/// A failure carries its reason, and the plain health route still answers `OK`: the process
/// is up, only its mail is broken. Conflating the two would let a mail problem hold the site
/// down, which NFR-5 forbids.
#[test_log::test(tokio::test)]
async fn ein_fehler_bleibt_auf_der_mailroute() {
    let state = MailHealthState::new();
    state.write_verdict(verdict(MailHealthStatus::Failed, "Verbindung abgelehnt"));
    let client = client(state);

    let (mail_status, mail_body) = client.get_json("/api/health/mail").await;
    let (health_status, health_body) = client.get_json("/api/health").await;

    assert_eq!(mail_status, StatusCode::OK);
    assert_eq!(mail_body["status"], "Failed");
    assert_eq!(mail_body["detail"], "Verbindung abgelehnt");
    assert_eq!(health_status, StatusCode::OK);
    assert_eq!(health_body, serde_json::json!("OK"));
}
