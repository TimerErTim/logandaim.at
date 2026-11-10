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
//! The setup is a typestate too: [`Rundenaufbau`] is either a master-data `Parkour` with its
//! one [`RundeOutgoingReferences`] pair, or a [`FreieRunde`] the archer owns (SPEC-runde rule
//! 17). The variant *is* the invariant — there is no `Option<AnlageId>` next to an
//! `Option<ParkourId>` that could disagree, and only `FreieRunde` has `ziel_hinzufügen` or
//! `ziel_entfernen`, so editing a Parkour round's Ziel list is a compile error.

use std::num::NonZeroU32;

use chrono::{DateTime, Utc};

use super::anlage::{AnlageId, ParkourId, ParkourSnapshot, PflockSnapshot, ZielId, ZielSnapshot};
use super::common::{Abstand, Farbe, NonEmptyString};
use super::konto::BenutzerId;
use super::schuss::{Erfasser, Schuss, Schussposition, ZielSchüsse, Zielwertung};

/// Identity of one Runde. Re-exported where a `Schuss` needs it.
pub use super::schuss::RundeId;

/// Lifecycle markers of the `Runde` typestate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InBearbeitung;

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
// The setup
// ---------------------------------------------------------------------------------------

/// The master-data identities a Runde started from.
///
/// A Parkour belongs to exactly one Anlage, so the two are one value: two separate
/// `Option`s could disagree, and "parkour without anlage" would be representable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RundeOutgoingReferences {
    anlage_id: AnlageId,
    parkour_id: ParkourId,
}

impl RundeOutgoingReferences {
    /// Derive the Parkour from the snapshot, so the pair can never name different Parkours.
    pub fn aus_snapshot(anlage_id: AnlageId, snapshot: &ParkourSnapshot) -> Self {
        Self {
            anlage_id,
            parkour_id: snapshot.parkour_id(),
        }
    }

    /// Rebuild a stored pair. The repository reads both ids from the stored setup itself,
    /// never from a denormalised column that a hand edit could split.
    pub fn from_parts(anlage_id: AnlageId, parkour_id: ParkourId) -> Self {
        Self {
            anlage_id,
            parkour_id,
        }
    }

    pub fn anlage_id(&self) -> AnlageId {
        self.anlage_id
    }

    pub fn parkour_id(&self) -> ParkourId {
        self.parkour_id
    }
}

/// The setup a Runde captured: a master-data Parkour or an ad-hoc round the archer owns.
///
/// The variant *is* the invariant: only `Parkour` carries master-data references, and only
/// `Freie` can have its Ziel list edited (SPEC-runde rules 17–18).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rundenaufbau {
    Parkour {
        referenzen: RundeOutgoingReferences,
        snapshot: ParkourSnapshot,
    },
    Freie(FreieRunde),
}

/// The one refusal [`Rundenaufbau::pruefe_position`] can answer.
#[derive(Debug, thiserror::Error)]
#[error("der Pflock gehört nicht zum Aufbau dieser Runde")]
pub struct PositionNichtImAufbau;

impl Rundenaufbau {
    /// Capture a Parkour setup; the references are derived from the snapshot.
    pub fn parkour(anlage_id: AnlageId, snapshot: ParkourSnapshot) -> Self {
        Self::Parkour {
            referenzen: RundeOutgoingReferences::aus_snapshot(anlage_id, &snapshot),
            snapshot,
        }
    }

    /// The master-data pair, `Some` only for a Parkour round.
    pub fn referenzen(&self) -> Option<&RundeOutgoingReferences> {
        match self {
            Self::Parkour { referenzen, .. } => Some(referenzen),
            Self::Freie(_) => None,
        }
    }

    pub fn snapshot(&self) -> Option<&ParkourSnapshot> {
        match self {
            Self::Parkour { snapshot, .. } => Some(snapshot),
            Self::Freie(_) => None,
        }
    }

    pub fn freie(&self) -> Option<&FreieRunde> {
        match self {
            Self::Parkour { .. } => None,
            Self::Freie(freie) => Some(freie),
        }
    }

