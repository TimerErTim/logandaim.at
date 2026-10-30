//! Outbound schemas of one logged Schuss and the per-Ziel Wertung folded from a set of
//! them.
//!
//! Scoring lives in the domain (`ZielSchüsse`, `Zielwertung`); this file only mirrors it.
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
#[cfg_attr(feature = "openapi", derive(poem_openapi::Union))]
#[cfg_attr(feature = "openapi", oai(externally_tagged))]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum Erfasser {
    /// The Benutzer that owns the row.
    Benutzer(BenutzerErfasser),
    /// The row was anonymised; only the time remains.
    Anonymisiert(AnonymisierterErfasser),
}

/// A row still owned by a Benutzer.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenutzerErfasser {
    /// Identity of the owning Benutzer.
    pub benutzer_id: Uuid,
}

/// A row whose owner was deleted.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnonymisierterErfasser {
    /// When the row was anonymised.
    pub am: chrono::DateTime<chrono::Utc>,
}

impl From<domain::Erfasser> for Erfasser {
    fn from(erfasser: domain::Erfasser) -> Self {
        match erfasser {
            domain::Erfasser::Benutzer(id) => {
                Self::Benutzer(BenutzerErfasser { benutzer_id: id.0 })
            }
            domain::Erfasser::Anonymisiert { am } => {
                Self::Anonymisiert(AnonymisierterErfasser { am })
            }
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
    pub erfasser: Erfasser,
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
pub struct Zielwertung {
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
            erfasser: schuss.erfasser().into(),
            pfeil: schuss.pfeil().get(),
            wertungszone: schuss.wertungszone().into(),
        }
    }
}

impl From<domain::Zielwertung> for Zielwertung {
    fn from(wertung: domain::Zielwertung) -> Self {
        Self {
            erster_treffer: wertung.erster_treffer(),
            mehrschuss: wertung.mehrschuss(),
            erster_pfeil_getroffen: wertung.erster_pfeil_getroffen(),
            anzahl_schüsse: wertung.anzahl_schüsse(),
        }
    }
}
