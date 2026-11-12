//! One logged Runde, holding the setup it was started from.
//!
//! A `Runde` is a standalone aggregate root. It never holds a collection of `Schuss`:
//! each `Schuss` carries `runde_id`, so two aggregates stay two aggregates and no use case
//! writes both. The Runde-wide Wertung folds the per-Ziel results once, in the order of the
//! captured route (SPEC-wertung rule 6, AC-8).
//!
//! The root is a **typestate**: only `RundeIn<InBearbeitung>` can be closed, and the closing
//! timestamp lives in the `Abgeschlossen` marker, so "when was it closed?" is not an
//! `Option` a reader must interpret.
//!
//! The setup records Parkour provenance when present, over a single round-owned route of
//! neutral target identities. Route mutations are exposed only by the running Runde state.

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

/// Lifecycle markers of the `Runde` typestate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InBearbeitung;

/// The free-round state permits just-in-time route edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FreieRundeInBearbeitung;

impl FreieRundeInBearbeitung {
    pub fn as_str(self) -> &'static str {
        "InBearbeitung"
    }

    pub fn ist_in_bearbeitung(self) -> bool {
        true
    }
}

/// The closed state carries when it happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Abgeschlossen {
    abgeschlossen_am: DateTime<Utc>,
}

impl Abgeschlossen {
    pub fn abgeschlossen_am(self) -> DateTime<Utc> {
        self.abgeschlossen_am
    }
}

// The typestate markers carry their own stored spelling: the two markers are the whole
// lifecycle, so an adapter never needs a second, parallel enum that restates them.
impl InBearbeitung {
    pub fn as_str(self) -> &'static str {
        "InBearbeitung"
    }

    pub fn ist_in_bearbeitung(self) -> bool {
        true
    }
}

impl Abgeschlossen {
    pub fn as_str(self) -> &'static str {
        "Abgeschlossen"
    }

    pub fn ist_in_bearbeitung(self) -> bool {
        false
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
    pub fn ziel_id(&self) -> RundenZielId {
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

/// The common route snapshot. Both setup kinds use the same round-local targets and
/// capture/read model; only the Parkour setup carries a configured stake set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RundenSnapshot {
    pflöcke: Vec<PflockSnapshot>,
    ziele: Vec<RundenZiel>,
}

impl RundenSnapshot {
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
}

/// The setup of a Runde: Anlage/Parkour provenance or a free-round name, over one common route.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rundenaufbau {
    Parkour {
        anlage_id: AnlageId,
        parkour_id: ParkourId,
        parkour_name: NonEmptyString,
        snapshot: RundenSnapshot,
    },
    Freie {
        name: NonEmptyString,
        snapshot: RundenSnapshot,
    },
}

#[derive(Debug, thiserror::Error)]
#[error("der Pflock gehört nicht zum Aufbau dieser Runde")]
pub struct PositionNichtImAufbau;

impl Rundenaufbau {
    pub fn parkour(anlage_id: AnlageId, snapshot: ParkourSnapshot) -> Self {
        Self::Parkour {
            anlage_id,
            parkour_id: snapshot.parkour_id(),
            parkour_name: snapshot.parkour_name().clone(),
            snapshot: RundenSnapshot::from_parkour(&snapshot),
        }
    }

    pub fn snapshot(&self) -> &RundenSnapshot {
        match self {
            Self::Parkour { snapshot, .. } | Self::Freie { snapshot, .. } => snapshot,
        }
    }

    pub fn parkour_name(&self) -> Option<&NonEmptyString> {
        match self {
            Self::Parkour { parkour_name, .. } => Some(parkour_name),
            _ => None,
        }
    }

    pub fn name(&self) -> &NonEmptyString {
        match self {
            Self::Parkour { parkour_name, .. } => parkour_name,
            Self::Freie { name, .. } => name,
        }
    }

    pub fn ziele(&self) -> impl Iterator<Item = &RundenZiel> {
        self.snapshot().ziele().iter()
    }

    pub fn ziel_id_für_schuss(&self, ziel_id: SchussZielId) -> Option<RundenZielId> {
        self.snapshot()
            .ziele()
            .iter()
            .find(|ziel| ziel.id.0 == ziel_id.0)
            .map(RundenZiel::id)
    }

