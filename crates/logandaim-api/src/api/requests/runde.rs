//! Inbound request of a Runde.
//!
//! The owner is never part of a body: it comes from the verified access token, and the use
//! case decides authorisation from the aggregate it loads (SPEC-konto rule 17).

use uuid::Uuid;

use logandaim_core::{domain::ParcoursId, service::RundeStarten};

/// Start a Runde at one active Parcours (AC-7).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundeStartenRequest {
    pub parcours_id: Uuid,
}

impl From<RundeStartenRequest> for RundeStarten {
    fn from(anfrage: RundeStartenRequest) -> Self {
        Self {
            parcours_id: ParcoursId(anfrage.parcours_id),
        }
    }
}
