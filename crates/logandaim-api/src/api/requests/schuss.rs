//! Inbound request of one logged Schuss.
//!
//! The owner is never part of a body: it comes from the verified access token, and the use
//! case decides authorisation from the Runde it loads (SPEC-konto rule 17). `pfeil` is kept
//! as a plain integer here so `domain::NeuerSchuss` performs the one validation.

use error_stack::Report;
use uuid::Uuid;

use logandaim_core::{
    domain::{RundeId, SchussZielId, Schussposition},
    service::{SchussRegistrieren, SchussRegistrierenFehler, SchussÄndern, SchussÄndernFehler},
};

use crate::schema::Wertungszone;

/// Position supplied for a logged arrow.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Union))]
#[cfg_attr(feature = "openapi", oai(externally_tagged))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SchusspositionRequest {
    Distanz(DistanzPositionRequest),
    Pflock(PflockPositionRequest),
}

/// A typed distance position request, in whole metres.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DistanzPositionRequest {
    /// Positive whole metres from the Ziel.
    pub meter: u32,
}

/// A position request naming one configured Pflock.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PflockPositionRequest {
    /// The configured Pflock's colour.
    pub farbe: crate::schema::Farbe,
}

fn position_aus_anfrage(position: SchusspositionRequest) -> Result<Schussposition, ()> {
    match position {
        SchusspositionRequest::Distanz(request) => Ok(Schussposition::Distanz(
            logandaim_core::domain::Abstand::neu(request.meter).map_err(|_| ())?,
        )),
        SchusspositionRequest::Pflock(request) => Ok(Schussposition::Pflock(
            logandaim_core::domain::Farbe::neu(request.farbe.as_str()).map_err(|_| ())?,
        )),
    }
}

/// Register one Schuss inside the caller's own open Runde.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchussErstellungRequest {
    /// The neutral round-local Ziel identity.
    pub ziel_id: Uuid,
    /// Where on the Ziel the arrow landed.
    pub wertungszone: Wertungszone,
    pub position: Option<SchusspositionRequest>,
}

impl SchussErstellungRequest {
    /// The Runde comes from the path, never from the body: one endpoint, one subject.
    pub fn in_use_case(
        self,
        runde_id: RundeId,
    ) -> Result<SchussRegistrieren, Report<SchussRegistrierenFehler>> {
        Ok(SchussRegistrieren {
            runde_id,
            ziel_id: SchussZielId(self.ziel_id),
            wertungszone: self.wertungszone.into(),
            position: self
                .position
                .map(|position| {
                    position_aus_anfrage(position)
                        .map_err(|_| Report::new(SchussRegistrierenFehler::UngültigeAnfrage))
                })
                .transpose()?,
        })
    }
}

/// Replace a Schuss's scoring zone and optional position.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchussÄnderungRequest {
    /// Where on the Ziel the arrow landed.
    pub wertungszone: Wertungszone,
    /// Optional shot position; null clears the stored position.
    pub position: Option<SchusspositionRequest>,
}

impl SchussÄnderungRequest {
    pub fn in_use_case(
        self,
        runde_id: RundeId,
        schuss_id: logandaim_core::domain::SchussId,
    ) -> Result<SchussÄndern, Report<SchussÄndernFehler>> {
        let position = self
            .position
            .map(|position| {
                position_aus_anfrage(position)
                    .map_err(|_| Report::new(SchussÄndernFehler::UngültigeAnfrage))
            })
            .transpose()?;
        Ok(SchussÄndern {
            runde_id,
            schuss_id,
            wertungszone: self.wertungszone.into(),
            position,
        })
    }
}
