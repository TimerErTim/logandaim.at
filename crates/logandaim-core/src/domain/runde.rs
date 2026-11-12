//! One logged Runde: its route, where it came from, and how far it has come.
//!
//! A `Runde` is a standalone aggregate root. It never holds a collection of `Schuss`:
//! each `Schuss` carries `runde_id`, so two aggregates stay two aggregates and no use case
//! writes both. The Runde-wide Wertung folds the per-Ziel results once, in the order of the
//! captured route (SPEC-wertung rule 6, AC-8).
//!
//! The root is a **typestate**. Two things live in the state, never in an `Option` or a
//! parallel enum:
//!
//! - the **setup kind**, as the provenance of the round ([`Rundenherkunft`]), and
//! - the **lifecycle**, as whether a closing timestamp exists ([`Abgeschlossen`]).
//!
//! The setup kind is the marker, so "a free round over a Parkour setup" has no spelling: the
//! marker fields are private and every constructor picks the marker from the same value it
//! takes the route from. Route mutations exist only on [`FreieRundeInArbeit`], which is the
//! AC-22 promise as a compile-time guarantee, and a closed round has no mutator at all.
//!
//! The route itself ([`Rundenaufbau`]) is the same shape in every state: it records the
//! topology and the captured data, and it is immutable once the round is closed.

use std::num::NonZeroU32;

use chrono::{DateTime, Utc};

use super::anlage::{
    AnlageId, AnlageZielId, ParkourId, ParkourSnapshot, PflockSnapshot, ZielPflöckeSnapshot,
    ZielSnapshot,
};
use super::common::{Abstand, Farbe, NonEmptyString};
use super::konto::BenutzerId;
use super::schuss::{Erfasser, Schuss, SchussZielId, Schussposition, ZielSchüsse, Zielwertung};

/// Identity of one Runde. Re-exported where a `Schuss` needs it.
pub use super::schuss::RundeId;

/// The one stored spelling of a running Runde, shared by both open markers.
///
/// The SQLite partial unique index `idx_runden_offen_pro_benutzer` and
/// `find_offen_by_benutzer` filter on this literal, so it is spelled once, here.
pub const ZUSTAND_IN_BEARBEITUNG: &str = "InBearbeitung";

/// The one stored spelling of a closed Runde.
pub const ZUSTAND_ABGESCHLOSSEN: &str = "Abgeschlossen";

// ---------------------------------------------------------------------------------------
// Where a Runde came from
// ---------------------------------------------------------------------------------------

/// The provenance of a Runde: the Parkour it was started at, or the name it gave itself.
///
/// Provenance, not topology: the route it was seeded with lives in [`Rundenaufbau`], which is
/// the same shape for both. A reader therefore asks two separate questions and never has to
/// interpret "which setup is this?" out of one field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rundenherkunft {
    Parkour {
        anlage_id: AnlageId,
        parkour_id: ParkourId,
        parkour_name: NonEmptyString,
    },
    Freie {
        name: NonEmptyString,
    },
}

impl Rundenherkunft {
    /// The round's name: the captured Parkour's, or the free round's own.
    pub fn name(&self) -> &NonEmptyString {
        match self {
            Self::Parkour { parkour_name, .. } => parkour_name,
            Self::Freie { name } => name,
        }
    }
}

/// The provenance every state can answer, so a closed round still says where it came from.
///
/// The markers hold the provenance, so this is what [`RundeIn`] borrows it through; nothing is
/// cloned to read a round's name.
pub trait RundenherkunftView {
    fn herkunft(&self) -> &Rundenherkunft;
}

// ---------------------------------------------------------------------------------------
// Lifecycle markers of the `Runde` typestate
// ---------------------------------------------------------------------------------------

/// A running round seeded from a Parkour.
///
/// The field is private and the only constructor is in this module, so this state cannot be
/// built over [`Rundenherkunft::Freie`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MitParkour {
    herkunft: Rundenherkunft,
}

/// A running round that owns its Ziel list and permits just-in-time route edits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreieRundeInBearbeitung {
    herkunft: Rundenherkunft,
}

/// The closed state: it carries when it happened and the provenance it happened under.
///
/// Both live in the marker, so "when was it closed?" and "what was it?" are not `Option`s a
/// reader must interpret, and closing never erases either.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abgeschlossen {
    herkunft: Rundenherkunft,
    abgeschlossen_am: DateTime<Utc>,
}

impl MitParkour {
    fn neu(anlage_id: AnlageId, parkour_id: ParkourId, parkour_name: NonEmptyString) -> Self {
        Self {
            herkunft: Rundenherkunft::Parkour {
                anlage_id,
                parkour_id,
                parkour_name,
            },
        }
    }

    pub fn as_str(&self) -> &'static str {
        ZUSTAND_IN_BEARBEITUNG
    }
}

impl FreieRundeInBearbeitung {
    fn neu(name: NonEmptyString) -> Self {
        Self {
            herkunft: Rundenherkunft::Freie { name },
        }
    }

    pub fn as_str(&self) -> &'static str {
        ZUSTAND_IN_BEARBEITUNG
    }
}

impl Abgeschlossen {
    pub fn abgeschlossen_am(&self) -> DateTime<Utc> {
        self.abgeschlossen_am
    }

    pub fn as_str(&self) -> &'static str {
        ZUSTAND_ABGESCHLOSSEN
    }
}

impl RundenherkunftView for MitParkour {
    fn herkunft(&self) -> &Rundenherkunft {
        &self.herkunft
    }
}

impl RundenherkunftView for FreieRundeInBearbeitung {
    fn herkunft(&self) -> &Rundenherkunft {
        &self.herkunft
    }
}

impl RundenherkunftView for Abgeschlossen {
    fn herkunft(&self) -> &Rundenherkunft {
        &self.herkunft
    }
}

// ---------------------------------------------------------------------------------------
// Round-owned target route
// ---------------------------------------------------------------------------------------

/// Neutral identity for a target within a single Runde.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RundenZielId(pub uuid::Uuid);

/// One target in the route captured by a Runde.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RundenZiel {
    id: RundenZielId,
    anlage_ziel_id: Option<AnlageZielId>,
    name: NonEmptyString,
    ordinal: NonZeroU32,
    pflöcke: ZielPflöckeSnapshot,
    abstände: Vec<(Farbe, Abstand)>,
}

impl RundenZiel {
    pub fn from_parts(
        id: RundenZielId,
        anlage_ziel_id: Option<AnlageZielId>,
        name: NonEmptyString,
        ordinal: NonZeroU32,
        pflöcke: ZielPflöckeSnapshot,
        abstände: Vec<(Farbe, Abstand)>,
    ) -> Self {
        Self {
            id,
            anlage_ziel_id,
            name,
            ordinal,
            pflöcke,
            abstände,
        }
    }

