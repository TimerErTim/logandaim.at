//! Outbound schemas of a Runde: the captured route, its per-Ziel results, and the totals.
//!
//! A Runde read is the only place a Wertung is served: there is no `/wertung` endpoint
//! (SPEC-wertung rule 6). The captured fields are copied verbatim from what was captured,
//! never re-read from master data (AC-7, AC-9).
//!
//! The setup is told by one field, `herkunft`: an object holding the whole Anlage/Parkour pair,
//! or absent for a `Freie Runde`. One optional object rather than two optional ids, so a client
//! cannot receive a pair that names two different Parkours, and `herkunft === null` is the
//! discriminator a capture screen branches on.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use logandaim_core::domain;

use crate::schema::{Erfasser, Farbe, Schuss, Zielwertung};

/// The master-data identities a Runde was started from, as one object.
///
/// Both ids or neither: the pair is what "this is a Parkour round" means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundeHerkunft {
    /// The Anlage the Parkour belongs to.
    pub anlage_id: Uuid,
    /// The Parkour this Runde was started on.
    pub parkour_id: Uuid,
}

impl From<(domain::AnlageId, domain::ParkourId)> for RundeHerkunft {
    fn from((anlage_id, parkour_id): (domain::AnlageId, domain::ParkourId)) -> Self {
        Self {
            anlage_id: anlage_id.0,
            parkour_id: parkour_id.0,
        }
    }
}

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

/// One target in this Runde's captured route: identity, name, ordinal, distances, and Wertung.
///
/// A free-round target fills the same shape; its Pflock and distance lists are empty because a
/// free round configures none.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RundeZiel {
    /// Identity of the target within this Runde.
    pub ziel_id: Uuid,
    /// Name captured in the route.
    pub tiername: String,
    /// 1-based route ordinal.
    pub zielnummer: u32,
    /// The Pflöcke that apply to this Ziel as captured, near to far: the Ziel's own set when
    /// it had one, the Parkour-wide set otherwise. Always empty on a free round.
    pub pflöcke: Vec<RundePflock>,
    /// The distances as captured. Always empty on a free round.
    pub abstände: Vec<Abstand>,
    /// The Wertung of this Ziel; absent while no Schuss has been logged at it.
    pub wertung: Option<Zielwertung>,
    /// The logged arrows at this Ziel, in Pfeil order.
    pub schüsse: Vec<Schuss>,
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
    /// The Anlage/Parkour pair; absent on a `Freie Runde`.
    pub herkunft: Option<RundeHerkunft>,
    /// The round's name: the captured Parkour's, or the free round's own.
    pub name: String,
    /// Who the Runde belongs to.
    pub erfasst_von: Erfasser,
    /// Whether it is still open or already closed.
    pub zustand: RundeZustand,
    /// Start of the Runde.
    pub begonnen_am: DateTime<Utc>,
    /// End of the Runde; absent while it is open.
    pub abgeschlossen_am: Option<DateTime<Utc>>,
    /// The captured Pflöcke' colours, near to far; empty on a free round.
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
    /// The Anlage/Parkour pair; absent on a `Freie Runde`.
    pub herkunft: Option<RundeHerkunft>,
    /// The round's name: the captured Parkour's, or the free round's own.
    pub name: String,
    /// Number of Ziele in the captured route.
    pub anzahl_ziele: usize,
    /// Number of Ziele with a recorded shot.
    pub anzahl_erfasste_ziele: usize,
    /// Whether it is still open or already closed.
    pub zustand: RundeZustand,
    /// Start of the Runde.
    pub begonnen_am: DateTime<Utc>,
    /// End of the Runde; absent while it is open.
    pub abgeschlossen_am: Option<DateTime<Utc>>,
}

// ------------------------------------------------------------------ conversions

/// The read model of a Runde with its Schüsse folded over the captured route.
///
/// The shape is the API's; the fold itself is the domain's ([`domain::Runde::ziel_wertungen`]
/// and [`domain::Runde::wertung`]), so no Wertung is re-derived at the edge.
impl Runde {
    pub fn gelesen(runde: domain::Runde, schüsse: Vec<domain::Schuss>) -> Self {
        let wertungen = runde.ziel_wertungen(schüsse.iter().cloned());
        let snapshot = runde.aufbau().snapshot();
        let pflöcke = snapshot.pflöcke();
        let ziele = snapshot.ziele();
        let ziele = ziele
            .into_iter()
            .zip(wertungen)
            .map(|(ziel, wertung)| RundeZiel {
                ziel_id: ziel.id().as_uuid(),
                tiername: ziel.name().to_string(),
                zielnummer: ziel.ordinal().get(),
                pflöcke: effective_pflöcke(ziel, pflöcke),
                abstände: ziel
                    .abstände()
                    .iter()
                    .map(|(farbe, abstand)| Abstand {
                        pflock: (*farbe).into(),
                        meter: abstand.meter(),
                    })
                    .collect(),
                wertung: wertung.map(Into::into),
                schüsse: schüsse
                    .iter()
                    .filter(|schuss| schuss.ziel_id() == ziel.id().as_schuss_ziel_id())
                    .cloned()
                    .map(Into::into)
                    .collect(),
            })
            .collect();

        let gesamt = runde.wertung(schüsse);
        let mut gerendert: Self = runde.into();
        gerendert.ziele = ziele;
        gerendert.gesamt_erster_treffer = gesamt.erster_treffer();
        gerendert.gesamt_mehrschuss = gesamt.mehrschuss();
        gerendert.ziele_mit_wertung = gesamt.ziele_mit_wertung();
        gerendert
    }
}

