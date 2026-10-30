//! Inbound request of one logged Schuss.
//!
//! The owner is never part of a body: it comes from the verified access token, and the use
//! case decides authorisation from the Runde it loads (SPEC-konto rule 17). `pfeil` is kept
//! as a plain integer here so `domain::NeuerSchuss` performs the one validation.

use error_stack::Report;
use uuid::Uuid;

use logandaim_core::{
    domain::{RundeId, ZielId},
    service::{SchussRegistrieren, SchussRegistrierenFehler},
};

use crate::schema::Wertungszone;

/// Register one Schuss inside the caller's own open Runde.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchussErstellungRequest {
    /// The Ziel that was aimed at; it must be part of the Runde's snapshot.
    pub ziel_id: Uuid,
    /// 1-based position of the arrow at its Ziel; `0` is rejected.
    pub pfeil: usize,
    /// Where on the Ziel the arrow landed.
    pub wertungszone: Wertungszone,
}

impl SchussErstellungRequest {
    /// The Runde comes from the path, never from the body: one endpoint, one subject.
    pub fn in_use_case(
        self,
        runde_id: RundeId,
    ) -> Result<SchussRegistrieren, Report<SchussRegistrierenFehler>> {
        Ok(SchussRegistrieren {
            runde_id,
            ziel_id: ZielId(self.ziel_id),
            pfeil: self.pfeil,
            wertungszone: self.wertungszone.into(),
        })
    }
}
