//! Outbound schemas of one logged Schuss and the per-Ziel Wertung folded from a set of
//! them.
//!
//! Scoring lives in the domain (`Schussmenge`, `Zielergebnis`); this file only mirrors it.
//! There is no endpoint that computes a Wertung on demand: a Runde read serves both
//! Wertungen (SPEC-wertung rule 6).

use uuid::Uuid;

use logandaim_core::domain;

/// API mirror of the domain's scoring zone.
///
/// `Spot` and `Kill` are hits, `Body` a non-lethal hit, `Miss` a miss.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[derive(serde::Serialize, serde::Deserialize)]
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

/// Who a row belongs to.
///
/// A Benutzer deletion anonymises its rows: the identity is gone and only the anonymisation
/// time remains, so history survives without naming anyone.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zuordnung {
    /// The owning Benutzer; absent once the row is anonymised.
    pub benutzer_id: Option<Uuid>,
    /// When the row was anonymised, as an RFC 3339 timestamp; absent while it is owned.
    pub anonymisiert_am: Option<String>,
}

impl From<domain::Zuordnung> for Zuordnung {
    fn from(zuordnung: domain::Zuordnung) -> Self {
        Self {
            benutzer_id: zuordnung.benutzer().map(|id| id.0),
            anonymisiert_am: zuordnung.anonymisiert_am().map(|am| am.to_rfc3339()),
        }
    }
}

/// One logged arrow.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schuss {
    /// Identity of the Schuss.
    pub id: Uuid,
    /// The Runde it belongs to.
    pub runde_id: Uuid,
    /// The Ziel it hit or missed.
    pub ziel_id: Uuid,
    /// Who logged it.
    pub zuordnung: Zuordnung,
    /// 1-based position of the arrow at its Ziel.
    pub pfeil: usize,
    /// Where on the Ziel it landed.
    pub wertungszone: Wertungszone,
}

/// The fold of every Schuss at one Ziel: both Wertungen plus the first-arrow hit.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zielergebnis {
    /// First-arrow hits at this Ziel.
    pub erster_treffer: u32,
    /// Follow-up hits at this Ziel.
    pub mehrschuss: u32,
    /// Whether the very first arrow hit.
    pub erster_pfeil_getroffen: bool,
    /// How many Schüsse were logged at this Ziel.
    pub anzahl_schüsse: usize,
}

impl From<domain::Schuss> for Schuss {
    fn from(schuss: domain::Schuss) -> Self {
        Self {
            id: schuss.id().0,
            runde_id: schuss.runde_id().0,
            ziel_id: schuss.ziel_id().0,
            zuordnung: schuss.zuordnung().into(),
            pfeil: schuss.pfeil().get(),
            wertungszone: schuss.wertungszone().into(),
        }
    }
}

impl From<domain::Zielergebnis> for Zielergebnis {
    fn from(ergebnis: domain::Zielergebnis) -> Self {
        Self {
            erster_treffer: ergebnis.erster_treffer().value(),
            mehrschuss: ergebnis.mehrschuss().value(),
            erster_pfeil_getroffen: ergebnis.erster_pfeil_getroffen(),
            anzahl_schüsse: ergebnis.anzahl_schüsse(),
        }
    }
}