    pub fn freie_mut(&mut self) -> Option<&mut FreieRunde> {
        match self {
            Self::Parkour { .. } => None,
            Self::Freie(freie) => Some(freie),
        }
    }

    /// The round's own name: the captured Parkour's, or the free round's.
    pub fn name(&self) -> &NonEmptyString {
        match self {
            Self::Parkour { snapshot, .. } => snapshot.parkour_name(),
            Self::Freie(freie) => freie.name(),
        }
    }

    /// The captured Parkour-wide Pflöcke; a free round has none.
    pub fn pflöcke(&self) -> &[PflockSnapshot] {
        match self {
            Self::Parkour { snapshot, .. } => snapshot.pflöcke(),
            Self::Freie(_) => &[],
        }
    }

    /// The Ziele the round walks, in visiting order, as the one view the fold needs.
    pub fn ziele(&self) -> impl Iterator<Item = ZielRef<'_>> {
        let parkour: Box<dyn Iterator<Item = ZielRef<'_>>> = match self {
            Self::Parkour { snapshot, .. } => {
                Box::new(snapshot.ziele().iter().map(ZielRef::Parkour))
            }
            Self::Freie(freie) => Box::new(freie.ziele().iter().map(ZielRef::Freie)),
        };
        parkour
    }

    pub fn enthält_ziel(&self, ziel_id: ZielId) -> bool {
        match self {
            Self::Parkour { snapshot, .. } => snapshot.enthält_ziel(ziel_id),
            Self::Freie(freie) => freie.enthält_ziel(ziel_id),
        }
    }

    /// Whether a Schuss may be stored with this position (SPEC-runde rule 19).
    ///
    /// A Parkour round resolves a Pflock against that Ziel's effective captured set; a
    /// `Freie Runde` configures no Pflock set, so the `Pflock` form carries the colour the
    /// archer picked and `Farbe` has already proven it is a colour. One owner for the
    /// question, so a third setup cannot silently skip it.
    pub fn pruefe_position(
        &self,
        ziel_id: ZielId,
        position: Option<Schussposition>,
    ) -> Result<(), PositionNichtImAufbau> {
        let Some(Schussposition::Pflock(farbe)) = position else {
            return Ok(());
        };
        match self {
            Self::Parkour { snapshot, .. } => {
                if snapshot.hat_pflock_am_ziel(ziel_id, farbe) {
                    Ok(())
                } else {
                    Err(PositionNichtImAufbau)
                }
            }
            Self::Freie(_) => Ok(()),
        }
    }
}

/// One Ziels of a `Freie Runde`, created on the spot (SPEC-runde rule 18).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreiesZiel {
    ziel_id: ZielId,
    name: NonEmptyString,
    ordinal: NonZeroU32,
}

impl FreiesZiel {
    pub fn from_parts(ziel_id: ZielId, name: NonEmptyString, ordinal: NonZeroU32) -> Self {
        Self {
            ziel_id,
            name,
            ordinal,
        }
    }

    pub fn ziel_id(&self) -> ZielId {
        self.ziel_id
    }

    pub fn name(&self) -> &NonEmptyString {
        &self.name
    }

    pub fn umbenennen(&mut self, name: NonEmptyString) {
        self.name = name;
    }

    /// 1-based ordinal, the same number the default name is built from.
    pub fn ordinal(&self) -> NonZeroU32 {
        self.ordinal
    }
}

/// A Runde with no Anlage and no Parkour, whose Ziele the archer adds while shooting.
///
/// The mutators live only here, so a `Rundenaufbau::Parkour` cannot have its Ziel list
/// edited: the type system refuses it, not a runtime check (SPEC-runde rule 18).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreieRunde {
    name: NonEmptyString,
    ziele: Vec<FreiesZiel>,
}

impl FreieRunde {
    pub fn neu(name: NonEmptyString) -> Self {
        Self {
            name,
            ziele: Vec::new(),
        }
    }

