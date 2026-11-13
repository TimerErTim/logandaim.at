//! Outbound schemas of a Runde: the captured route, its per-Ziel results, and the totals.
//!
//! A Runde read is the only place a Wertung is served: there is no `/wertung` endpoint
//! (SPEC-wertung rule 6). The captured fields are copied verbatim from what was captured,
//! never re-read from master data (AC-7, AC-9).

use chrono::{DateTime, Utc};
use uuid::Uuid;

use logandaim_core::domain;

use crate::schema::{Erfasser, Farbe, Schuss, Zielwertung};

/// The Anlage/Parkour identities a Runde was started from.
///
/// Both ids are present together; free rounds have no provenance.
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

impl From<domain::RundenHerkunft> for RundeHerkunft {
    fn from(herkunft: domain::RundenHerkunft) -> Self {
        Self {
            anlage_id: herkunft.anlage_id.0,
            parkour_id: herkunft.parkour_id.0,
        }
    }
}

/// One Pflock of this Ziel's captured route.
///
/// `abstand` is the configured distance from this Pflock to its Ziel, in metres; it is absent
/// when no distance was configured.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pflock {
    /// The captured Pflock's colour.
    pub farbe: Farbe,
    /// Distance from this Pflock to its Ziel, in metres; absent when unconfigured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abstand: Option<u32>,
}

/// One target in this Runde's captured route: identity, name, ordinal, Pflöcke, and Wertung.
///
/// A free-round Ziel has no configured Pflöcke.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ziel {
    /// Identity of the target within this Runde.
    pub ziel_id: Uuid,
    /// Name captured in the route.
    pub tiername: String,
    /// 1-based route ordinal.
    pub zielnummer: u32,
    /// The Pflöcke that apply to this Ziel as captured, near to far.
    #[cfg_attr(feature = "openapi", oai(rename = "pfloecke"))]
    #[serde(rename = "pfloecke")]
    pub pflöcke: Vec<Pflock>,
    /// The Wertung of this Ziel; absent while no Schuss has been logged at it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wertung: Option<Zielwertung>,
    /// The logged arrows at this Ziel, in Pfeil order.
    #[cfg_attr(feature = "openapi", oai(rename = "schuesse"))]
    #[serde(rename = "schuesse")]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub herkunft: Option<RundeHerkunft>,
    /// The round's own name.
    pub name: String,
    /// Who the Runde belongs to.
    pub erfasst_von: Erfasser,
    /// Whether it is still open or already closed.
    pub zustand: RundeZustand,
    /// Start of the Runde.
    pub begonnen_am: DateTime<Utc>,
    /// End of the Runde; absent while it is open.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abgeschlossen_am: Option<DateTime<Utc>>,
    /// The Ziele as captured, with their results.
    pub ziele: Vec<Ziel>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub herkunft: Option<RundeHerkunft>,
    /// The round's own name.
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
        let ziele = runde
            .aufbau()
            .ziele()
            .iter()
            .zip(wertungen)
            .map(|(ziel, wertung)| Ziel {
                ziel_id: ziel.id().0,
                tiername: ziel.name().to_string(),
                zielnummer: ziel.ordinal().get(),
                pflöcke: ziel
                    .pflöcke()
                    .iter()
                    .map(|pflock| Pflock {
                        farbe: pflock.farbe().into(),
                        abstand: pflock.abstand().map(|abstand| abstand.meter()),
                    })
                    .collect(),
                wertung: wertung.map(Into::into),
                schüsse: schüsse
                    .iter()
                    .filter(|schuss| schuss.ziel_id() == ziel.id().into())
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

/// A freshly started or freshly closed Runde, before any Wertung is folded.
///
/// `RundeStarten` and `RundeAbschliessen` answer the aggregate, not a read model, so this
/// impl renders the captured route exactly and leaves every Wertung empty.
impl From<domain::Runde> for Runde {
    fn from(runde: domain::Runde) -> Self {
        let herkunft = runde.herkunft().copied().map(Into::into);
        let name = runde.name().to_string();
        let aufbau = runde.aufbau();
        Self {
            id: runde.id().0,
            herkunft,
            name,
            erfasst_von: runde.erfasst_von().into(),
            zustand: zustand(&runde),
            begonnen_am: runde.begonnen_am(),
            abgeschlossen_am: runde.abgeschlossen_am(),
            ziele: aufbau
                .ziele()
                .iter()
                .map(|ziel| Ziel {
                    ziel_id: ziel.id().0,
                    tiername: ziel.name().to_string(),
                    zielnummer: ziel.ordinal().get(),
                    pflöcke: ziel
                        .pflöcke()
                        .iter()
                        .map(|pflock| Pflock {
                            farbe: pflock.farbe().into(),
                            abstand: pflock.abstand().map(|abstand| abstand.meter()),
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
        let herkunft = runde.herkunft().copied().map(Into::into);
        let name = runde.name().to_string();
        let anzahl_ziele = runde.aufbau().ziele().len();
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