    pub fn enthält_ziel(&self, ziel_id: RundenZielId) -> bool {
        self.snapshot().enthält_ziel(ziel_id)
    }

    pub fn pruefe_position(
        &self,
        ziel_id: RundenZielId,
        position: Option<Schussposition>,
    ) -> Result<(), PositionNichtImAufbau> {
        if !self.enthält_ziel(ziel_id) {
            return Err(PositionNichtImAufbau);
        }
        if !self.enthält_ziel(ziel_id) {
            return Err(PositionNichtImAufbau);
        }
        if let Some(Schussposition::Pflock(farbe)) = position {
            match self {
                Self::Parkour { snapshot, .. } if snapshot.hat_pflock_am_ziel(ziel_id, farbe) => {
                    Ok(())
                }
                Self::Freie { .. } => Ok(()),
                _ => Err(PositionNichtImAufbau),
            }
        } else {
            Ok(())
        }
    }

    fn farben(&self, schüsse: impl IntoIterator<Item = Schuss>) -> Vec<Farbe> {
        let freie = matches!(self, Self::Freie { .. });
        let mut farben = Vec::new();
        if freie {
            for schuss in schüsse {
                if self.ziel_id_für_schuss(schuss.ziel_id()).is_some()
                    && let Some(Schussposition::Pflock(farbe)) = schuss.position()
                    && !farben.contains(&farbe)
                {
                    farben.push(farbe);
                }
            }
        }
        farben
    }
}

// ---------------------------------------------------------------------------------------
// Creation input
// ---------------------------------------------------------------------------------------

/// Creation input of a Runde: everything except its identity and lifecycle state.
///
/// A variant, not a bag of optional fields, so neither setup can be built half-way: the
/// `Rundenaufbau` follows from the variant (SPEC-runde rule 17).
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

    /// The setup this input describes, built once so the pair cannot be split.
    pub fn aufbau(&self) -> Rundenaufbau {
        match self {
            Self::Parkour {
                anlage_id,
                snapshot,
                ..
            } => Rundenaufbau::Parkour {
                anlage_id: *anlage_id,
                parkour_id: snapshot.parkour_id(),
                parkour_name: snapshot.parkour_name().clone(),
                snapshot: RundenSnapshot::from_parkour(snapshot),
            },
            Self::Freie { name, .. } => Rundenaufbau::Freie {
                name: name.clone(),
                snapshot: RundenSnapshot::from_parts(Vec::new(), Vec::new()),
            },
        }
    }
}

// ---------------------------------------------------------------------------------------
// The aggregate
// ---------------------------------------------------------------------------------------

/// One logged Runde: who, the frozen setup, and whether it is still running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RundeIn<S> {
    id: RundeId,
    erfasst_von: Erfasser,
    aufbau: Rundenaufbau,
    begonnen_am: DateTime<Utc>,
    state: S,
}

