use uuid::Uuid;

use logandaim_core::{
    domain::{SchussId, ZielId},
    service::{SchussRegistrieren, WertungBerechnen},
};

use crate::schema::Wertungszone;

/// Inbound request for registering one Schuss at a Ziel. `pfeil` is validated by
/// `domain::NeuerSchuss` when the use case builds it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchussErstellungRequest {
    pub ziel_id: Uuid,
    pub pfeil: usize,
    pub wertungszone: Wertungszone,
}

/// Inbound request for summarizing a selection of Schüsse.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WertungRequest {
    pub schuss_ids: Vec<Uuid>,
}

impl From<SchussErstellungRequest> for SchussRegistrieren {
    fn from(erstellung: SchussErstellungRequest) -> Self {
        SchussRegistrieren {
            ziel_id: ZielId(erstellung.ziel_id),
            pfeil: erstellung.pfeil,
            wertungszone: erstellung.wertungszone.into(),
        }
    }
}

impl From<WertungRequest> for WertungBerechnen {
    fn from(anfrage: WertungRequest) -> Self {
        WertungBerechnen {
            schuss_ids: anfrage.schuss_ids.into_iter().map(SchussId).collect(),
        }
    }
}
