//! The verdict of the background probe that watches the mail relay.
//!
//! Mail is part of the MVP, so a deployment wants to know whether the relay works without
//! sending a message. That answer lives here: one small object, produced by a probe loop
//! and read by a route. It deliberately describes the *last* probe rather than a live
//! connection — a request must never wait on the relay (NFR-5).

use chrono::{DateTime, Utc};

/// What the last probe of the mail relay found.
///
/// `Unknown` is the honest answer before any probe has finished, and it is why this is not
/// a boolean: "not probed yet" and "probed and broken" call for different operator action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum MailHealthStatus {
    /// No probe has reported yet: the first one is still running, or none is configured.
    Unknown,
    /// The relay accepted the connection, the TLS upgrade, and the login.
    Ok,
    /// The last probe failed. Sending will fail too.
    Failed,
}

/// The mail relay's last probe verdict.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHealth {
    /// The verdict. Only `ok` means a send would work right now.
    pub status: MailHealthStatus,
    /// When the probe last reported, in UTC. Absent until one has.
    pub checked_at: Option<DateTime<Utc>>,
    /// One German sentence naming what failed; empty while `status` is `ok`.
    pub detail: String,
}

impl Default for MailHealth {
    fn default() -> Self {
        Self {
            status: MailHealthStatus::Unknown,
            checked_at: None,
            detail: "Noch kein Prüfergebnis".to_owned(),
        }
    }
}
