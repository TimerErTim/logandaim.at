//! Outbound schemas of a Runde: the captured route, its per-Ziel results, and the totals.
//!
//! A Runde read is the only place a Wertung is served: there is no `/wertung` endpoint
//! (SPEC-wertung rule 6). The snapshot fields are copied verbatim from what was captured,
//! never re-read from master data (AC-7, AC-9).

use chrono::{DateTime, Utc};
use uuid::Uuid;

use logandaim_core::{
    domain,
    service::{RundeAnsicht, Zielansicht},
};

use crate::schema::{Erfasser, Farbe, Zielwertung};

/// One Pflock of this Runde' captured route, in near-to-far order.
///
/// A captured route keeps no master-data identity, so the colour is what tells the Pflöcke
/// apart.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundePflock {
    /// The captured Pflock's colour.
    pub farbe: Farbe,
    /// 0-based position, near to far.
    pub position: usize,
}

/// One captured distance, named by the Pflock' colour.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abstand {
    /// The captured Pflock's colour.
    pub pflock: Farbe,
    /// Distance from that Pflock to the Ziel, in metres.
    pub meter: u32,
}

/// One Ziel as this Runde captured it: identity, animal name, ordinal, distances, Wertung.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundeZiel {
    /// Identity of the Ziel as captured.
    pub ziel_id: Uuid,
    /// Animal name as captured.
    pub tiername: String,
    /// 1-based ordinal as captured, distinguishing duplicate animal names.
    pub zielnummer: u32,
    /// The distances as captured.
    pub abstände: Vec<Abstand>,
    /// The Wertung of this Ziel; absent while no Schuss has been logged at it.
    pub wertung: Option<Zielwertung>,
}

/// `InBearbeitung` or `Abgeschlossen`.
///
/// A completed Runde refuses further Schüsse and its Wertungen are final.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RundeZustand {
    InBearbeitung,
    Abgeschlossen,
}

/// The whole read model of one Runde, as `GET /runden/{id}` answers it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runde {
    /// Identity of the Runde.
    pub id: Uuid,
    /// The Anlage the Parkour belongs to.
    pub anlage_id: Uuid,
    /// The Parkour this Runde was started on.
    pub parkour_id: Uuid,
    /// The Parkour' name as captured at the start, not its current name.
    pub parkour_name: String,
    /// Who the Runde belongs to.
    pub erfasser: Erfasser,
    /// Whether it is still open or already closed.
    pub zustand: RundeZustand,
    /// Start of the Runde.
    pub begonnen_am: DateTime<Utc>,
    /// End of the Runde; absent while it is open.
    pub abgeschlossen_am: Option<DateTime<Utc>>,
    /// The captured Pflöcke' colours, near to far.
    pub pflöcke: Vec<RundePflock>,
    /// The Ziele as captured, with their results.
    pub ziele: Vec<RundeZiel>,
    /// First-arrow hits over the whole Runde.
    pub gesamt_erster_treffer: u32,
    /// Follow-up hits over the whole Runde.
    pub gesamt_mehrschuss: u32,
    /// How many Ziele already carry a Wertung.
    pub ziele_mit_wertung: usize,
}

/// A Runde without its captured route, for a list of many.
///
/// Named for what it is, not for the endpoint that serves it: the same shape answers any
/// collection of Runden.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundeBasic {
    /// Identity of the Runde.
    pub id: Uuid,
    /// The Anlage the Parkour belongs to.
    pub anlage_id: Uuid,
    /// The Parkour this Runde was started on.
    pub parkour_id: Uuid,
    /// The Parkour' name as captured at the start.
    pub parkour_name: String,
    /// Whether it is still open or already closed.
    pub zustand: RundeZustand,
    /// Start of the Runde.
    pub begonnen_am: DateTime<Utc>,
    /// End of the Runde; absent while it is open.
    pub abgeschlossen_am: Option<DateTime<Utc>>,
}

// ------------------------------------------------------------------ conversions