    fn from_parkour(ziel: &ZielSnapshot) -> Self {
        Self {
            id: RundenZielId(uuid::Uuid::new_v4()),
            anlage_ziel_id: Some(ziel.ziel_id()),
            name: ziel.tiername().clone(),
            ordinal: ziel.zielnummer(),
            pflöcke: ziel.pflöcke().clone(),
            abstände: ziel.abstände().to_vec(),
        }
    }

    pub fn id(&self) -> RundenZielId {
        self.id
    }
    pub fn anlage_ziel_id(&self) -> Option<AnlageZielId> {
        self.anlage_ziel_id
    }
    pub fn name(&self) -> &NonEmptyString {
        &self.name
    }
    pub fn ordinal(&self) -> NonZeroU32 {
        self.ordinal
    }
    pub fn pflöcke(&self) -> &ZielPflöckeSnapshot {
        &self.pflöcke
    }
    pub fn abstände(&self) -> &[(Farbe, Abstand)] {
        &self.abstände
    }

    fn umbenennen(&mut self, name: NonEmptyString) {
        self.name = name;
    }
}

/// The route one Runde captured: its Pflöcke and its ordered Ziele.
///
/// Topology and data only, with no provenance inside it, so the same value serves a Parkour
/// round, a free round, and a closed round. A Parkour round seeds it from the `ParkourSnapshot`
/// (AC-7/AC-9: a value copy, never a live read) and leaves it alone; a free round appends and
/// removes targets while it runs; a closed round keeps it as the round's historical snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rundenaufbau {
    pflöcke: Vec<PflockSnapshot>,
    ziele: Vec<RundenZiel>,
}

impl Rundenaufbau {
    pub fn from_parts(pflöcke: Vec<PflockSnapshot>, ziele: Vec<RundenZiel>) -> Self {
        Self { pflöcke, ziele }
    }

    fn from_parkour(snapshot: &ParkourSnapshot) -> Self {
        Self {
            pflöcke: snapshot.pflöcke().to_vec(),
            ziele: snapshot
                .ziele()
                .iter()
                .map(RundenZiel::from_parkour)
                .collect(),
        }
    }

    pub fn pflöcke(&self) -> &[PflockSnapshot] {
        &self.pflöcke
    }
    pub fn ziele(&self) -> &[RundenZiel] {
        &self.ziele
    }
    pub fn ziel(&self, id: RundenZielId) -> Option<&RundenZiel> {
        self.ziele.iter().find(|ziel| ziel.id == id)
    }
    pub fn enthält_ziel(&self, id: RundenZielId) -> bool {
        self.ziel(id).is_some()
    }

    fn ziel_mut(&mut self, id: RundenZielId) -> Option<&mut RundenZiel> {
        self.ziele.iter_mut().find(|ziel| ziel.id == id)
    }

    fn ziel_hinzufügen(&mut self, id: RundenZielId, name: Option<NonEmptyString>) {
        let ordinal = self
            .ziele
            .iter()
            .map(|ziel| ziel.ordinal.get())
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        let ordinal = NonZeroU32::new(ordinal).unwrap_or(NonZeroU32::MIN);
        let name = name.unwrap_or_else(|| {
            NonEmptyString::neu(format!("Ziel {ordinal}")).expect("ordinal name is non-empty")
        });
        self.ziele.push(RundenZiel::from_parts(
            id,
            None,
            name,
            ordinal,
            ZielPflöckeSnapshot::Explicit(Vec::new()),
            Vec::new(),
        ));
    }

    fn ziel_entfernen(&mut self, id: RundenZielId) -> Option<RundenZiel> {
        let index = self.ziele.iter().position(|ziel| ziel.id == id)?;
        Some(self.ziele.remove(index))
    }

    pub fn hat_pflock_am_ziel(&self, id: RundenZielId, farbe: Farbe) -> bool {
        self.ziel(id).is_some_and(|ziel| {
            ziel.pflöcke
                .effektive(&self.pflöcke)
                .iter()
                .any(|pflock| pflock.farbe() == farbe)
        })
    }

    pub fn ziel_id_für_schuss(&self, ziel_id: SchussZielId) -> Option<RundenZielId> {
        self.ziele
            .iter()
            .find(|ziel| ziel.id.0 == ziel_id.0)
            .map(RundenZiel::id)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("der Pflock gehört nicht zum Aufbau dieser Runde")]
pub struct PositionNichtImAufbau;

// ---------------------------------------------------------------------------------------
// Creation input
// ---------------------------------------------------------------------------------------

/// Creation input of a Runde: everything except its identity and lifecycle state.
///
/// A variant, not a bag of optional fields, so neither setup can be built half-way: the
/// provenance and the route both follow from the variant (SPEC-runde rule 17).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeueRunde {
    Parkour {
        anlage_id: AnlageId,
        erfasser: Erfasser,
        /// A value copy taken from the `Anlage` in the same transaction, so a later
        /// master-data edit cannot reach it (AC-7, AC-9).
        snapshot: ParkourSnapshot,
        begonnen_am: DateTime<Utc>,
    },
    Freie {
        name: NonEmptyString,
        erfasser: Erfasser,
        begonnen_am: DateTime<Utc>,
    },
}

impl NeueRunde {
    /// Start a Parkour round; the Anlage/Parkour pair is derived from the snapshot.
    pub fn mit_parkour(
        anlage_id: AnlageId,
        erfasser: Erfasser,
        snapshot: ParkourSnapshot,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self::Parkour {
            anlage_id,
            erfasser,
            snapshot,
            begonnen_am,
        }
    }

    /// Start a `Freie Runde`; it names no Anlage and no Parkour.
    pub fn ohne_parkour(
        name: NonEmptyString,
        erfasser: Erfasser,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self::Freie {
            name,
            erfasser,
            begonnen_am,
        }
    }

    pub fn erfasst_von(&self) -> Erfasser {
        match self {
            Self::Parkour { erfasser, .. } | Self::Freie { erfasser, .. } => *erfasser,
        }
    }

    pub fn begonnen_am(&self) -> DateTime<Utc> {
        match self {
            Self::Parkour { begonnen_am, .. } | Self::Freie { begonnen_am, .. } => *begonnen_am,
        }
    }

