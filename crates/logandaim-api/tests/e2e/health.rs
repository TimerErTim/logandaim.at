//! The two health routes, and the contract that keeps them apart.
//!
//! `GET /api/health` answers the literal `OK`, and three consumers parse exactly that: the
//! Compose healthcheck, `tasks/verify.toml`, and `deploy-vps-remote.sh`. The mail verdict
//! therefore lives on its own route.

use std::sync::Arc;

use async_trait::async_trait;
use logandaim_core::{
    App,
    adapters::NullMailSender,
    application::IdentityProviderConfiguration,
    ports::{MailSender, RepositoryResult},
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};
use logandaim_fakes::FakeMailSender;
use poem::http::StatusCode;
use serde_json::Value;

use super::LogandaimApiTestClient;

/// The health routes need only the shape of the app: no repository is called.
struct Unimplemented;

#[async_trait]
impl UnitOfWorkProvider for Unimplemented {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>> {
        unimplemented!()
    }
}

fn client(mail_sender: Arc<dyn MailSender>) -> LogandaimApiTestClient {
    LogandaimApiTestClient::new(
        App::builder()
            .uow_provider(Box::new(Unimplemented))
            .mail_sender(mail_sender)
            .konto(IdentityProviderConfiguration::neu(
                super::ISSUER,
                super::AUDIENCE,
            ))
            .build(),
    )
}

/// Poll the mail route until the background probe has reported.
async fn verdict(client: &LogandaimApiTestClient) -> Value {
    for _ in 0..100 {
        let (status, body) = client.get_json("/api/health/mail").await;
        assert_eq!(status, StatusCode::OK);
        if body["status"] != "Unknown" {
            return body;
        }
        tokio::task::yield_now().await;
    }
    panic!("the mail probe never reported");
}

/// The literal body the three external parsers match on. Changing this is a deployment
/// change, not a cosmetic one.
#[test_log::test(tokio::test)]
async fn gesundheit_antwortet_den_literal_ok() {
    let client = client(Arc::new(FakeMailSender::new()));

    let (status, body) = client.get_json("/api/health").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, serde_json::json!("OK"));
}

/// The first read answers before any probe has run: `Unknown`, never an untested `Ok`.
#[test_log::test(tokio::test)]
async fn ohne_prüfergebnis_meldet_die_mailroute_unbekannt() {
    let client = client(Arc::new(FakeMailSender::new()));

    let (status, body) = client.get_json("/api/health/mail").await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "Unknown");
    assert_eq!(body["checkedAt"], Value::Null);
}

#[test_log::test(tokio::test)]
async fn ein_erreichbares_relay_meldet_ok() {
    let client = client(Arc::new(FakeMailSender::new()));

    let body = verdict(&client).await;

    assert_eq!(body["status"], "Ok");
    assert!(body["checkedAt"].is_string());
}

/// A failure carries its reason, and the plain health route still answers `OK`: the process
/// is up, only its mail is broken (NFR-5).
#[test_log::test(tokio::test)]
async fn ein_fehler_bleibt_auf_der_mailroute() {
    let client = client(Arc::new(NullMailSender));

    let body = verdict(&client).await;
    let (health_status, health_body) = client.get_json("/api/health").await;

    assert_eq!(body["status"], "Failed");
    assert!(body["checkedAt"].is_string());
    assert_eq!(body["detail"], "der Mailversand ist nicht konfiguriert");
    assert_eq!(health_status, StatusCode::OK);
    assert_eq!(health_body, serde_json::json!("OK"));
}
