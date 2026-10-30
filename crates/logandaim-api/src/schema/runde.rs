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

/// One Pflock captured in the snapshot, in far-to-near order.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotPflock {
    /// The Pflock's colour as `#rrggbb`.
    pub farbe: String,
    /// 0-based position, far to near.
    pub position: usize,
}

/// One Pflock colour with its captured distance.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotAbstand {
    /// The captured Pflock's colour as `#rrggbb`.
    pub pflock: String,
    /// Distance from that Pflock to the Ziel, in metres.
    pub meter: u32,
}

/// One captured Ziel: its identity, animal name, target group, and distances as recorded.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotZiel {
    /// Identity of the Ziel as captured.
    pub ziel_id: Uuid,
    /// Animal name as captured.
    pub tiername: String,
    /// Target group as captured.
    pub zielgruppe: u8,
    /// The distances as captured.
    pub abstände: Vec<SnapshotAbstand>,
    /// Both Wertungen of this Ziel; absent while no Schuss has been logged at it.
    pub ergebnis: Option<Zielergebnis>,
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
    /// The Anlage the Parcours belongs to.
    pub anlage_id: Uuid,
    /// The Parcours this Runde was started on.
    pub parcours_id: Uuid,
    /// The Parcours' name as captured at the start, not its current name.
    pub parcours_name: String,
    /// Who the Runde belongs to.
    pub zuordnung: Zuordnung,
    /// Whether it is still open or already closed.
    pub zustand: RundeZustand,
    /// Start of the Runde, as an RFC 3339 timestamp.
    pub begonnen_am: String,
    /// End of the Runde, as an RFC 3339 timestamp; absent while it is open.
    pub abgeschlossen_am: Option<String>,
    /// The Pflöcke as captured, far to near.
    pub pflöcke: Vec<SnapshotPflock>,
    /// The Ziele as captured, with their results.
    pub ziele: Vec<SnapshotZiel>,
    /// First-arrow hits over the whole Runde.
    pub gesamt_erster_treffer: u32,
    /// Follow-up hits over the whole Runde.
    pub gesamt_mehrschuss: u32,
    /// How many Ziele already carry a result.
    pub ziele_mit_ergebnis: usize,
}

/// One row of `GET /runden`.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundenZeile {
    /// Identity of the Runde.
    pub id: Uuid,
    /// The Anlage the Parcours belongs to.
    pub anlage_id: Uuid,
    /// The Parcours this Runde was started on.
    pub parcours_id: Uuid,
    /// The Parcours' name as captured at the start.
    pub parcours_name: String,
    /// Whether it is still open or already closed.
    pub zustand: RundeZustand,
    /// Start of the Runde, as an RFC 3339 timestamp.
    pub begonnen_am: String,
    /// End of the Runde, as an RFC 3339 timestamp; absent while it is open.
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