    /// Where this round comes from, built once so the pair cannot be split.
    pub fn herkunft(&self) -> Rundenherkunft {
        match self {
            Self::Parkour {
                anlage_id,
                snapshot,
                ..
            } => Rundenherkunft::Parkour {
                anlage_id: *anlage_id,
                parkour_id: snapshot.parkour_id(),
                parkour_name: snapshot.parkour_name().clone(),
            },
            Self::Freie { name, .. } => Rundenherkunft::Freie { name: name.clone() },
        }
    }
}

// ---------------------------------------------------------------------------------------
// The aggregate
// ---------------------------------------------------------------------------------------

/// One logged Runde: who, the captured route, and the state that names its kind and lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RundeIn<S> {
    id: RundeId,
    erfasst_von: Erfasser,
    aufbau: Rundenaufbau,
    begonnen_am: DateTime<Utc>,
    state: S,
}

pub type RundeInBearbeitung = RundeIn<MitParkour>;
pub type FreieRundeInArbeit = RundeIn<FreieRundeInBearbeitung>;
pub type RundeAbgeschlossen = RundeIn<Abgeschlossen>;

/// The setup-specific states of an open Runde and its common closed state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Runde {
    InBearbeitung(RundeInBearbeitung),
    FreieRundeInArbeit(FreieRundeInArbeit),
    Abgeschlossen(RundeAbgeschlossen),
}

impl<S> RundeIn<S> {
    pub fn id(&self) -> RundeId {
        self.id
    }

    pub fn aufbau(&self) -> &Rundenaufbau {
        &self.aufbau
    }

    pub fn erfasst_von(&self) -> Erfasser {
        self.erfasst_von
    }

    pub fn begonnen_am(&self) -> DateTime<Utc> {
        self.begonnen_am
    }

    pub fn gehört_zu(&self, benutzer_id: BenutzerId) -> bool {
        self.erfasst_von.benutzer() == Some(benutzer_id)
    }

    /// Whether the captured route contains this Ziel (the AC-9 contract a Schuss obeys).
    pub fn enthält_ziel(&self, ziel_id: RundenZielId) -> bool {
        self.aufbau.enthält_ziel(ziel_id)
    }

    /// Replace the owner with [`Erfasser::Anonymisiert`] (SPEC-datenschutz retention).
    /// The route, the provenance, and every Schuss stay untouched (NFR-3).
    pub fn anonymisiere(&mut self, am: DateTime<Utc>) {
        self.erfasst_von = Erfasser::Anonymisiert { am };
    }

    /// Fold the given Schüsse into one Wertung per Ziel, in visiting order.
    ///
    /// `None` marks a Ziel no Schuss was logged at (AC-6). The slice pairs with the route's
    /// own Ziel order, so a reader zips the two instead of re-deriving a fold.
    pub fn ziel_wertungen(
        &self,
        schüsse: impl IntoIterator<Item = Schuss>,
    ) -> Vec<Option<Zielwertung>> {
        let eigene: Vec<Schuss> = schüsse
            .into_iter()
            .filter(|schuss| schuss.runde_id() == self.id)
            .collect();
        self.aufbau
            .ziele()
            .iter()
            .map(|ziel| {
                // The filter names exactly this Ziel, so the collection cannot refuse a row.
                let ziel_id = ziel.id();
                ZielSchüsse::für_ziel(
                    ziel_id.into(),
                    eigene
                        .iter()
                        .filter(|schuss| {
                            self.aufbau.ziel_id_für_schuss(schuss.ziel_id()) == Some(ziel_id)
                        })
                        .cloned(),
                )
                .ok()
                .and_then(|menge| menge.wertung())
            })
            .collect()
    }

    /// Fold every Schuss of this Runde into its per-Ziel and Runde-wide Wertung.
    pub fn wertung(&self, schüsse: impl IntoIterator<Item = Schuss>) -> Rundenwertung {
        let eigener: Vec<Schuss> = schüsse
            .into_iter()
            .filter(|schuss| schuss.runde_id() == self.id)
            .collect();
        let je_ziel = self.aufbau.ziele().iter().filter_map(|ziel| {
            // The filter names exactly this Ziel, so the collection cannot refuse a row here.
            let ziel_id = ziel.id();
            let schuss_ziel_id = ziel_id.into();
            ZielSchüsse::für_ziel(
                schuss_ziel_id,
                eigener
                    .iter()
                    .filter(|schuss| {
                        self.aufbau.ziel_id_für_schuss(schuss.ziel_id()) == Some(ziel_id)
                    })
                    .cloned(),
            )
            .ok()
        });
        Rundenwertung::aus_zielen(je_ziel.map(|menge| menge.wertung()))
    }
}

impl<S: RundenherkunftView> RundeIn<S> {
    /// Where this round came from, borrowed from the state that stores it.
    pub fn herkunft(&self) -> &Rundenherkunft {
        self.state.herkunft()
    }

    /// The round's name: the captured Parkour's, or the free round's own.
    pub fn name(&self) -> &NonEmptyString {
        self.herkunft().name()
    }

    /// Whether this position may be captured at this Ziel (AC-9, AC-23).
    ///
    /// A Parkour round only accepts a Pflock its route carries at that Ziel; a free round
    /// configures no Pflock set, so the colour the archer picked is a position as such. The
    /// provenance decides, which is why the question is asked here and not of the route.
    pub fn pruefe_position(
        &self,
        ziel_id: RundenZielId,
        position: Option<Schussposition>,
    ) -> Result<(), PositionNichtImAufbau> {
        if !self.aufbau.enthält_ziel(ziel_id) {
            return Err(PositionNichtImAufbau);
        }
        match (self.herkunft(), position) {
            (_, None) => Ok(()),
            (Rundenherkunft::Freie { .. }, Some(_)) => Ok(()),
            (Rundenherkunft::Parkour { .. }, Some(Schussposition::Distanz(_))) => Ok(()),
            (Rundenherkunft::Parkour { .. }, Some(Schussposition::Pflock(farbe)))
                if self.aufbau.hat_pflock_am_ziel(ziel_id, farbe) =>
            {
                Ok(())
            }
            (Rundenherkunft::Parkour { .. }, Some(Schussposition::Pflock(_))) => {
                Err(PositionNichtImAufbau)
            }
        }
    }
}

impl RundeInBearbeitung {
    /// Start a Parkour round from its captured setup.
    ///
    /// The parameters are the Parkour's own identities, which a free round does not have, so
    /// a free round cannot reach this constructor at all — not even by mistake.
    fn aus_neu(
        id: RundeId,
        erfasser: Erfasser,
        anlage_id: AnlageId,
        snapshot: ParkourSnapshot,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        let state = MitParkour::neu(
            anlage_id,
            snapshot.parkour_id(),
            snapshot.parkour_name().clone(),
        );
        Self {
            id,
            erfasst_von: erfasser,
            aufbau: Rundenaufbau::from_parkour(&snapshot),
            begonnen_am,
            state,
        }
    }