fn zustand(runde: &domain::Runde) -> RundeZustand {
    if runde.ist_in_bearbeitung() {
        RundeZustand::InBearbeitung
    } else {
        RundeZustand::Abgeschlossen
    }
}

/// The Pflöcke that apply to one captured Ziel, near to far: its own set when it was
/// captured with one, the Parkour-wide captured set otherwise.
///
/// A `Freies Ziel` captured no Pflock set and its round has no Parkour-wide set, so the answer
/// is empty — the two lists it could inherit from do not exist.
fn effective_pflöcke(
    ziel: &domain::RundenZiel,
    parkour: &[domain::PflockSnapshot],
) -> Vec<RundePflock> {
    // The captured snapshot already assigns each Pflock its position from the array order,
    // so either set renders as it was captured.
    ziel.pflöcke()
        .effektive(parkour)
        .iter()
        .map(|pflock| RundePflock {
            farbe: pflock.farbe().into(),
            position: pflock.position(),
        })
        .collect()
}

/// A freshly started or freshly closed Runde, before any Wertung is folded.
///
/// `RundeStarten` and `RundeAbschliessen` answer the aggregate, not a read model, so this
/// impl renders the captured route exactly and leaves every Wertung empty.
impl From<domain::Runde> for Runde {
    fn from(runde: domain::Runde) -> Self {
        let (herkunft, name) = match runde.aufbau() {
            domain::Rundenaufbau::Parkour {
                anlage_id,
                parkour_id,
                parkour_name,
                ..
            } => (
                Some(RundeHerkunft::from((*anlage_id, *parkour_id))),
                parkour_name.to_string(),
            ),
            domain::Rundenaufbau::Freie { name, .. } => (None, name.to_string()),
        };
        let snapshot = runde.aufbau().snapshot();
        let pflöcke = snapshot.pflöcke();
        let ziele = snapshot.ziele();
        Self {
            id: runde.id().0,
            herkunft,
            name,
            erfasst_von: runde.erfasst_von().into(),
            zustand: zustand(&runde),
            begonnen_am: runde.begonnen_am(),
            abgeschlossen_am: runde.abgeschlossen_am(),
            pflöcke: pflöcke
                .iter()
                .map(|pflock| RundePflock {
                    farbe: pflock.farbe().into(),
                    position: pflock.position(),
                })
                .collect(),
            ziele: ziele
                .into_iter()
                .map(|ziel| RundeZiel {
                    ziel_id: ziel.id().as_uuid(),
                    tiername: ziel.name().to_string(),
                    zielnummer: ziel.ordinal().get(),
                    pflöcke: effective_pflöcke(ziel, pflöcke),
                    abstände: ziel
                        .abstände()
                        .iter()
                        .map(|(farbe, abstand)| Abstand {
                            pflock: (*farbe).into(),
                            meter: abstand.meter(),
                        })
                        .collect(),
                    wertung: None,
                    schüsse: Vec::new(),
                })
                .collect(),
            gesamt_erster_treffer: 0,
            gesamt_mehrschuss: 0,
            ziele_mit_wertung: 0,
        }
    }
}

impl From<domain::Runde> for RundeBasic {
    fn from(runde: domain::Runde) -> Self {
        let (herkunft, name) = match runde.aufbau() {
            domain::Rundenaufbau::Parkour {
                anlage_id,
                parkour_id,
                parkour_name,
                ..
            } => (
                Some(RundeHerkunft::from((*anlage_id, *parkour_id))),
                parkour_name.to_string(),
            ),
            domain::Rundenaufbau::Freie { name, .. } => (None, name.to_string()),
        };
        let anzahl_ziele = runde.aufbau().snapshot().ziele().len();
        Self {
            id: runde.id().0,
            herkunft,
            name,
            anzahl_ziele,
            anzahl_erfasste_ziele: 0,
            zustand: zustand(&runde),
            begonnen_am: runde.begonnen_am(),
            abgeschlossen_am: runde.abgeschlossen_am(),
        }
    }
}
