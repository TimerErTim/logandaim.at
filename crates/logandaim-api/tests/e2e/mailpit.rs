//! Read a confirmation or recovery token out of Mailpit's HTTP API.
//!
//! The e2e suite goes through a real SMTP relay for AC-19 — that is the point of the case:
//! the mail must survive the adapter, the transport, and a real server. Everything else uses
//! the in-memory fake, because a suite where every case needs a daemon is a suite that gets
//! skipped.
//!
//! A missing Mailpit is a **failure with instructions**, never a skip: a silently skipped
//! AC-19 is the exact failure this module exists to prevent.

use std::time::Duration;

use serde_json::Value;

/// Where Mailpit listens; overridable so CI and a local daemon can differ.
fn basis_url() -> String {
    std::env::var("LOGANDAIM_MAILPIT_URL").unwrap_or_else(|_| "http://127.0.0.1:8025".to_owned())
}

/// Whether Mailpit answers at all.
pub async fn erreichbar() -> bool {
    reqwest::Client::new()
        .get(format!("{}/api/v1/info", basis_url()))
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .map(|resp| resp.status().is_success())
        .unwrap_or(false)
}

/// Panic with an actionable message when Mailpit is not running.
pub async fn verlange_mailpit() {
    assert!(
        erreichbar().await,
        "Mailpit ist unter {} nicht erreichbar. Starte ihn mit `mise daemons start mailpit`.",
        basis_url()
    );
}

/// Every message in the mailbox, oldest first.
pub async fn nachrichten() -> Vec<Value> {
    let antwort = reqwest::Client::new()
        .get(format!("{}/api/v1/messages", basis_url()))
        .send()
        .await
        .unwrap_or_else(|fehler| {
            panic!(
                "Mailpit ist unter {} nicht erreichbar ({fehler}). Starte ihn mit \
                 `mise daemons start mailpit`.",
                basis_url()
            )
        })
        .json::<Value>()
        .await
        .expect("Mailpit antwortet mit JSON");

    antwort["messages"].as_array().cloned().unwrap_or_default()
}

/// The body of the newest message addressed to `empfänger`, decoded from Mailpit's
/// base64-free `Text` field.
pub async fn letzter_text_an(empfänger: &str) -> String {
    let nachrichten = nachrichten().await;
    let passend = nachrichten
        .iter()
        .rev()
        .find(|nachricht| {
            nachricht["To"]
                .as_array()
                .map(|empfänger_liste| {
                    empfänger_liste.iter().any(|eintrag| {
                        eintrag["Address"]
                            .as_str()
                            .map(|adresse| adresse.eq_ignore_ascii_case(empfänger))
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false)
        })
        .unwrap_or_else(|| {
            panic!(
                "keine Mail an {empfänger} in Mailpit; vorhanden: {}",
                nachrichten.len()
            )
        });

    let id = passend["ID"].as_str().expect("jede Nachricht hat eine ID");
    let detail = reqwest::Client::new()
        .get(format!("{}/api/v1/message/{id}", basis_url()))
        .send()
        .await
        .expect("Mailpit liefert die Nachricht")
        .json::<Value>()
        .await
        .expect("Mailpit antwortet mit JSON");

    detail["Text"]
        .as_str()
        .unwrap_or_else(|| detail["HTML"].as_str().unwrap_or_default())
        .to_owned()
}

/// Forget every message, so a case asserts on its own mail only.
pub async fn leeren() {
    reqwest::Client::new()
        .delete(format!("{}/api/v1/messages", basis_url()))
        .send()
        .await
        .expect("Mailpit leert die Mailbox");
}

pub use super::token_aus;