pub type RundeInBearbeitung = RundeIn<InBearbeitung>;
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

    /// Whether the setup contains this Ziel (the AC-9 contract a Schuss obeys).
    pub fn enthält_ziel(&self, ziel_id: RundenZielId) -> bool {
        self.aufbau.enthält_ziel(ziel_id)
    }

    /// Replace the owner with [`Erfasser::Anonymisiert`] (SPEC-datenschutz retention).
    /// The setup and every Schuss stay untouched (NFR-3).
    pub fn anonymisiere(&mut self, am: DateTime<Utc>) {
        self.erfasst_von = Erfasser::Anonymisiert { am };
    }

    /// Fold the given Schüsse into one Wertung per Ziel, in visiting order.
    ///
    /// `None` marks a Ziel no Schuss was logged at (AC-6). The slice pairs with the setup's
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
            .map(|ziel| {
                // The filter names exactly this Ziel, so the collection cannot refuse a row.
                let ziel_id = ziel.ziel_id();
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
        let je_ziel = self.aufbau.ziele().filter_map(|ziel| {
            // The filter names exactly this Ziel, so the collection cannot refuse a row here.
            let ziel_id = ziel.ziel_id();
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

impl RundeInBearbeitung {
    fn aus_neu(id: RundeId, neu: NeueRunde) -> Self {
        Self {
            id,
            erfasst_von: neu.erfasst_von(),
            aufbau: neu.aufbau(),
            begonnen_am: neu.begonnen_am(),
            state: InBearbeitung,
        }
    }

    fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            erfasst_von: erfasser,
            aufbau,
            begonnen_am,
            state: InBearbeitung,
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
                abgeschlossen_am: am,
            },
        }
    }
}

impl FreieRundeInArbeit {
    pub fn aus_neu(id: RundeId, neu: NeueRunde) -> Self {
        Self {
            id,
            erfasst_von: neu.erfasst_von(),
            aufbau: neu.aufbau(),
            begonnen_am: neu.begonnen_am(),
            state: FreieRundeInBearbeitung,
        }
    }

    pub fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            erfasst_von: erfasser,
            aufbau,
            begonnen_am,
            state: FreieRundeInBearbeitung,
        }
    }

    pub fn ziel_hinzufügen(&mut self, id: RundenZielId, name: Option<NonEmptyString>) -> bool {
        let Rundenaufbau::Freie { snapshot, .. } = &mut self.aufbau else {
            unreachable!("free-round typestate always carries free setup")
        };
        if snapshot.enthält_ziel(id) {
            return false;
        }
        snapshot.ziel_hinzufügen(id, name);
        true
    }

    pub fn ziel_entfernen(&mut self, id: RundenZielId) -> Option<RundenZiel> {
        let Rundenaufbau::Freie { snapshot, .. } = &mut self.aufbau else {
            unreachable!("free-round typestate always carries free setup")
        };
        snapshot.ziel_entfernen(id)
    }

    pub fn ziel_umbenennen(&mut self, id: RundenZielId, name: NonEmptyString) -> bool {
        let Rundenaufbau::Freie { snapshot, .. } = &mut self.aufbau else {
            unreachable!("free-round typestate always carries free setup")
        };
        let Some(ziel) = snapshot.ziel_mut(id) else {
            return false;
        };
        ziel.umbenennen(name);
        true
    }

    pub fn abschliessen(self, am: DateTime<Utc>) -> RundeAbgeschlossen {
        RundeIn {
            id: self.id,
            erfasst_von: self.erfasst_von,
            aufbau: self.aufbau,
            begonnen_am: self.begonnen_am,
            state: Abgeschlossen {
                abgeschlossen_am: am,
            },
        }
    }
}

impl RundeAbgeschlossen {
    /// Rebuild a persisted, closed Runde.
    pub fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
        abgeschlossen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            erfasst_von: erfasser,
            aufbau,
            begonnen_am,
            state: Abgeschlossen { abgeschlossen_am },
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
    /// Rebuild a persisted Runde in either state.
    ///
    /// The closing timestamp *is* the state, so the adapter passes what it read and the
    /// typestate follows: `None` is a running Runde, `Some` a closed one.
    pub fn aus_neu(id: RundeId, neu: NeueRunde) -> Self {
        match neu {
            parkour @ NeueRunde::Parkour { .. } => {
                Self::InBearbeitung(RundeInBearbeitung::aus_neu(id, parkour))
            }
            freie @ NeueRunde::Freie { .. } => {
                Self::FreieRundeInArbeit(FreieRundeInArbeit::aus_neu(id, freie))
            }
        }
    }

