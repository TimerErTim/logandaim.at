use uuid::Uuid;

use logandaim_core::domain::{self, Wertungsergebnis};

/// API mirror of [`domain::Wertungszone`]. Kept separate so the domain never learns
/// about OpenAPI, serde, or HTTP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Wertungszone {
    Spot,
    Kill,
    Body,
    Miss,
}

impl From<domain::Wertungszone> for Wertungszone {
    fn from(zone: domain::Wertungszone) -> Self {
        match zone {
            domain::Wertungszone::Spot => Self::Spot,
            domain::Wertungszone::Kill => Self::Kill,
            domain::Wertungszone::Body => Self::Body,
            domain::Wertungszone::Miss => Self::Miss,
        }
    }
}

impl From<Wertungszone> for domain::Wertungszone {
    fn from(zone: Wertungszone) -> Self {
        match zone {
            Wertungszone::Spot => Self::Spot,
            Wertungszone::Kill => Self::Kill,
            Wertungszone::Body => Self::Body,
            Wertungszone::Miss => Self::Miss,
        }
    }
}

/// One logged arrow, translated from the domain.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Schuss {
    pub id: Uuid,
    pub ziel_id: Uuid,
    pub pfeil: usize,
    pub wertungszone: Wertungszone,
}

/// Result of a Wertung calculation: the two Wertungen plus first-arrow hit info.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Wertung {
    pub erster_treffer: u32,
    pub mehrschuss: u32,
    pub erster_pfeil_getroffen: bool,
    pub anzahl_schüsse: usize,
}

pub fn schema_schuss_from_domain(schuss: domain::Schuss) -> Schuss {
    Schuss {
        id: schuss.id().0,
        ziel_id: schuss.ziel_id().0,
        pfeil: schuss.pfeil().get(),
        wertungszone: schuss.wertungszone().into(),
    }
}

pub fn schema_wertung_from_domain(ergebnis: Wertungsergebnis) -> Wertung {
    Wertung {
        erster_treffer: ergebnis.erster_treffer.value(),
        mehrschuss: ergebnis.mehrschuss.value(),
        erster_pfeil_getroffen: ergebnis.erster_pfeil_getroffen,
        anzahl_schüsse: ergebnis.anzahl_schüsse,
    }
}
