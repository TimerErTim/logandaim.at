use uuid::Uuid;

use logandaim_core::{
    domain::{SchussId, ZielId},
    service::{SchussRegistrieren, WertungBerechnen},
};

use crate::schema::Wertungszone;

/// Inbound request for registering one Schuss at a Ziel.
///
/// Pure transport DTO. `pfeil` is validated by `domain::NeuerSchuss` when the use case
/// builds it, so the rule stays in one place.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SchussErstellung {
    pub ziel_id: Uuid,
    pub pfeil: usize,
    pub wertungszone: Wertungszone,
}

/// Inbound request for summarizing a selection of Schüsse.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct WertungsAnfrage {
    pub schuss_ids: Vec<Uuid>,
}

impl From<SchussErstellung> for SchussRegistrieren {
    fn from(erstellung: SchussErstellung) -> Self {
        SchussRegistrieren {
            ziel_id: ZielId(erstellung.ziel_id),
            pfeil: erstellung.pfeil,
            wertungszone: erstellung.wertungszone.into(),
        }
    }
}

impl From<WertungsAnfrage> for WertungBerechnen {
    fn from(anfrage: WertungsAnfrage) -> Self {
        WertungBerechnen {
            schuss_ids: anfrage.schuss_ids.into_iter().map(SchussId).collect(),
        }
    }
}