    pub fn from_parts(name: NonEmptyString, ziele: Vec<FreiesZiel>) -> Self {
        Self { name, ziele }
    }

    pub fn name(&self) -> &NonEmptyString {
        &self.name
    }

    pub fn ziele(&self) -> &[FreiesZiel] {
        &self.ziele
    }

    pub fn ziel_mut(&mut self, ziel_id: ZielId) -> Option<&mut FreiesZiel> {
        self.ziele.iter_mut().find(|ziel| ziel.ziel_id == ziel_id)
    }

    pub fn ziel(&self, ziel_id: ZielId) -> Option<&FreiesZiel> {
        self.ziele.iter().find(|ziel| ziel.ziel_id == ziel_id)
    }

    pub fn enthält_ziel(&self, ziel_id: ZielId) -> bool {
        self.ziel(ziel_id).is_some()
    }

    /// Append a Ziel with the next ordinal, named "Ziel N" unless a name was given.
    ///
    /// The ordinal never reuses a removed Ziel's number, so two Ziele of one Runde never
    /// share a default name.
    pub fn ziel_hinzufügen(&mut self, ziel_id: ZielId, name: Option<NonEmptyString>) {
        let ordinal = self
            .ziele
            .iter()
            .map(|ziel| ziel.ordinal.get())
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        let ordinal = NonZeroU32::new(ordinal).unwrap_or(NonZeroU32::MIN);
        let name = name.unwrap_or_else(|| {
            NonEmptyString::neu(format!("Ziel {ordinal}"))
                .expect("a formatted ordinal is never empty")
        });
        self.ziele
            .push(FreiesZiel::from_parts(ziel_id, name, ordinal));
    }

    /// Remove one Ziel; its Schüsse are the caller's to delete in the same transaction.
    pub fn ziel_entfernen(&mut self, ziel_id: ZielId) -> Option<FreiesZiel> {
        let index = self.ziele.iter().position(|ziel| ziel.ziel_id == ziel_id)?;
        Some(self.ziele.remove(index))
    }

    /// The distinct colours the round's Schüsse already name, in first-used order.
    ///
    /// The vocabulary a capture screen offers; it constrains nothing, because a free
    /// round's colour is caller data (SPEC-runde rule 19).
    pub fn farben(&self, schüsse: impl IntoIterator<Item = Schuss>) -> Vec<Farbe> {
        let eigene: Vec<Schuss> = schüsse
            .into_iter()
            .filter(|schuss| self.enthält_ziel(schuss.ziel_id()))
            .collect();
        let mut farben: Vec<Farbe> = Vec::new();
        for schuss in eigene {
            if let Some(Schussposition::Pflock(farbe)) = schuss.position()
                && !farben.contains(&farbe)
            {
                farben.push(farbe);
            }
        }
        farben
    }
}