    /// Rebuild a persisted Parkour round; the same Parkour identities gate the state.
    fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        anlage_id: AnlageId,
        parkour_id: ParkourId,
        parkour_name: NonEmptyString,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            erfasst_von: erfasser,
            aufbau,
            begonnen_am,
            state: MitParkour::neu(anlage_id, parkour_id, parkour_name),
        }
    }

    /// Close a Parkour-backed Runde: no further Schuss may be registered.
    pub fn abschliessen(self, am: DateTime<Utc>) -> RundeAbgeschlossen {
        RundeIn {
            id: self.id,
            erfasst_von: self.erfasst_von,
            aufbau: self.aufbau,
            begonnen_am: self.begonnen_am,
            state: Abgeschlossen {
                herkunft: self.state.herkunft,
                abgeschlossen_am: am,
            },
        }
    }
}

impl FreieRundeInArbeit {
    /// Start a free round: it names itself and owns an empty route.
    fn aus_neu(
        id: RundeId,
        erfasser: Erfasser,
        name: NonEmptyString,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            erfasst_von: erfasser,
            aufbau: Rundenaufbau::from_parts(Vec::new(), Vec::new()),
            begonnen_am,
            state: FreieRundeInBearbeitung::neu(name),
        }
    }

    fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        name: NonEmptyString,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            erfasst_von: erfasser,
            aufbau,
            begonnen_am,
            state: FreieRundeInBearbeitung::neu(name),
        }
    }

    /// Append a Ziel with a fresh round-local identity and answer that identity.
    ///
    /// The round mints the id, so a caller cannot hand in one that already exists and the
    /// append cannot be refused: a duplicate is unrepresentable, not an error to report.
    pub fn ziel_hinzufügen(&mut self, name: Option<NonEmptyString>) -> RundenZielId {
        let id = RundenZielId(uuid::Uuid::new_v4());
        self.aufbau.ziel_hinzufügen(id, name);
        id
    }

    pub fn ziel_entfernen(&mut self, id: RundenZielId) -> Option<RundenZiel> {
        self.aufbau.ziel_entfernen(id)
    }

    pub fn ziel_umbenennen(&mut self, id: RundenZielId, name: NonEmptyString) -> bool {
        let Some(ziel) = self.aufbau.ziel_mut(id) else {
            return false;
        };
        ziel.umbenennen(name);
        true
    }

    /// Close the free round: its route and its names are the round's historical snapshot.
    pub fn abschliessen(self, am: DateTime<Utc>) -> RundeAbgeschlossen {
        RundeIn {
            id: self.id,
            erfasst_von: self.erfasst_von,
            aufbau: self.aufbau,
            begonnen_am: self.begonnen_am,
            state: Abgeschlossen {
                herkunft: self.state.herkunft,
                abgeschlossen_am: am,
            },
        }
    }
}

impl RundeAbgeschlossen {
    /// Rebuild a persisted, closed Runde.
    fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        herkunft: Rundenherkunft,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
        abgeschlossen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            erfasst_von: erfasser,
            aufbau,
            begonnen_am,
            state: Abgeschlossen {
                herkunft,
                abgeschlossen_am,
            },
        }
    }

    pub fn abgeschlossen_am(&self) -> DateTime<Utc> {
        self.state.abgeschlossen_am
    }
}

impl From<RundeInBearbeitung> for Runde {
    fn from(runde: RundeInBearbeitung) -> Self {
        Self::InBearbeitung(runde)
    }
}

impl From<FreieRundeInArbeit> for Runde {
    fn from(runde: FreieRundeInArbeit) -> Self {
        Self::FreieRundeInArbeit(runde)
    }
}

impl From<RundeAbgeschlossen> for Runde {
    fn from(runde: RundeAbgeschlossen) -> Self {
        Self::Abgeschlossen(runde)
    }
}

impl Runde {
    /// Start a Runde in the state its setup names.
    ///
    /// The state and the route are read from the same variant, so neither can be picked
    /// independently of the other.
    pub fn aus_neu(id: RundeId, neu: NeueRunde) -> Self {
        match neu {
            NeueRunde::Parkour {
                anlage_id,
                erfasser,
                snapshot,
                begonnen_am,
            } => Self::InBearbeitung(RundeInBearbeitung::aus_neu(
                id,
                erfasser,
                anlage_id,
                snapshot,
                begonnen_am,
            )),
            NeueRunde::Freie {
                name,
                erfasser,
                begonnen_am,
            } => Self::FreieRundeInArbeit(FreieRundeInArbeit::aus_neu(
                id,
                erfasser,
                name,
                begonnen_am,
            )),
        }
    }