    pub fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
        abgeschlossen_am: Option<DateTime<Utc>>,
    ) -> Self {
        match (aufbau, abgeschlossen_am) {
            (aufbau @ Rundenaufbau::Parkour { .. }, None) => Self::InBearbeitung(
                RundeInBearbeitung::from_parts(id, erfasser, aufbau, begonnen_am),
            ),
            (aufbau @ Rundenaufbau::Freie { .. }, None) => Self::FreieRundeInArbeit(
                FreieRundeInArbeit::from_parts(id, erfasser, aufbau, begonnen_am),
            ),
            (aufbau, Some(am)) => Self::Abgeschlossen(RundeAbgeschlossen::from_parts(
                id,
                erfasser,
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

    pub fn farben(&self, schüsse: impl IntoIterator<Item = Schuss>) -> Vec<Farbe> {
        match self {
            Self::InBearbeitung(runde) => runde.aufbau.farben(schüsse),
            Self::FreieRundeInArbeit(runde) => runde.aufbau.farben(schüsse),
            Self::Abgeschlossen(runde) => runde.aufbau.farben(schüsse),
        }
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

    pub fn ist_freie_runde(&self) -> bool {
        matches!(
            self,
            Self::FreieRundeInArbeit(_)
                | Self::Abgeschlossen(RundeIn {
                    aufbau: Rundenaufbau::Freie { .. },
                    ..
                })
        )
    }

    pub fn enthält_ziel(&self, ziel_id: RundenZielId) -> bool {
        match self {
            Self::InBearbeitung(runde) => runde.enthält_ziel(ziel_id),
            Self::FreieRundeInArbeit(runde) => runde.enthält_ziel(ziel_id),
            Self::Abgeschlossen(runde) => runde.enthält_ziel(ziel_id),
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

    fn snapshot(ziele: &[(u128, &str)]) -> ParkourSnapshot {
        ParkourSnapshot::from_parts(
            ParkourId(Uuid::from_u128(7)),
            name("Waldrunde"),
            vec![PflockSnapshot::neu(Farbe::neu("#c0392b").unwrap(), 0)],
            ziele
                .iter()
                .map(|&(id, tiername)| {
                    ZielSnapshot::neu(
                        AnlageZielId(Uuid::from_u128(id)),
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

    fn runde(ziele: &[(u128, &str)]) -> Runde {
        let aufbau = Rundenaufbau::parkour(AnlageId(Uuid::nil()), snapshot(ziele));
        let Rundenaufbau::Parkour { snapshot, .. } = aufbau else {
            unreachable!();
        };
        let mut snapshot = snapshot;
        for ziel in &mut snapshot.ziele {
            ziel.id = RundenZielId(ziel.anlage_ziel_id.expect("test target provenance").0);
        }
        Runde::InBearbeitung(RundeInBearbeitung::from_parts(
            RundeId(Uuid::nil()),
            Erfasser::Benutzer(BenutzerId(Uuid::nil())),
            Rundenaufbau::Parkour {
                anlage_id: AnlageId(Uuid::nil()),
                parkour_id: ParkourId(Uuid::from_u128(7)),
                parkour_name: name("Waldrunde"),
                snapshot,
            },
            jetzt(),
        ))
    }

    fn freie_runde() -> Runde {
        Runde::FreieRundeInArbeit(FreieRundeInArbeit::aus_neu(
            RundeId(Uuid::nil()),
            NeueRunde::ohne_parkour(
                name("Wiese daheim"),
                Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                jetzt(),
            ),
        ))
    }

    fn schuss(ziel: u128, pfeil: usize, zone: Wertungszone) -> Schuss {
        Schuss::neu(
            SchussId(Uuid::from_u128(pfeil as u128)),
            NeuerSchuss::neu(
                RundeId(Uuid::nil()),
                Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                RundenZielId(Uuid::from_u128(ziel)).into(),
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
            schuss(10, 1, Wertungszone::Kill),
            schuss(10, 2, Wertungszone::Kill),
            schuss(10, 3, Wertungszone::Spot),
            Schuss::neu(
                SchussId(Uuid::from_u128(11)),
                NeuerSchuss::neu(
                    RundeId(Uuid::nil()),
                    Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                    RundenZielId(Uuid::from_u128(11)).into(),
                    2,
                    Wertungszone::Spot,
                    None,
                )
                .unwrap(),
            ),
        ];
        let wertung = runde.wertung(schüsse);
        assert_eq!(wertung.erster_treffer(), 18 + 14);
        assert_eq!(wertung.mehrschuss(), 38 + 14);
        assert_eq!(wertung.ziele_mit_wertung(), 2);
    }

    #[test_log::test]
    fn ac15_zielwertungen_zeigen_leerzustand_und_fortschritt_aus_schuessen() {
        let runde = runde(&[(10, "Keiler"), (11, "Reh")]);
        let schüsse = vec![schuss(10, 1, Wertungszone::Kill)];
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
        let wertung = runde.wertung(vec![schuss(11, 1, Wertungszone::Kill)]);
        assert_eq!(wertung.erster_treffer(), 18);
        assert_eq!(wertung.mehrschuss(), 18);
        assert_eq!(wertung.ziele_mit_wertung(), 1);
    }

    #[test_log::test]
    fn ac8_fremde_schüsse_zaehlen_nicht() {
        let runde = runde(&[(10, "Keiler")]);
        let mut fremd = Schuss::neu(
            SchussId(Uuid::from_u128(2)),
            NeuerSchuss::neu(
                RundeId(Uuid::from_u128(99)),
                Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                RundenZielId(Uuid::from_u128(10)).into(),
                1,
                Wertungszone::Spot,
                None,
            )
            .unwrap(),
        );
        fremd.anonymisiere(jetzt());
        let wertung = runde.wertung(vec![
            schuss(10, 1, Wertungszone::Kill),
            fremd,
            schuss(99, 1, Wertungszone::Spot),
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
    fn ac9_der_snapshot_kennt_nur_die_platzierte_route() {
        let runde = runde(&[(10, "Keiler")]);
        let ziel_id = runde.aufbau().ziele().next().unwrap().id();
        assert!(runde.enthält_ziel(ziel_id));
        assert!(!runde.enthält_ziel(RundenZielId(Uuid::from_u128(11))));
    }

    #[test_log::test]
    fn anonymisieren_laesst_den_aufbau_unberuehrt() {
        let mut runde = runde(&[(10, "Keiler")]);
        let vorher = runde.aufbau().clone();
        runde.anonymisiere(jetzt());
        assert_eq!(runde.erfasst_von(), Erfasser::Anonymisiert { am: jetzt() });
        assert_eq!(runde.aufbau(), &vorher);
    }

    #[test_log::test]
    fn from_parts_roundtrip_beider_aufbauten_und_zustaende() {
        let offen = runde(&[(10, "Keiler")]);
        let geladen = Runde::from_parts(
            offen.id(),
            offen.erfasst_von(),
            offen.aufbau().clone(),
            offen.begonnen_am(),
            offen.abgeschlossen_am(),
        );
        assert_eq!(offen, geladen);

        let freie = freie_runde();
        let geladen = Runde::from_parts(
            freie.id(),
            freie.erfasst_von(),
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
            geschlossen.aufbau().clone(),
            geschlossen.begonnen_am(),
            geschlossen.abgeschlossen_am(),
        );
        assert_eq!(geschlossen, geladen);
    }

    #[test_log::test]
    fn ein_abschlusszeitpunkt_macht_die_geladene_runde_abgeschlossen() {
        // The timestamp *is* the state: there is no second flag that could disagree with it.
        let offen = runde(&[(10, "Keiler")]);
        let geladen = Runde::from_parts(
            offen.id(),
            offen.erfasst_von(),
            offen.aufbau().clone(),
            offen.begonnen_am(),
            Some(jetzt()),
        );
        assert_eq!(geladen.zustand(), "Abgeschlossen");
        assert_eq!(geladen.abgeschlossen_am(), Some(jetzt()));
    }

    #[test_log::test]
    fn ac21_ein_parkour_aufbau_traegt_seine_provenienz_und_den_gemeinsamen_snapshot() {
        let runde = runde(&[(10, "Keiler")]);
        let Rundenaufbau::Parkour {
            anlage_id,
            parkour_id,
            parkour_name,
            snapshot,
        } = runde.aufbau()
        else {
            panic!("ein Parkour-Aufbau trägt Referenzen");
        };
        assert_eq!(*anlage_id, AnlageId(Uuid::nil()));
        assert_eq!(*parkour_id, ParkourId(Uuid::from_u128(7)));
        assert_eq!(parkour_name.as_str(), "Waldrunde");
        assert_eq!(snapshot.pflöcke().len(), 1);
        assert_eq!(
            snapshot.ziele()[0].anlage_ziel_id(),
            Some(AnlageZielId(Uuid::from_u128(10)))
        );
        assert_eq!(snapshot.ziele()[0].id().0, Uuid::from_u128(10));
    }

    #[test_log::test]
    fn ac21_eine_freie_runde_hat_weder_anlage_noch_parkour_noch_pfloecke() {
        let runde = freie_runde();
        let Rundenaufbau::Freie { name, snapshot } = runde.aufbau() else {
            panic!("eine freie Runde trägt ihren Aufbau");
        };
        assert_eq!(name.as_str(), "Wiese daheim");
        assert!(snapshot.pflöcke().is_empty());
        assert!(snapshot.ziele().is_empty());
    }

    #[test_log::test]
    fn ac22_freie_ziele_werden_fortlaufend_nummeriert_und_nie_wiederverwendet() {
        let Runde::FreieRundeInArbeit(mut runde) = freie_runde() else {
            panic!("eine freie Runde hat ihren eigenen offenen Typ");
        };
        let erstes = RundenZielId(Uuid::from_u128(1));
        let zweites = RundenZielId(Uuid::from_u128(2));
        assert!(runde.ziel_hinzufügen(erstes, None));
        assert!(runde.ziel_hinzufügen(zweites, None));
        assert!(runde.ziel_hinzufügen(RundenZielId(Uuid::from_u128(3)), Some(name("Baumstumpf"))));

        let namen: Vec<&str> = runde
            .aufbau()
            .ziele()
            .map(|ziel| ziel.name().as_str())
            .collect();
        assert_eq!(namen, ["Ziel 1", "Ziel 2", "Baumstumpf"]);
        assert_eq!(runde.aufbau().snapshot().ziele()[2].ordinal().get(), 3);

        assert_eq!(
            runde
                .ziel_entfernen(zweites)
                .map(|ziel| ziel.ordinal().get()),
            Some(2)
        );
        assert!(runde.ziel_hinzufügen(RundenZielId(Uuid::from_u128(4)), None));
        let letztes = runde.aufbau().snapshot().ziele().last().unwrap();
        assert_eq!(letztes.ordinal().get(), 4);
        assert_eq!(letztes.name().as_str(), "Ziel 4");
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
    fn ac22_ein_parkour_aufbau_hat_keinen_ziel_mutator() {
        // The compile-time half of the promise: this block deliberately does not compile,
        // because `ziel_hinzufügen` exists only on `FreieRunde`.
        //
        // ```compile_fail
        // let mut aufbau = Rundenaufbau::parkour(AnlageId(uuid::Uuid::nil()), snapshot);
        // aufbau.ziel_hinzufügen(AnlageZielId(uuid::Uuid::nil()), None);
        // ```
        let aufbau = rundenaufbau();
        assert!(matches!(aufbau, Rundenaufbau::Parkour { .. }));
    }

    fn rundenaufbau() -> Rundenaufbau {
        Rundenaufbau::parkour(AnlageId(Uuid::nil()), snapshot(&[(10, "Keiler")]))
    }

    #[test_log::test]
    fn ac3_ziele_werden_in_einer_round_localen_route_abgebildet() {
        let Runde::FreieRundeInArbeit(mut runde) = freie_runde() else {
            panic!("eine freie Runde hat ihren eigenen offenen Typ");
        };
        let id = RundenZielId(Uuid::from_u128(5));
        assert!(runde.ziel_hinzufügen(id, None));
        let ziele: Vec<_> = runde.aufbau().ziele().collect();
        assert_eq!(ziele.len(), 1);
        assert_eq!(ziele[0].id(), id);
        assert_eq!(ziele[0].name().as_str(), "Ziel 1");
        assert_eq!(ziele[0].ordinal().get(), 1);
        assert!(ziele[0].abstände().is_empty());

        let parkour = rundenaufbau();
        let ziel = parkour.ziele().next().unwrap();
        assert_eq!(ziel.name().as_str(), "Keiler");
        assert_eq!(ziel.ordinal().get(), 1);
        assert_eq!(
            ziel.anlage_ziel_id(),
            Some(AnlageZielId(Uuid::from_u128(10)))
        );
    }

    #[test_log::test]
    fn ac23_parkour_pflock_wird_gegen_das_ziel_geprueft() {
        let aufbau = rundenaufbau();
        let ziel = aufbau.ziele().next().unwrap().id();
        let farbe = Farbe::neu("#c0392b").unwrap();
        assert!(
            aufbau
                .pruefe_position(ziel, Some(Schussposition::Pflock(farbe)))
                .is_ok()
        );
        assert!(
            aufbau
                .pruefe_position(
                    ziel,
                    Some(Schussposition::Distanz(Abstand::neu(35).unwrap()))
                )
                .is_ok()
        );
        assert!(
            aufbau
                .pruefe_position(
                    ziel,
                    Some(Schussposition::Pflock(Farbe::neu("#e8b81c").unwrap()))
                )
                .is_err(),
            "ein fremder Pflock ist keine Position dieser Runde"
        );
        assert!(
            aufbau
                .pruefe_position(ziel, Some(Schussposition::Pflock(farbe)))
                .is_ok()
        );
    }

    #[test_log::test]
    fn ac23_eine_freie_runde_nimmt_jede_farbe_und_jede_distanz() {
        let Runde::FreieRundeInArbeit(mut runde) = freie_runde() else {
            panic!("eine freie Runde hat ihren eigenen offenen Typ");
        };
        let ziel = RundenZielId(Uuid::from_u128(1));
        assert!(runde.ziel_hinzufügen(ziel, None));
        let aufbau = runde.aufbau().clone();
        // A free round configures no Pflock set, so the colour the archer picked is a
        // position; `Farbe` has already proven it is a colour.
        for hex in ["#c0392b", "#1b4f9c", "#abc123"] {
            assert!(
                aufbau
                    .pruefe_position(ziel, Some(Schussposition::Pflock(Farbe::neu(hex).unwrap())))
                    .is_ok()
            );
        }
        assert!(
            aufbau
                .pruefe_position(
                    ziel,
                    Some(Schussposition::Distanz(Abstand::neu(35).unwrap()))
                )
                .is_ok()
        );
        assert!(aufbau.pruefe_position(ziel, None).is_ok());
    }

    #[test_log::test]
    fn ac23_farben_sind_die_bereits_verwendeten_farben() {
        let Runde::FreieRundeInArbeit(mut runde) = freie_runde() else {
            panic!("eine freie Runde hat ihren eigenen offenen Typ");
        };
        let ziel_id = RundenZielId(Uuid::from_u128(1));
        assert!(runde.ziel_hinzufügen(ziel_id, None));
        let rot = Farbe::neu("#c0392b").unwrap();
        let blau = Farbe::neu("#1b4f9c").unwrap();
        let schüsse = vec![
            Schuss::neu(
                SchussId(Uuid::from_u128(1)),
                NeuerSchuss::neu(
                    RundeId(Uuid::nil()),
                    Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                    ziel_id.into(),
                    1,
                    Wertungszone::Kill,
                    Some(Schussposition::Pflock(rot)),
                )
                .unwrap(),
            ),
            Schuss::neu(
                SchussId(Uuid::from_u128(2)),
                NeuerSchuss::neu(
                    RundeId(Uuid::nil()),
                    Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                    ziel_id.into(),
                    2,
                    Wertungszone::Spot,
                    Some(Schussposition::Pflock(rot)),
                )
                .unwrap(),
            ),
            Schuss::neu(
                SchussId(Uuid::from_u128(3)),
                NeuerSchuss::neu(
                    RundeId(Uuid::nil()),
                    Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                    ziel_id.into(),
                    3,
                    Wertungszone::Spot,
                    Some(Schussposition::Distanz(Abstand::neu(35).unwrap())),
                )
                .unwrap(),
            ),
        ];
        assert_eq!(
            Runde::FreieRundeInArbeit(runde.clone()).farben(schüsse),
            vec![rot]
        );
        assert!(
            !Runde::FreieRundeInArbeit(runde)
                .farben(Vec::new())
                .contains(&blau)
        );
    }

    #[test_log::test]
    fn ac24_die_wertung_faltet_ueber_die_freien_ziele() {
        let Runde::FreieRundeInArbeit(mut runde) = freie_runde() else {
            panic!("eine freie Runde hat ihren eigenen offenen Typ");
        };
        assert!(runde.ziel_hinzufügen(RundenZielId(Uuid::from_u128(1)), None));
        assert!(runde.ziel_hinzufügen(RundenZielId(Uuid::from_u128(2)), None));
        let schüsse = vec![
            schuss(1, 1, Wertungszone::Kill),
            schuss(1, 2, Wertungszone::Spot),
            schuss(2, 1, Wertungszone::Miss),
        ];
        let wertung = runde.wertung(schüsse.clone());
        assert_eq!(wertung.erster_treffer(), 18);
        assert_eq!(wertung.mehrschuss(), 32);
        assert_eq!(wertung.ziele_mit_wertung(), 2);

        // Removing a Ziel drops its result from the fold (AC-22/AC-24).
        runde.ziel_entfernen(RundenZielId(Uuid::from_u128(2)));
        let wertung = runde.wertung(schüsse);
        assert_eq!(wertung.ziele_mit_wertung(), 1);
    }
}
