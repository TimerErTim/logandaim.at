//! Inbound request of a Runde.
//!
//! The owner is never part of a body: it comes from the verified access token, and the use
//! case decides authorisation from the aggregate it loads (SPEC-konto rule 17).

use chrono::NaiveDate;
use uuid::Uuid;

use logandaim_core::{
    domain::ParkourId,
    service::{RundeDatumÄndern, RundeStarten},
};

/// Start a Runde on one active Parkour.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundeStartenRequest {
    /// The Parkour to shoot. It must be active.
    pub parkour_id: Uuid,
    /// Optional calendar date; defaults to the current date.
    pub datum: Option<NaiveDate>,
}

impl From<RundeStartenRequest> for RundeStarten {
    fn from(anfrage: RundeStartenRequest) -> Self {
        Self {
            parkour_id: ParkourId(anfrage.parkour_id),
            datum: anfrage.datum,
        }
    }
}

/// Set a Runde's calendar date without changing its captured route or shots.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundeDatumRequest {
    /// Calendar date of the Runde.
    pub datum: NaiveDate,
}

impl RundeDatumRequest {
    pub fn in_use_case(self, runde_id: logandaim_core::domain::RundeId) -> RundeDatumÄndern {
        RundeDatumÄndern {
            runde_id,
            datum: self.datum,
        }
    }
}