    /// Rebuild a persisted Runde in either state.
    ///
    /// The closing timestamp *is* the state, so the adapter passes what it read and the
    /// typestate follows: `None` is a running Runde, `Some` a closed one. The provenance is
    /// the same value the state is chosen from, so a loaded round cannot disagree with itself.
    pub fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        herkunft: Rundenherkunft,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
        abgeschlossen_am: Option<DateTime<Utc>>,
    ) -> Self {
        match (herkunft, abgeschlossen_am) {
            (
                Rundenherkunft::Parkour {
                    anlage_id,
                    parkour_id,
                    parkour_name,
                },
                None,
            ) => Self::InBearbeitung(RundeInBearbeitung::from_parts(
                id,
                erfasser,
                anlage_id,
                parkour_id,
                parkour_name,
                aufbau,
                begonnen_am,
            )),
            (Rundenherkunft::Freie { name }, None) => Self::FreieRundeInArbeit(
                FreieRundeInArbeit::from_parts(id, erfasser, name, aufbau, begonnen_am),
            ),
            (herkunft, Some(am)) => Self::Abgeschlossen(RundeAbgeschlossen::from_parts(
                id,
                erfasser,
                herkunft,
                aufbau,
                begonnen_am,
                am,
            )),
        }
    }

    pub fn id(&self) -> RundeId {
        match self {
            Self::InBearbeitung(runde) => runde.id(),
            Self::FreieRundeInArbeit(runde) => runde.id(),
            Self::Abgeschlossen(runde) => runde.id(),
        }
    }

    pub fn aufbau(&self) -> &Rundenaufbau {
        match self {
            Self::InBearbeitung(runde) => runde.aufbau(),
            Self::FreieRundeInArbeit(runde) => runde.aufbau(),
            Self::Abgeschlossen(runde) => runde.aufbau(),
        }
    }

    /// Where this round came from; a closed round answers it too.
    pub fn herkunft(&self) -> &Rundenherkunft {
        match self {
            Self::InBearbeitung(runde) => runde.herkunft(),
            Self::FreieRundeInArbeit(runde) => runde.herkunft(),
            Self::Abgeschlossen(runde) => runde.herkunft(),
        }
    }

    /// The round's name: the captured Parkour's, or the free round's own.
    pub fn name(&self) -> &NonEmptyString {
        self.herkunft().name()
    }

    pub fn erfasst_von(&self) -> Erfasser {
        match self {
            Self::InBearbeitung(runde) => runde.erfasst_von(),
            Self::FreieRundeInArbeit(runde) => runde.erfasst_von(),
            Self::Abgeschlossen(runde) => runde.erfasst_von(),
        }
    }

    /// The stored lifecycle spelling, taken from the active variant itself.
    pub fn zustand(&self) -> &'static str {
        match self {
            Self::InBearbeitung(runde) => runde.state.as_str(),
            Self::FreieRundeInArbeit(runde) => runde.state.as_str(),
            Self::Abgeschlossen(runde) => runde.state.as_str(),
        }
    }

    pub fn begonnen_am(&self) -> DateTime<Utc> {
        match self {
            Self::InBearbeitung(runde) => runde.begonnen_am(),
            Self::FreieRundeInArbeit(runde) => runde.begonnen_am(),
            Self::Abgeschlossen(runde) => runde.begonnen_am(),
        }
    }

    pub fn abgeschlossen_am(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::InBearbeitung(_) | Self::FreieRundeInArbeit(_) => None,
            Self::Abgeschlossen(runde) => Some(runde.abgeschlossen_am()),
        }
    }

    pub fn gehört_zu(&self, benutzer_id: BenutzerId) -> bool {
        match self {
            Self::InBearbeitung(runde) => runde.gehört_zu(benutzer_id),
            Self::FreieRundeInArbeit(runde) => runde.gehört_zu(benutzer_id),
            Self::Abgeschlossen(runde) => runde.gehört_zu(benutzer_id),
        }
    }

    pub fn ist_in_bearbeitung(&self) -> bool {
        matches!(self, Self::InBearbeitung(_) | Self::FreieRundeInArbeit(_))
    }

    /// Whether this round is a free one, whether it still runs or is closed.
    pub fn ist_freie_runde(&self) -> bool {
        matches!(self.herkunft(), Rundenherkunft::Freie { .. })
    }

    pub fn enthält_ziel(&self, ziel_id: RundenZielId) -> bool {
        match self {
            Self::InBearbeitung(runde) => runde.enthält_ziel(ziel_id),
            Self::FreieRundeInArbeit(runde) => runde.enthält_ziel(ziel_id),
            Self::Abgeschlossen(runde) => runde.enthält_ziel(ziel_id),
        }
    }

    /// Whether this position may be captured at this Ziel (AC-9, AC-23).
    pub fn pruefe_position(
        &self,
        ziel_id: RundenZielId,
        position: Option<Schussposition>,
    ) -> Result<(), PositionNichtImAufbau> {
        match self {
            Self::InBearbeitung(runde) => runde.pruefe_position(ziel_id, position),
            Self::FreieRundeInArbeit(runde) => runde.pruefe_position(ziel_id, position),
            Self::Abgeschlossen(runde) => runde.pruefe_position(ziel_id, position),
        }
    }

    pub fn anonymisiere(&mut self, am: DateTime<Utc>) {
        match self {
            Self::InBearbeitung(runde) => runde.anonymisiere(am),
            Self::FreieRundeInArbeit(runde) => runde.anonymisiere(am),
            Self::Abgeschlossen(runde) => runde.anonymisiere(am),
        }
    }

    pub fn wertung(&self, schüsse: impl IntoIterator<Item = Schuss>) -> Rundenwertung {
        match self {
            Self::InBearbeitung(runde) => runde.wertung(schüsse),
            Self::FreieRundeInArbeit(runde) => runde.wertung(schüsse),
            Self::Abgeschlossen(runde) => runde.wertung(schüsse),
        }
    }

    pub fn ziel_wertungen(
        &self,
        schüsse: impl IntoIterator<Item = Schuss>,
    ) -> Vec<Option<Zielwertung>> {
        match self {
            Self::InBearbeitung(runde) => runde.ziel_wertungen(schüsse),
            Self::FreieRundeInArbeit(runde) => runde.ziel_wertungen(schüsse),
            Self::Abgeschlossen(runde) => runde.ziel_wertungen(schüsse),
        }
    }
}

/// The total per Wertung over one Runde's ordered Ziele.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rundenwertung {
    erster_treffer: u32,
    mehrschuss: u32,
    ziele_mit_wertung: usize,
}

impl Rundenwertung {
    /// Sum per Wertung over all Ziel results. A Ziel without a result is skipped, not
    /// counted as zero (AC-8).
    pub fn aus_zielen(ziele: impl IntoIterator<Item = Option<Zielwertung>>) -> Self {
        ziele
            .into_iter()
            .flatten()
            .fold(Self::default(), |wertung, ziel| Self {
                erster_treffer: wertung.erster_treffer + ziel.erster_treffer(),
                mehrschuss: wertung.mehrschuss + ziel.mehrschuss(),
                ziele_mit_wertung: wertung.ziele_mit_wertung + 1,
            })
    }

    pub fn erster_treffer(&self) -> u32 {
        self.erster_treffer
    }

    pub fn mehrschuss(&self) -> u32 {
        self.mehrschuss
    }

