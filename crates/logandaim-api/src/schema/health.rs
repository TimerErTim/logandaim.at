//! What the last probe of the mail relay found.
//!
//! It describes the *last* probe rather than a live connection: a request must never wait
//! on the relay (NFR-5).

use chrono::{DateTime, Utc};

/// The mail relay's last probe verdict, tagged by `status`.
///
/// Not a boolean: "not probed yet" and "probed and broken" call for different operator
/// action.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Union))]
#[cfg_attr(feature = "openapi", oai(discriminator_name = "status"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "status")]
pub enum MailHealth {
    #[cfg_attr(feature = "openapi", oai(mapping = "Unknown"))]
    Unknown(MailHealthUnknown),
    #[cfg_attr(feature = "openapi", oai(mapping = "Ok"))]
    Ok(MailHealthOk),
    #[cfg_attr(feature = "openapi", oai(mapping = "Failed"))]
    Failed(MailHealthFailed),
}

/// No probe has reported yet.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHealthUnknown {
    /// When a probe last reported, in UTC, if one ever has.
    pub checked_at: Option<DateTime<Utc>>,
}

/// The relay accepted the connection, the TLS upgrade, and the login.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHealthOk {
    /// When the probe reported, in UTC.
    pub checked_at: DateTime<Utc>,
}

/// The last probe failed. Sending will fail too.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MailHealthFailed {
    /// When the probe reported, in UTC.
    pub checked_at: DateTime<Utc>,
    /// What failed: credentials, TLS, or reachability.
    pub detail: String,
}

impl Default for MailHealth {
    fn default() -> Self {
        Self::Unknown(MailHealthUnknown::default())
    }
}

impl MailHealth {
    pub fn checked_at(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Unknown(unknown) => unknown.checked_at,
            Self::Ok(ok) => Some(ok.checked_at),
            Self::Failed(failed) => Some(failed.checked_at),
        }
    }
}