/// One Ziel of any setup, as the one view the Wertung fold and the read model need.
#[derive(Debug, Clone, Copy)]
pub enum ZielRef<'a> {
    Parkour(&'a ZielSnapshot),
    Freie(&'a FreiesZiel),
}

impl<'a> ZielRef<'a> {
    pub fn ziel_id(&self) -> ZielId {
        match self {
            Self::Parkour(ziel) => ziel.ziel_id(),
            Self::Freie(ziel) => ziel.ziel_id(),
        }
    }

    pub fn name(&self) -> &'a NonEmptyString {
        match self {
            Self::Parkour(ziel) => ziel.tiername(),
            Self::Freie(ziel) => ziel.name(),
        }
    }

    pub fn ordinal(&self) -> NonZeroU32 {
        match self {
            Self::Parkour(ziel) => ziel.zielnummer(),
            Self::Freie(ziel) => ziel.ordinal(),
        }
    }

    /// The captured Pflock distances; a free Ziel configures none.
    pub fn abstände(&self) -> &'a [(Farbe, Abstand)] {
        match self {
            Self::Parkour(ziel) => ziel.abstände(),
            Self::Freie(_) => &[],
        }
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
    pub fn neu(
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
    pub fn frei(name: NonEmptyString, erfasser: Erfasser, begonnen_am: DateTime<Utc>) -> Self {
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
            } => Rundenaufbau::parkour(*anlage_id, snapshot.clone()),
            Self::Freie { name, .. } => Rundenaufbau::Freie(FreieRunde::neu(name.clone())),
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
pub type RundeAbgeschlossen = RundeIn<Abgeschlossen>;

/// The sum of both states, for code that does not know the state at compile time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Runde {
    InBearbeitung(RundeInBearbeitung),
    Abgeschlossen(RundeAbgeschlossen),
}

impl<S> RundeIn<S> {
    pub fn id(&self) -> RundeId {
        self.id
    }

    pub fn aufbau(&self) -> &Rundenaufbau {
        &self.aufbau
    }

    /// The master-data pair, `Some` only for a Parkour round.
    pub fn referenzen(&self) -> Option<&RundeOutgoingReferences> {
        self.aufbau.referenzen()
    }

    pub fn anlage_id(&self) -> Option<AnlageId> {
        self.referenzen().map(RundeOutgoingReferences::anlage_id)
    }

    pub fn parkour_id(&self) -> Option<ParkourId> {
        self.referenzen().map(RundeOutgoingReferences::parkour_id)
    }

    /// The captured setup, `Some` only for a Parkour round.
    pub fn snapshot(&self) -> Option<&ParkourSnapshot> {
        self.aufbau.snapshot()
    }

    pub fn freie(&self) -> Option<&FreieRunde> {
        self.aufbau.freie()
    }

    pub fn freie_mut(&mut self) -> Option<&mut FreieRunde> {
        self.aufbau.freie_mut()
    }

    /// The round's own name: the captured Parkour's, or the free round's.
    pub fn name(&self) -> &NonEmptyString {
        self.aufbau.name()
    }

    /// The captured Parkour-wide Pflöcke; a free round has none.
    pub fn pflöcke(&self) -> &[PflockSnapshot] {
        self.aufbau.pflöcke()
    }

    /// The Ziele the round walks, in visiting order.
    pub fn ziele(&self) -> impl Iterator<Item = ZielRef<'_>> {
        self.aufbau.ziele()
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
    pub fn enthält_ziel(&self, ziel_id: ZielId) -> bool {
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
                ZielSchüsse::für_ziel(
                    ziel.ziel_id(),
                    eigene
                        .iter()
                        .filter(|schuss| schuss.ziel_id() == ziel.ziel_id())
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
            ZielSchüsse::für_ziel(
                ziel.ziel_id(),
                eigener
                    .iter()
                    .filter(|schuss| schuss.ziel_id() == ziel.ziel_id())
                    .cloned(),
            )
            .ok()
        });
        Rundenwertung::aus_zielen(je_ziel.map(|menge| menge.wertung()))
    }
}

impl RundeInBearbeitung {
    pub fn neu(id: RundeId, neu: NeueRunde) -> Self {
        Self {
            id,
            erfasst_von: neu.erfasst_von(),
            aufbau: neu.aufbau(),
            begonnen_am: neu.begonnen_am(),
            state: InBearbeitung,
        }
    }