    /// How many Ziele contributed a result.
    pub fn ziele_mit_wertung(&self) -> usize {
        self.ziele_mit_wertung
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::anlage::{PflockSnapshot, ZielPflöckeSnapshot, ZielSnapshot};
    use crate::domain::schuss::{NeuerSchuss, SchussId, Wertungszone};
    use std::num::NonZeroU32;
    use uuid::Uuid;

    fn name(wert: &str) -> NonEmptyString {
        NonEmptyString::neu(wert).unwrap()
    }

    /// The master-data id space the fixture captures from. It is deliberately unrelated to
    /// the round-local ids the round mints, so a leak of one into the other is visible.
    const ANLAGE_ZIEL_BASIS: u128 = 10_000;

    fn snapshot(ziele: &[(u128, &str)]) -> ParkourSnapshot {
        ParkourSnapshot::from_parts(
            ParkourId(Uuid::from_u128(7)),
            name("Waldrunde"),
            vec![PflockSnapshot::neu(Farbe::neu("#c0392b").unwrap(), 0)],
            ziele
                .iter()
                .map(|&(id, tiername)| {
                    ZielSnapshot::neu(
                        AnlageZielId(Uuid::from_u128(ANLAGE_ZIEL_BASIS + id)),
                        NonZeroU32::new(1).unwrap(),
                        name(tiername),
                        ZielPflöckeSnapshot::InheritFromParkour,
                        Vec::new(),
                    )
                })
                .collect(),
        )
    }

    fn jetzt() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    /// A running Parkour round over the given targets, keyed by the caller's label.
    ///
    /// The fixture never rewrites a round-local id: the round mints them, exactly as it does
    /// in production, and a caller reaches a target through the master-data id it was
    /// captured from.
    fn runde(ziele: &[(u128, &str)]) -> Runde {
        Runde::aus_neu(
            RundeId(Uuid::nil()),
            NeueRunde::mit_parkour(
                AnlageId(Uuid::nil()),
                Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                snapshot(ziele),
                jetzt(),
            ),
        )
    }

    fn freie_runde() -> Runde {
        Runde::FreieRundeInArbeit(FreieRundeInArbeit::aus_neu(
            RundeId(Uuid::nil()),
            Erfasser::Benutzer(BenutzerId(Uuid::nil())),
            name("Wiese daheim"),
            jetzt(),
        ))
    }

    /// The round-local id of the target captured from the master-data target `anlage_ziel`.
    fn ziel_id_zu(runde: &Runde, anlage_ziel: u128) -> RundenZielId {
        runde
            .aufbau()
            .ziele()
            .iter()
            .find(|ziel| {
                ziel.anlage_ziel_id()
                    == Some(AnlageZielId(Uuid::from_u128(
                        ANLAGE_ZIEL_BASIS + anlage_ziel,
                    )))
            })
            .expect("the round captured this target")
            .id()
    }

    fn schuss(runde: &Runde, anlage_ziel: u128, pfeil: usize, zone: Wertungszone) -> Schuss {
        schuss_auf(runde.id(), ziel_id_zu(runde, anlage_ziel), pfeil, zone)
    }

    fn schuss_auf(
        runde_id: RundeId,
        ziel_id: RundenZielId,
        pfeil: usize,
        zone: Wertungszone,
    ) -> Schuss {
        Schuss::neu(
            SchussId(Uuid::from_u128(pfeil as u128)),
            NeuerSchuss::neu(
                runde_id,
                Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                ziel_id.into(),
                pfeil,
                zone,
                None,
            )
            .unwrap(),
        )
    }

    #[test_log::test]
    fn ac8_die_summe_ist_die_summe_der_zielwertungen() {
        // Ziel 10: Kill, Kill, Spot = erster 18, mehr 38.
        // Ziel 11: Miss, Spot = erster 14, mehr 14.
        let runde = runde(&[(10, "Keiler"), (11, "Reh")]);
        let schüsse = vec![
            schuss(&runde, 10, 1, Wertungszone::Kill),
            schuss(&runde, 10, 2, Wertungszone::Kill),
            schuss(&runde, 10, 3, Wertungszone::Spot),
            schuss(&runde, 11, 2, Wertungszone::Spot),
        ];
        let wertung = runde.wertung(schüsse);
        assert_eq!(wertung.erster_treffer(), 18 + 14);
        assert_eq!(wertung.mehrschuss(), 38 + 14);
        assert_eq!(wertung.ziele_mit_wertung(), 2);
    }

    #[test_log::test]
    fn ac15_zielwertungen_zeigen_leerzustand_und_fortschritt_aus_schuessen() {
        let runde = runde(&[(10, "Keiler"), (11, "Reh")]);
        let schüsse = vec![schuss(&runde, 10, 1, Wertungszone::Kill)];
        let ziel_wertungen = runde.ziel_wertungen(schüsse.clone());
        let Some(erster) = ziel_wertungen[0] else {
            panic!("logged Ziel has a result");
        };
        assert_eq!(erster.erster_treffer(), 18);
        assert_eq!(erster.mehrschuss(), 18);
        assert!(ziel_wertungen[1].is_none(), "uncaptured Ziel has no result");
        let wertung = runde.wertung(schüsse);
        assert_eq!(wertung.ziele_mit_wertung(), 1);
    }

    #[test_log::test]
    fn ac8_ziele_ohne_schuss_werden_uebersprungen() {
        let runde = runde(&[(10, "Keiler"), (11, "Reh")]);
        let wertung = runde.wertung(vec![schuss(&runde, 11, 1, Wertungszone::Kill)]);
        assert_eq!(wertung.erster_treffer(), 18);
        assert_eq!(wertung.mehrschuss(), 18);
        assert_eq!(wertung.ziele_mit_wertung(), 1);
    }

    #[test_log::test]
    fn ac8_fremde_schüsse_zaehlen_nicht() {
        let runde = runde(&[(10, "Keiler")]);
        let mut fremd = schuss_auf(
            RundeId(Uuid::from_u128(99)),
            RundenZielId(Uuid::from_u128(99)),
            1,
            Wertungszone::Spot,
        );
        fremd.anonymisiere(jetzt());
        let wertung = runde.wertung(vec![
            schuss(&runde, 10, 1, Wertungszone::Kill),
            fremd,
            // A target this round never captured: same round, foreign Ziel.
            schuss_auf(
                RundeId(Uuid::nil()),
                RundenZielId(Uuid::from_u128(99)),
                1,
                Wertungszone::Spot,
            ),
        ]);
        assert_eq!(wertung.erster_treffer(), 18);
        assert_eq!(wertung.mehrschuss(), 18);
        assert_eq!(wertung.ziele_mit_wertung(), 1);
    }

    #[test_log::test]
    fn eine_leere_runde_hat_null() {
        let runde = runde(&[(10, "Keiler")]);
        let wertung = runde.wertung(Vec::new());
        assert_eq!(wertung.erster_treffer(), 0);
        assert_eq!(wertung.ziele_mit_wertung(), 0);
    }

    #[test_log::test]
    fn abschliessen_ist_eine_einbahnstrasse() {
        let Runde::InBearbeitung(runde) = runde(&[(10, "Keiler")]) else {
            panic!("eine neue Parkour-Runde ist in Bearbeitung");
        };
        let geschlossen: Runde = runde.abschliessen(jetzt()).into();
        assert_eq!(geschlossen.zustand(), "Abgeschlossen");
        assert_eq!(geschlossen.abgeschlossen_am(), Some(jetzt()));
        assert!(!geschlossen.ist_in_bearbeitung());
        // No `abschliessen` exists on the closed type: it is a compile error, not a refusal.
    }

    #[test_log::test]
    fn ac9_der_aufbau_kennt_nur_die_platzierte_route() {
        let runde = runde(&[(10, "Keiler")]);
        let ziel_id = runde.aufbau().ziele().first().unwrap().id();
        assert!(runde.enthält_ziel(ziel_id));
        assert!(!runde.enthält_ziel(RundenZielId(Uuid::from_u128(11))));
    }

    #[test_log::test]
    fn anonymisieren_laesst_aufbau_und_herkunft_unberuehrt() {
        let mut runde = runde(&[(10, "Keiler")]);
        let vorher = runde.aufbau().clone();
        let herkunft = runde.herkunft().clone();
        runde.anonymisiere(jetzt());
        assert_eq!(runde.erfasst_von(), Erfasser::Anonymisiert { am: jetzt() });
        assert_eq!(runde.aufbau(), &vorher);
        assert_eq!(runde.herkunft(), &herkunft);
    }

    #[test_log::test]
    fn from_parts_roundtrip_beider_aufbauten_und_zustaende() {
        let offen = runde(&[(10, "Keiler")]);
        let geladen = Runde::from_parts(
            offen.id(),
            offen.erfasst_von(),
            offen.herkunft().clone(),
            offen.aufbau().clone(),
            offen.begonnen_am(),
            offen.abgeschlossen_am(),
        );
        assert_eq!(offen, geladen);

        let freie = freie_runde();
        let geladen = Runde::from_parts(
            freie.id(),
            freie.erfasst_von(),
            freie.herkunft().clone(),
            freie.aufbau().clone(),
            freie.begonnen_am(),
            freie.abgeschlossen_am(),
        );
        assert_eq!(freie, geladen);

        let Runde::InBearbeitung(aktiv) = offen else {
            panic!("eine Parkour-Runde ist offen");
        };
        let geschlossen: Runde = aktiv.abschliessen(jetzt()).into();
        let geladen = Runde::from_parts(
            geschlossen.id(),
            geschlossen.erfasst_von(),
            geschlossen.herkunft().clone(),
            geschlossen.aufbau().clone(),
            geschlossen.begonnen_am(),
            geschlossen.abgeschlossen_am(),
        );
        assert_eq!(geschlossen, geladen);

        // The closed *free* round is its own case: its route is the free one's, and its
        // provenance must survive the round trip just as a Parkour round's does (#13).
        let Runde::FreieRundeInArbeit(aktiv) = freie_runde() else {
            panic!("eine freie Runde ist offen");
        };
        let mut aktiv = aktiv;
        aktiv.ziel_hinzufügen(Some(name("Baumstumpf")));
        let geschlossen: Runde = aktiv.abschliessen(jetzt()).into();
        assert_eq!(geschlossen.zustand(), "Abgeschlossen");
        assert!(geschlossen.ist_freie_runde());
        let geladen = Runde::from_parts(
            geschlossen.id(),
            geschlossen.erfasst_von(),
            geschlossen.herkunft().clone(),
            geschlossen.aufbau().clone(),
            geschlossen.begonnen_am(),
            geschlossen.abgeschlossen_am(),
        );
        assert_eq!(geschlossen, geladen);
        assert_eq!(geladen.aufbau().ziele().len(), 1);
        assert_eq!(geladen.name().as_str(), "Wiese daheim");
    }

    #[test_log::test]
    fn ein_abschlusszeitpunkt_macht_die_geladene_runde_abgeschlossen() {
        // The timestamp *is* the state: there is no second flag that could disagree with it.
        let offen = runde(&[(10, "Keiler")]);
        let geladen = Runde::from_parts(
            offen.id(),
            offen.erfasst_von(),
            offen.herkunft().clone(),
            offen.aufbau().clone(),
            offen.begonnen_am(),
            Some(jetzt()),
        );
        assert_eq!(geladen.zustand(), "Abgeschlossen");
        assert_eq!(geladen.abgeschlossen_am(), Some(jetzt()));
    }

    #[test_log::test]
    fn ac21_ein_parkour_aufbau_traegt_seine_provenienz_und_den_gemeinsamen_aufbau() {
        let runde = runde(&[(10, "Keiler")]);
        let Rundenherkunft::Parkour {
            anlage_id,
            parkour_id,
            parkour_name,
        } = runde.herkunft()
        else {
            panic!("ein Parkour-Aufbau trägt Referenzen");
        };
        assert_eq!(*anlage_id, AnlageId(Uuid::nil()));
        assert_eq!(*parkour_id, ParkourId(Uuid::from_u128(7)));
        assert_eq!(parkour_name.as_str(), "Waldrunde");
        assert_eq!(runde.aufbau().pflöcke().len(), 1);
        assert_eq!(
            runde.aufbau().ziele()[0].anlage_ziel_id(),
            Some(AnlageZielId(Uuid::from_u128(ANLAGE_ZIEL_BASIS + 10)))
        );
        // The route's own identity is minted by the round, never the master-data one (#12):
        // a leak of `AnlageZielId` into `RundenZielId` fails right here.
        assert_ne!(
            runde.aufbau().ziele()[0].id().0,
            Uuid::from_u128(ANLAGE_ZIEL_BASIS + 10)
        );
    }

    #[test_log::test]
    fn ac21_eine_freie_runde_hat_weder_anlage_noch_parkour_noch_pfloecke() {
        let runde = freie_runde();
        let Rundenherkunft::Freie { name } = runde.herkunft() else {
            panic!("eine freie Runde trägt ihren Aufbau");
        };
        assert_eq!(name.as_str(), "Wiese daheim");
        assert!(runde.aufbau().pflöcke().is_empty());
        assert!(runde.aufbau().ziele().is_empty());
        // Both open states store the same spelling, and the store filters on it (#8).
        assert_eq!(runde.zustand(), ZUSTAND_IN_BEARBEITUNG);
    }

    #[test_log::test]
    fn ac22_freie_ziele_werden_fortlaufend_nummeriert_und_nie_wiederverwendet() {
        let Runde::FreieRundeInArbeit(mut runde) = freie_runde() else {
            panic!("eine freie Runde hat ihren eigenen offenen Typ");
        };
        let erstes = runde.ziel_hinzufügen(None);
        let zweites = runde.ziel_hinzufügen(None);
        let drittes = runde.ziel_hinzufügen(Some(name("Baumstumpf")));

        let namen: Vec<&str> = runde
            .aufbau()
            .ziele()
            .iter()
            .map(|ziel| ziel.name().as_str())
            .collect();
        assert_eq!(namen, ["Ziel 1", "Ziel 2", "Baumstumpf"]);
        assert_eq!(runde.aufbau().ziele()[2].ordinal().get(), 3);

        assert_eq!(
            runde
                .ziel_entfernen(zweites)
                .map(|ziel| ziel.ordinal().get()),
            Some(2)
        );
        // A removed identity is never handed out again: the next Ziel gets a fresh one and
        // the removed one stays unknown to the round.
        let viertes = runde.ziel_hinzufügen(None);
        assert_ne!(viertes, erstes);
        assert_ne!(viertes, drittes);
        assert_ne!(viertes, zweites);
        assert!(!runde.enthält_ziel(zweites));
        let letztes = runde.aufbau().ziele().last().unwrap();
        assert_eq!(letztes.ordinal().get(), 4);
        assert_eq!(letztes.name().as_str(), "Ziel 4");
        assert_eq!(letztes.id(), viertes);
        assert!(
            runde
                .ziel_entfernen(RundenZielId(Uuid::from_u128(99)))
                .is_none()
        );

        let abgeschlossen = runde.abschliessen(jetzt());
        let abgeschlossen: Runde = abgeschlossen.into();
        assert!(!matches!(abgeschlossen, Runde::FreieRundeInArbeit(_)));
    }

    #[test_log::test]
    fn ac22_abschliessen_behaelt_route_und_herkunft() {
        // The compile-time half of the AC-22 promise — a Parkour round has no route mutator
        // — is `tests/compile_fail/parkour_hat_keinen_ziel_mutator.rs`, because "this method
        // does not exist" can only be proven by failing to compile.
        let Runde::InBearbeitung(runde) = runde(&[(10, "Keiler")]) else {
            panic!("eine neue Parkour-Runde ist in Bearbeitung");
        };
        let ziel_id = runde.aufbau().ziele()[0].id();
        let aufbau = runde.aufbau().clone();
        let geschlossen: Runde = runde.abschliessen(jetzt()).into();
        assert_eq!(geschlossen.zustand(), "Abgeschlossen");
        // Closing erases nothing: the route and the provenance are the round's snapshot.
        assert_eq!(geschlossen.aufbau(), &aufbau);
        assert!(geschlossen.enthält_ziel(ziel_id));
        assert!(matches!(
            geschlossen.herkunft(),
            Rundenherkunft::Parkour { .. }
        ));
        assert!(!geschlossen.ist_freie_runde());
    }

    #[test_log::test]
    fn ac3_ziele_werden_in_einer_round_localen_route_abgebildet() {
        let Runde::FreieRundeInArbeit(mut freie) = freie_runde() else {
            panic!("eine freie Runde hat ihren eigenen offenen Typ");
        };
        let id = freie.ziel_hinzufügen(None);
        let ziele = freie.aufbau().ziele();
        assert_eq!(ziele.len(), 1);
        assert_eq!(ziele[0].id(), id);
        assert_eq!(ziele[0].name().as_str(), "Ziel 1");
        assert_eq!(ziele[0].ordinal().get(), 1);
        assert!(ziele[0].abstände().is_empty());

        let parkour = runde(&[(10, "Keiler")]);
        let ziel = &parkour.aufbau().ziele()[0];
        assert_eq!(ziel.name().as_str(), "Keiler");
        assert_eq!(ziel.ordinal().get(), 1);
        assert_eq!(
            ziel.anlage_ziel_id(),
            Some(AnlageZielId(Uuid::from_u128(ANLAGE_ZIEL_BASIS + 10)))
        );
    }

    #[test_log::test]
    fn ac23_parkour_pflock_wird_gegen_das_ziel_geprueft() {
        let runde = runde(&[(10, "Keiler")]);
        let ziel = runde.aufbau().ziele()[0].id();
        let farbe = Farbe::neu("#c0392b").unwrap();
        assert!(
            runde
                .pruefe_position(ziel, Some(Schussposition::Pflock(farbe)))
                .is_ok()
        );
        assert!(
            runde
                .pruefe_position(
                    ziel,
                    Some(Schussposition::Distanz(Abstand::neu(35).unwrap()))
                )
                .is_ok()
        );
        assert!(
            runde
                .pruefe_position(
                    ziel,
                    Some(Schussposition::Pflock(Farbe::neu("#e8b81c").unwrap()))
                )
                .is_err(),
            "ein fremder Pflock ist keine Position dieser Runde"
        );
        // A Ziel the route never captured is refused before the Pflock is looked at.
        assert!(
            runde
                .pruefe_position(
                    RundenZielId(Uuid::from_u128(99)),
                    Some(Schussposition::Pflock(farbe))
                )
                .is_err()
        );
    }

    #[test_log::test]
    fn ac23_eine_freie_runde_nimmt_jede_farbe_und_jede_distanz() {
        let Runde::FreieRundeInArbeit(mut runde) = freie_runde() else {
            panic!("eine freie Runde hat ihren eigenen offenen Typ");
        };
        let ziel = runde.ziel_hinzufügen(None);
        // A free round configures no Pflock set, so the colour the archer picked is a
        // position; `Farbe` has already proven it is a colour.
        for hex in ["#c0392b", "#1b4f9c", "#abc123"] {
            assert!(
                runde
                    .pruefe_position(ziel, Some(Schussposition::Pflock(Farbe::neu(hex).unwrap())))
                    .is_ok()
            );
        }
        assert!(
            runde
                .pruefe_position(
                    ziel,
                    Some(Schussposition::Distanz(Abstand::neu(35).unwrap()))
                )
                .is_ok()
        );
        assert!(runde.pruefe_position(ziel, None).is_ok());
    }

    #[test_log::test]
    fn ac24_die_wertung_faltet_ueber_die_freien_ziele() {
        let Runde::FreieRundeInArbeit(mut runde) = freie_runde() else {
            panic!("eine freie Runde hat ihren eigenen offenen Typ");
        };
        let erstes = runde.ziel_hinzufügen(None);
        let zweites = runde.ziel_hinzufügen(None);
        let schüsse = vec![
            schuss_auf(runde.id(), erstes, 1, Wertungszone::Kill),
            schuss_auf(runde.id(), erstes, 2, Wertungszone::Spot),
            schuss_auf(runde.id(), zweites, 1, Wertungszone::Miss),
        ];
        let wertung = runde.wertung(schüsse.clone());
        assert_eq!(wertung.erster_treffer(), 18);
        assert_eq!(wertung.mehrschuss(), 32);
        assert_eq!(wertung.ziele_mit_wertung(), 2);

        // Removing a Ziel drops its result from the fold (AC-22/AC-24).
        runde.ziel_entfernen(zweites);
        let wertung = runde.wertung(schüsse);
        assert_eq!(wertung.ziele_mit_wertung(), 1);
    }
}