impl From<domain::RundeZustand> for RundeZustand {
    fn from(zustand: domain::RundeZustand) -> Self {
        match zustand {
            domain::RundeZustand::InBearbeitung => Self::InBearbeitung,
            domain::RundeZustand::Abgeschlossen => Self::Abgeschlossen,
        }
    }
}

impl From<Zielansicht> for RundeZiel {
    fn from(ansicht: Zielansicht) -> Self {
        Self {
            ziel_id: ansicht.ziel_id.0,
            tiername: ansicht.tiername.to_string(),
            zielnummer: ansicht.zielnummer.get(),
            abstände: ansicht
                .abstände
                .into_iter()
                .map(|(farbe, abstand)| Abstand {
                    pflock: farbe.into(),
                    meter: abstand.meter(),
                })
                .collect(),
            wertung: ansicht.wertung.map(Into::into),
        }
    }
}

impl From<RundeAnsicht> for Runde {
    fn from(ansicht: RundeAnsicht) -> Self {
        let runde = &ansicht.runde;
        Self {
            id: runde.id().0,
            anlage_id: runde.anlage_id().0,
            parkour_id: runde.parkour_id().0,
            parkour_name: runde.snapshot().parkour_name().to_string(),
            erfasser: runde.erfasser().into(),
            zustand: runde.zustand().into(),
            begonnen_am: runde.begonnen_am(),
            abgeschlossen_am: runde.abgeschlossen_am(),
            pflöcke: runde
                .snapshot()
                .pflöcke()
                .iter()
                .map(|pflock| RundePflock {
                    farbe: pflock.farbe().into(),
                    position: pflock.position(),
                })
                .collect(),
            ziele: ansicht.ziele.into_iter().map(Into::into).collect(),
            gesamt_erster_treffer: ansicht.gesamt_erster_treffer,
            gesamt_mehrschuss: ansicht.gesamt_mehrschuss,
            ziele_mit_wertung: ansicht.ziele_mit_wertung,
        }
    }
}

/// A freshly started or freshly closed Runde, before any Wertung is folded.
///
/// `RundeStarten` and `RundeAbschliessen` answer the aggregate, not a read model, so this
/// impl renders the captured route exactly and leaves every Wertung empty.
impl From<domain::Runde> for Runde {
    fn from(runde: domain::Runde) -> Self {
        let ziele = runde
            .snapshot()
            .ziele()
            .iter()
            .map(|ziel| RundeZiel {
                ziel_id: ziel.ziel_id().0,
                tiername: ziel.tiername().to_string(),
                zielnummer: ziel.zielnummer().get(),
                abstände: ziel
                    .abstände()
                    .iter()
                    .map(|(farbe, abstand)| Abstand {
                        pflock: (*farbe).into(),
                        meter: abstand.meter(),
                    })
                    .collect(),
                wertung: None,
            })
            .collect();
        Self {
            id: runde.id().0,
            anlage_id: runde.anlage_id().0,
            parkour_id: runde.parkour_id().0,
            parkour_name: runde.snapshot().parkour_name().to_string(),
            erfasser: runde.erfasser().into(),
            zustand: runde.zustand().into(),
            begonnen_am: runde.begonnen_am(),
            abgeschlossen_am: runde.abgeschlossen_am(),
            pflöcke: runde
                .snapshot()
                .pflöcke()
                .iter()
                .map(|pflock| RundePflock {
                    farbe: pflock.farbe().into(),
                    position: pflock.position(),
                })
                .collect(),
            ziele,
            gesamt_erster_treffer: 0,
            gesamt_mehrschuss: 0,
            ziele_mit_wertung: 0,
        }
    }
}

impl From<domain::Runde> for RundeBasic {
    fn from(runde: domain::Runde) -> Self {
        Self {
            id: runde.id().0,
            anlage_id: runde.anlage_id().0,
            parkour_id: runde.parkour_id().0,
            parkour_name: runde.snapshot().parkour_name().to_string(),
            zustand: runde.zustand().into(),
            begonnen_am: runde.begonnen_am(),
            abgeschlossen_am: runde.abgeschlossen_am(),
        }
    }
}
