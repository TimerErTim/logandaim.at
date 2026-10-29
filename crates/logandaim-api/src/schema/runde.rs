//! Outbound schemas of a Runde: the captured snapshot, its per-Ziel results, and the
//! totals.
//!
//! A Runde read is the only place a Wertung is served: there is no `/wertung` endpoint
//! (SPEC-wertung rule 6). The snapshot fields are copied verbatim from what was captured,
//! never re-read from master data (AC-7, AC-9).

use uuid::Uuid;

use logandaim_core::{
    domain,
    service::{RundeAnsicht, Zielansicht},
};

use crate::schema::{Zielergebnis, Zuordnung};

/// One captured Pflock, in far-to-near order.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotPflock {
    pub bezeichnung: String,
    /// Caller data ("Rot", "Blau"), never a design token.
    pub farbe: String,
    /// 0-based, far to near.
    pub position: usize,
}

/// One Pflock label with its captured distance.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotAbstand {
    pub pflock: String,
    pub meter: u32,
}

/// One captured Ziel: identity, animal name, target group, distances as they were.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotZiel {
    pub ziel_id: Uuid,
    pub tiername: String,
    pub zielgruppe: u8,
    pub abstände: Vec<SnapshotAbstand>,
    /// Absent for a Ziel without any Schuss (AC-6 of SPEC-wertung).
    pub ergebnis: Option<Zielergebnis>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum RundeZustand {
    InBearbeitung,
    Abgeschlossen,
}

/// `GET /runden/{id}`: the whole read model of one Runde.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runde {
    pub id: Uuid,
    pub anlage_id: Uuid,
    pub parcours_id: Uuid,
    /// As captured at the start, not the Parcours' current name (AC-9).
    pub parcours_name: String,
    pub zuordnung: Zuordnung,
    pub zustand: RundeZustand,
    pub begonnen_am: String,
    pub abgeschlossen_am: Option<String>,
    pub pflöcke: Vec<SnapshotPflock>,
    pub ziele: Vec<SnapshotZiel>,
    pub gesamt_erster_treffer: u32,
    pub gesamt_mehrschuss: u32,
    pub ziele_mit_ergebnis: usize,
}

/// One row of `GET /runden`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundenZeile {
    pub id: Uuid,
    pub anlage_id: Uuid,
    pub parcours_id: Uuid,
    pub parcours_name: String,
    pub zustand: RundeZustand,
    pub begonnen_am: String,
    pub abgeschlossen_am: Option<String>,
}

// ------------------------------------------------------------------ conversions

fn zeitpunkt(wert: chrono::DateTime<chrono::Utc>) -> String {
    wert.to_rfc3339()
}

impl From<domain::RundeZustand> for RundeZustand {
    fn from(zustand: domain::RundeZustand) -> Self {
        match zustand {
            domain::RundeZustand::InBearbeitung => Self::InBearbeitung,
            domain::RundeZustand::Abgeschlossen => Self::Abgeschlossen,
        }
    }
}

impl From<domain::SnapshotPflock> for SnapshotPflock {
    fn from(pflock: domain::SnapshotPflock) -> Self {
        Self {
            bezeichnung: pflock.bezeichnung().to_owned(),
            farbe: pflock.farbe().to_owned(),
            position: pflock.position(),
        }
    }
}

impl From<Zielansicht> for SnapshotZiel {
    fn from(ansicht: Zielansicht) -> Self {
        Self {
            ziel_id: ansicht.ziel_id.0,
            tiername: ansicht.tiername,
            zielgruppe: ansicht.zielgruppe,
            abstände: ansicht
                .abstände
                .into_iter()
                .map(|(pflock, meter)| SnapshotAbstand { pflock, meter })
                .collect(),
            ergebnis: ansicht.ergebnis.map(Into::into),
        }
    }
}

impl From<RundeAnsicht> for Runde {
    fn from(ansicht: RundeAnsicht) -> Self {
        let runde = &ansicht.runde;
        Self {
            id: runde.id().0,
            anlage_id: runde.anlage_id().0,
            parcours_id: runde.parcours_id().0,
            parcours_name: runde.snapshot().parcours_name().to_owned(),
            zuordnung: runde.zuordnung().into(),
            zustand: runde.zustand().into(),
            begonnen_am: zeitpunkt(runde.begonnen_am()),
            abgeschlossen_am: runde.abgeschlossen_am().map(zeitpunkt),
            pflöcke: runde
                .snapshot()
                .pflöcke()
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
            ziele: ansicht.ziele.into_iter().map(Into::into).collect(),
            gesamt_erster_treffer: ansicht.gesamt_erster_treffer,
            gesamt_mehrschuss: ansicht.gesamt_mehrschuss,
            ziele_mit_ergebnis: ansicht.ziele_mit_ergebnis,
        }
    }
}

/// A freshly started or freshly closed Runde, before any Wertung is folded.
///
/// `RundeStarten` and `RundeAbschliessen` answer the aggregate, not a read model, so this
/// impl renders the snapshot exactly and leaves every Ergebnis empty.
impl From<domain::Runde> for Runde {
    fn from(runde: domain::Runde) -> Self {
        let ziele = runde
            .snapshot()
            .ziele()
            .iter()
            .map(|ziel| SnapshotZiel {
                ziel_id: ziel.ziel_id().0,
                tiername: ziel.tiername().to_owned(),
                zielgruppe: ziel.zielgruppe(),
                abstände: ziel
                    .abstände()
                    .iter()
                    .map(|(pflock, meter)| SnapshotAbstand {
                        pflock: pflock.clone(),
                        meter: *meter,
                    })
                    .collect(),
                ergebnis: None,
            })
            .collect();
        Self {
            id: runde.id().0,
            anlage_id: runde.anlage_id().0,
            parcours_id: runde.parcours_id().0,
            parcours_name: runde.snapshot().parcours_name().to_owned(),
            zuordnung: runde.zuordnung().into(),
            zustand: runde.zustand().into(),
            begonnen_am: zeitpunkt(runde.begonnen_am()),
            abgeschlossen_am: runde.abgeschlossen_am().map(zeitpunkt),
            pflöcke: runde
                .snapshot()
                .pflöcke()
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
            ziele,
            gesamt_erster_treffer: 0,
            gesamt_mehrschuss: 0,
            ziele_mit_ergebnis: 0,
        }
    }
}

impl From<domain::Runde> for RundenZeile {
    fn from(runde: domain::Runde) -> Self {
        Self {
            id: runde.id().0,
            anlage_id: runde.anlage_id().0,
            parcours_id: runde.parcours_id().0,
            parcours_name: runde.snapshot().parcours_name().to_owned(),
            zustand: runde.zustand().into(),
            begonnen_am: zeitpunkt(runde.begonnen_am()),
            abgeschlossen_am: runde.abgeschlossen_am().map(zeitpunkt),
        }
    }
}