    /// Rebuild a persisted, still running Runde.
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
            state: InBearbeitung,
        }
    }

    /// Close the Runde: no further Schuss may be registered.
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
    pub fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
        abgeschlossen_am: Option<DateTime<Utc>>,
    ) -> Self {
        match abgeschlossen_am {
            None => Self::InBearbeitung(RundeInBearbeitung::from_parts(
                id,
                erfasser,
                aufbau,
                begonnen_am,
            )),
            Some(am) => Self::Abgeschlossen(RundeAbgeschlossen::from_parts(
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
            Self::Abgeschlossen(runde) => runde.id(),
        }
    }

    pub fn aufbau(&self) -> &Rundenaufbau {
        match self {
            Self::InBearbeitung(runde) => runde.aufbau(),
            Self::Abgeschlossen(runde) => runde.aufbau(),
        }
    }

    pub fn referenzen(&self) -> Option<&RundeOutgoingReferences> {
        self.aufbau().referenzen()
    }

    pub fn anlage_id(&self) -> Option<AnlageId> {
        self.referenzen().map(RundeOutgoingReferences::anlage_id)
    }

    pub fn parkour_id(&self) -> Option<ParkourId> {
        self.referenzen().map(RundeOutgoingReferences::parkour_id)
    }

    pub fn snapshot(&self) -> Option<&ParkourSnapshot> {
        self.aufbau().snapshot()
    }

    pub fn freie(&self) -> Option<&FreieRunde> {
        self.aufbau().freie()
    }

    pub fn freie_mut(&mut self) -> Option<&mut FreieRunde> {
        match self {
            Self::InBearbeitung(runde) => runde.freie_mut(),
            Self::Abgeschlossen(runde) => runde.freie_mut(),
        }
    }

    pub fn name(&self) -> &NonEmptyString {
        self.aufbau().name()
    }

    pub fn pflöcke(&self) -> &[PflockSnapshot] {
        self.aufbau().pflöcke()
    }

    pub fn ziele(&self) -> impl Iterator<Item = ZielRef<'_>> {
        self.aufbau().ziele()
    }

    pub fn erfasst_von(&self) -> Erfasser {
        match self {
            Self::InBearbeitung(runde) => runde.erfasst_von(),
            Self::Abgeschlossen(runde) => runde.erfasst_von(),
        }
    }

    /// The stored lifecycle spelling, taken from the active variant itself.
    pub fn zustand(&self) -> &'static str {
        match self {
            Self::InBearbeitung(runde) => runde.state.as_str(),
            Self::Abgeschlossen(runde) => runde.state.as_str(),
        }
    }

    pub fn begonnen_am(&self) -> DateTime<Utc> {
        match self {
            Self::InBearbeitung(runde) => runde.begonnen_am(),
            Self::Abgeschlossen(runde) => runde.begonnen_am(),
        }
    }

    pub fn abgeschlossen_am(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::InBearbeitung(_) => None,
            Self::Abgeschlossen(runde) => Some(runde.abgeschlossen_am()),
        }
    }

    pub fn gehört_zu(&self, benutzer_id: BenutzerId) -> bool {
        match self {
            Self::InBearbeitung(runde) => runde.gehört_zu(benutzer_id),
            Self::Abgeschlossen(runde) => runde.gehört_zu(benutzer_id),
        }
    }

    pub fn ist_in_bearbeitung(&self) -> bool {
        matches!(self, Self::InBearbeitung(_))
    }

    pub fn enthält_ziel(&self, ziel_id: ZielId) -> bool {
        match self {
            Self::InBearbeitung(runde) => runde.enthält_ziel(ziel_id),
            Self::Abgeschlossen(runde) => runde.enthält_ziel(ziel_id),
        }
    }

    /// Replace the owner in either state (SPEC-datenschutz retention).
    pub fn anonymisiere(&mut self, am: DateTime<Utc>) {
        match self {
            Self::InBearbeitung(runde) => runde.anonymisiere(am),
            Self::Abgeschlossen(runde) => runde.anonymisiere(am),
        }
    }

    /// The per-Ziel and Runde-wide Wertung over the given Schüsse.
    ///
    /// Schüsse of another Runde are filtered out, so passing every loaded row is safe.
    pub fn wertung(&self, schüsse: impl IntoIterator<Item = Schuss>) -> Rundenwertung {
        match self {
            Self::InBearbeitung(runde) => runde.wertung(schüsse),
            Self::Abgeschlossen(runde) => runde.wertung(schüsse),
        }
    }

    /// One Wertung per captured Ziel, in visiting order. Delegated like [`Runde::wertung`].
    pub fn ziel_wertungen(
        &self,
        schüsse: impl IntoIterator<Item = Schuss>,
    ) -> Vec<Option<Zielwertung>> {
        match self {
            Self::InBearbeitung(runde) => runde.ziel_wertungen(schüsse),
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
                        ZielId(Uuid::from_u128(id)),
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
        Runde::InBearbeitung(RundeInBearbeitung::neu(
            RundeId(Uuid::nil()),
            NeueRunde::neu(
                AnlageId(Uuid::nil()),
                Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                snapshot(ziele),
                jetzt(),
            ),
        ))
    }

    fn freie_runde() -> Runde {
        Runde::InBearbeitung(RundeInBearbeitung::neu(
            RundeId(Uuid::nil()),
            NeueRunde::frei(
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
                ZielId(Uuid::from_u128(ziel)),
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
                    ZielId(Uuid::from_u128(11)),
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
                ZielId(Uuid::from_u128(10)),
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
            panic!("eine neue Runde ist in Bearbeitung");
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
        assert!(runde.enthält_ziel(ZielId(Uuid::from_u128(10))));
        assert!(!runde.enthält_ziel(ZielId(Uuid::from_u128(11))));
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
            panic!("offen");
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
    fn ac21_ein_parkour_aufbau_traegt_genau_ein_identitaetspaar() {
        let runde = runde(&[(10, "Keiler")]);
        let Some(referenzen) = runde.referenzen() else {
            panic!("ein Parkour-Aufbau trägt Referenzen");
        };
        assert_eq!(referenzen.anlage_id(), AnlageId(Uuid::nil()));
        assert_eq!(referenzen.parkour_id(), ParkourId(Uuid::from_u128(7)));
        // The pair is one value derived from the snapshot, so it cannot name two Parkours.
        assert_eq!(runde.anlage_id(), Some(AnlageId(Uuid::nil())));
        assert_eq!(runde.parkour_id(), Some(ParkourId(Uuid::from_u128(7))));
        assert!(runde.freie().is_none());
        assert_eq!(runde.name().as_str(), "Waldrunde");
        assert_eq!(runde.pflöcke().len(), 1);
    }

    #[test_log::test]
    fn ac21_eine_freie_runde_hat_weder_anlage_noch_parkour_noch_pfloecke() {
        let runde = freie_runde();
        assert!(runde.referenzen().is_none());
        assert!(runde.anlage_id().is_none());
        assert!(runde.parkour_id().is_none());
        assert!(runde.snapshot().is_none());
        assert!(runde.pflöcke().is_empty());
        assert_eq!(runde.name().as_str(), "Wiese daheim");
        assert_eq!(runde.ziele().count(), 0);
        let Some(freie) = runde.freie() else {
            panic!("eine freie Runde trägt ihren Aufbau");
        };
        assert!(freie.ziele().is_empty());
    }

    #[test_log::test]
    fn ac22_freie_ziele_werden_fortlaufend_nummeriert_und_nie_wiederverwendet() {
        let mut runde = freie_runde();
        let freie = runde.freie_mut().expect("eine freie Runde");
        let erstes = ZielId(Uuid::from_u128(1));
        let zweites = ZielId(Uuid::from_u128(2));
        freie.ziel_hinzufügen(erstes, None);
        freie.ziel_hinzufügen(zweites, None);
        freie.ziel_hinzufügen(ZielId(Uuid::from_u128(3)), Some(name("Baumstumpf")));

        let namen: Vec<&str> = freie
            .ziele()
            .iter()
            .map(|ziel| ziel.name().as_str())
            .collect();
        assert_eq!(namen, ["Ziel 1", "Ziel 2", "Baumstumpf"]);
        assert_eq!(freie.ziele()[2].ordinal().get(), 3);

        // Removing the second Ziel frees no number: the next one keeps counting.
        assert_eq!(
            freie
                .ziel_entfernen(zweites)
                .map(|ziel| ziel.ordinal().get()),
            Some(2)
        );
        freie.ziel_hinzufügen(ZielId(Uuid::from_u128(4)), None);
        assert_eq!(freie.ziele().last().expect("ein Ziel").ordinal().get(), 4);
        assert_eq!(
            freie.ziele().last().expect("ein Ziel").name().as_str(),
            "Ziel 4"
        );
        assert!(freie.ziel_entfernen(ZielId(Uuid::from_u128(99))).is_none());
    }

    #[test_log::test]
    fn ac22_ein_parkour_aufbau_hat_keinen_ziel_mutator() {
        // The compile-time half of the promise: this block deliberately does not compile,
        // because `ziel_hinzufügen` exists only on `FreieRunde`.
        //
        // ```compile_fail
        // let mut aufbau = Rundenaufbau::parkour(AnlageId(uuid::Uuid::nil()), snapshot);
        // aufbau.ziel_hinzufügen(ZielId(uuid::Uuid::nil()), None);
        // ```
        let mut aufbau = rundenaufbau();
        assert!(aufbau.freie().is_none());
        assert!(aufbau.freie_mut().is_none());
    }

    fn rundenaufbau() -> Rundenaufbau {
        Rundenaufbau::parkour(AnlageId(Uuid::nil()), snapshot(&[(10, "Keiler")]))
    }

    #[test_log::test]
    fn zielref_zeigt_beide_aufbauten_gleich() {
        let mut runde = freie_runde();
        runde
            .freie_mut()
            .expect("freie Runde")
            .ziel_hinzufügen(ZielId(Uuid::from_u128(5)), None);
        let ziele: Vec<ZielRef<'_>> = runde.ziele().collect();
        assert_eq!(ziele.len(), 1);
        assert_eq!(ziele[0].ziel_id(), ZielId(Uuid::from_u128(5)));
        assert_eq!(ziele[0].name().as_str(), "Ziel 1");
        assert_eq!(ziele[0].ordinal().get(), 1);
        assert!(ziele[0].abstände().is_empty());

        let parkour = rundenaufbau();
        let ziele: Vec<ZielRef<'_>> = parkour.ziele().collect();
        assert_eq!(ziele[0].name().as_str(), "Keiler");
        assert_eq!(ziele[0].ordinal().get(), 1);
        assert!(ziele[0].abstände().is_empty());
    }

    #[test_log::test]
    fn ac23_parkour_pflock_wird_gegen_das_ziel_geprueft() {
        let aufbau = rundenaufbau();
        let ziel = ZielId(Uuid::from_u128(10));
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
        let aufbau = Rundenaufbau::Freie(FreieRunde::neu(name("Wiese")));
        let ziel = ZielId(Uuid::from_u128(1));
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
        let mut runde = freie_runde();
        let freie = runde.freie_mut().expect("freie Runde");
        freie.ziel_hinzufügen(ZielId(Uuid::from_u128(1)), None);
        let rot = Farbe::neu("#c0392b").unwrap();
        let blau = Farbe::neu("#1b4f9c").unwrap();
        let schüsse = vec![
            Schuss::neu(
                SchussId(Uuid::from_u128(1)),
                NeuerSchuss::neu(
                    RundeId(Uuid::nil()),
                    Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                    ZielId(Uuid::from_u128(1)),
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
                    ZielId(Uuid::from_u128(1)),
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
                    ZielId(Uuid::from_u128(1)),
                    3,
                    Wertungszone::Spot,
                    Some(Schussposition::Distanz(Abstand::neu(35).unwrap())),
                )
                .unwrap(),
            ),
        ];
        assert_eq!(freie.farben(schüsse), vec![rot]);
        assert!(!freie.farben(Vec::new()).contains(&blau));
    }

    #[test_log::test]
    fn ac24_die_wertung_faltet_ueber_die_freien_ziele() {
        let mut runde = freie_runde();
        {
            let freie = runde.freie_mut().expect("freie Runde");
            freie.ziel_hinzufügen(ZielId(Uuid::from_u128(1)), None);
            freie.ziel_hinzufügen(ZielId(Uuid::from_u128(2)), None);
        }
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
        runde
            .freie_mut()
            .expect("freie Runde")
            .ziel_entfernen(ZielId(Uuid::from_u128(2)));
        let wertung = runde.wertung(schüsse);
        assert_eq!(wertung.ziele_mit_wertung(), 1);
    }
}
