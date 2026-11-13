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
//! - the **provenance**, as the Parkour a round was started at ([`RundenHerkunft`]) — a free
//!   round has none, which is why the marker for one carries nothing, and
//! - the **lifecycle**, as whether a closing timestamp exists ([`Abgeschlossen`]).
//!
//! The provenance is the marker, so "a free round over a Parkour setup" has no spelling: the
//! marker fields are private and every constructor picks the marker from the same value it
//! takes the route from. Route mutations exist only on [`FreieRundeInBearbeitung`], which is
//! the AC-22 promise as a compile-time guarantee, and a closed round has no mutator at all.
//!
//! The route itself ([`Rundenaufbau`]) is the same shape in every state: it records the
//! topology and the captured data, and it is immutable once the round is closed. It is built
//! once, at capture ([`Rundenaufbau::aus_parkour`]), so per-target stake inheritance is already
//! resolved when the route is stored.

use std::num::NonZeroU32;

use chrono::{DateTime, Utc};

use super::anlage::{AnlageId, AnlageZiel, AnlageZielId, Parkour, ParkourFehler, ParkourId};
use super::common::{Abstand, Farbe, NonEmptyString};
use super::konto::BenutzerId;
use super::schuss::{Erfasser, Schuss, Schussposition, ZielSchüsse, Zielwertung};

/// Identity of one Runde. The aggregate owns its identity; Schuss only refers to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RundeId(pub uuid::Uuid);

/// A typestate's persisted lifecycle spelling.
pub trait RundeTypestate {
    fn zustand() -> &'static str;
}

// ---------------------------------------------------------------------------------------
// Where a Runde came from
// ---------------------------------------------------------------------------------------

/// The provenance of a round that was started at a Parkour: which Parkour, at which Anlage.
///
/// Provenance, not topology: the route it was seeded with lives in [`Rundenaufbau`], which is
/// the same shape for both. A free round has no provenance at all — it answers `None` — so
/// the round's *name* is not stored here: a free round names itself, and the name lives on
/// the round.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RundenHerkunft {
    pub anlage_id: AnlageId,
    pub parkour_id: ParkourId,
}

// ---------------------------------------------------------------------------------------
// Lifecycle markers of the `Runde` typestate
// ---------------------------------------------------------------------------------------

/// A running round seeded from a Parkour.
///
/// The field is private and the only constructors are in this module, so this state cannot be
/// built without a provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MitParkourInBearbeitung {
    herkunft: RundenHerkunft,
}

impl MitParkourInBearbeitung {
    fn neu(herkunft: RundenHerkunft) -> Self {
        Self { herkunft }
    }

    pub fn herkunft(&self) -> &RundenHerkunft {
        &self.herkunft
    }
}

impl RundeTypestate for MitParkourInBearbeitung {
    fn zustand() -> &'static str {
        "InBearbeitung"
    }
}

/// A running round that owns its Ziel list and permits just-in-time route edits.
///
/// A unit struct: it names no Anlage and no Parkour, so "a free round that also has a
/// provenance" has no spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FreieRundeInBearbeitung;

impl RundeTypestate for FreieRundeInBearbeitung {
    fn zustand() -> &'static str {
        "InBearbeitung"
    }
}

/// The closed state: it carries when it happened and the provenance it happened under.
///
/// The timestamp lives in the marker, so "when was it closed?" is not an `Option` a reader
/// must interpret, and closing never erases it. The provenance is an `Option` because a
/// closed *free* round still has none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abgeschlossen {
    abgeschlossen_am: DateTime<Utc>,
    herkunft: Option<RundenHerkunft>,
}

impl Abgeschlossen {
    pub fn abgeschlossen_am(&self) -> DateTime<Utc> {
        self.abgeschlossen_am
    }

    pub fn herkunft(&self) -> Option<&RundenHerkunft> {
        self.herkunft.as_ref()
    }
}

impl RundeTypestate for Abgeschlossen {
    fn zustand() -> &'static str {
        "Abgeschlossen"
    }
}

// ---------------------------------------------------------------------------------------
// Round-owned target route
// ---------------------------------------------------------------------------------------

/// Neutral identity for a target within a single Runde.
///
/// Deliberately not the master-data [`AnlageZielId`]: a round keeps its own target list, and
/// a later master-data edit may not rename or remove anything the round captured (AC-7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZielId(pub uuid::Uuid);

/// One shooting position of a round's route: its colour and its captured distance.
///
/// The distance is folded in at capture, so a reader never has to join a Pflock list against
/// a separate distance map. `None` is a Pflock the master data carries no distance for — a
/// missing estimate, not a zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pflock {
    farbe: Farbe,
    abstand: Option<Abstand>,
}

impl Pflock {
    pub fn neu(farbe: Farbe, abstand: Option<Abstand>) -> Self {
        Self { farbe, abstand }
    }

    pub fn farbe(&self) -> Farbe {
        self.farbe
    }

    /// The distance configured for this Pflock at capture, if there was one.
    pub fn abstand(&self) -> Option<Abstand> {
        self.abstand
    }
}

/// One target in the route captured by a Runde.
///
/// `pflöcke` is the Ziel's **effective** set: its own when the master data gave it one, the
/// Parkour-wide set otherwise. Inheritance is resolved here, at capture, so the stored route
/// answers "which Pflock may be shot from here?" on its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ziel {
    id: ZielId,
    anlage_ziel_id: Option<AnlageZielId>,
    name: NonEmptyString,
    ordinal: NonZeroU32,
    pflöcke: Vec<Pflock>,
}

impl Ziel {
    pub fn from_parts(
        id: ZielId,
        anlage_ziel_id: Option<AnlageZielId>,
        name: NonEmptyString,
        ordinal: NonZeroU32,
        pflöcke: Vec<Pflock>,
    ) -> Self {
        Self {
            id,
            anlage_ziel_id,
            name,
            ordinal,
            pflöcke,
        }
    }

    pub fn id(&self) -> ZielId {
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

    /// The Pflock set that applies to this Ziel, near to far, with its distances.
    pub fn pflöcke(&self) -> &[Pflock] {
        &self.pflöcke
    }

    /// Whether this Ziel's captured set contains `farbe`.
    pub fn hat_pflock(&self, farbe: Farbe) -> bool {
        self.pflöcke.iter().any(|pflock| pflock.farbe() == farbe)
    }

    fn umbenennen(&mut self, name: NonEmptyString) {
        self.name = name;
    }
}

/// The route one Runde captured: its ordered Ziele, each with its own effective Pflock set.
///
/// Topology and data only, with no provenance inside it, so the same value serves a Parkour
/// round, a free round, and a closed round. A Parkour round seeds it by capture
/// ([`Rundenaufbau::aus_parkour`], AC-7/AC-9: a value copy, never a live read) and leaves it
/// alone; a free round appends and removes targets while it runs; a closed round keeps it as
/// the round's historical record.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Rundenaufbau {
    ziele: Vec<Ziel>,
}

impl Rundenaufbau {
    pub fn from_parts(ziele: Vec<Ziel>) -> Self {
        Self { ziele }
    }

    /// Capture one Parkour's current setup as a value copy of this round's route.
    ///
    /// Each placement becomes one [`Ziel`], and its **effective** Pflock set — its own when
    /// the master data gave it one, the Parkour-wide set otherwise — is flattened into one
    /// list, with each configured distance folded into the matching [`Pflock`]. A Pflock the
    /// Parkour configures no distance for keeps `abstand: None`.
    ///
    /// The `ziel` lookup is the owning `Anlage`: the Parkour holds an [`AnlageZielId`], but the
    /// name and ordinal an archer sees are the Anlage's.
    pub fn aus_parkour<'a>(
        parkour: &Parkour,
        ziel: impl Fn(AnlageZielId) -> Option<&'a AnlageZiel>,
    ) -> Result<Self, ParkourFehler> {
        let ziele = parkour
            .platzierungen()
            .iter()
            .map(|platzierung| {
                let anlage_ziel =
                    ziel(platzierung.ziel_id()).ok_or(ParkourFehler::ZielNichtGefunden)?;
                let pflöcke = platzierung
                    .effektive_pflöcke(parkour.pflöcke())
                    .iter()
                    .map(|pflock| Pflock::neu(pflock.farbe(), platzierung.abstand_für(pflock.id())))
                    .collect();
                Ok(Ziel::from_parts(
                    ZielId(uuid::Uuid::new_v4()),
                    Some(anlage_ziel.id()),
                    anlage_ziel.tiername().clone(),
                    anlage_ziel.zielnummer(),
                    pflöcke,
                ))
            })
            .collect::<Result<Vec<_>, ParkourFehler>>()?;
        Ok(Self { ziele })
    }

    pub fn ziele(&self) -> &[Ziel] {
        &self.ziele
    }

    pub fn ziel(&self, id: ZielId) -> Option<&Ziel> {
        self.ziele.iter().find(|ziel| ziel.id == id)
    }

    pub fn enthält_ziel(&self, id: ZielId) -> bool {
        self.ziel(id).is_some()
    }

    fn ziel_mut(&mut self, id: ZielId) -> Option<&mut Ziel> {
        self.ziele.iter_mut().find(|ziel| ziel.id == id)
    }

    fn ziel_hinzufügen(&mut self, id: ZielId, name: Option<NonEmptyString>) {
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
        // A free Ziel configures no Pflock set: the archer names a colour per shot.
        self.ziele
            .push(Ziel::from_parts(id, None, name, ordinal, Vec::new()));
    }

    fn ziel_entfernen(&mut self, id: ZielId) -> Option<Ziel> {
        let index = self.ziele.iter().position(|ziel| ziel.id == id)?;
        Some(self.ziele.remove(index))
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
        parkour_id: ParkourId,
        /// The Parkour's name at capture; a later rename may not reach it (AC-7).
        name: NonEmptyString,
        /// The route captured from the Parkour, distances already folded in.
        aufbau: Rundenaufbau,
        erfasser: Erfasser,
        begonnen_am: DateTime<Utc>,
    },
    Freie {
        name: NonEmptyString,
        erfasser: Erfasser,
        begonnen_am: DateTime<Utc>,
    },
}

impl NeueRunde {
    /// Start a Parkour round from an already captured route.
    ///
    /// The route is a value copy taken in the same transaction as the round (AC-7, AC-9), so
    /// this constructor takes it finished rather than reading master data itself.
    pub fn mit_parkour(
        anlage_id: AnlageId,
        parkour_id: ParkourId,
        name: NonEmptyString,
        aufbau: Rundenaufbau,
        erfasser: Erfasser,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self::Parkour {
            anlage_id,
            parkour_id,
            name,
            aufbau,
            erfasser,
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

    pub fn name(&self) -> &NonEmptyString {
        match self {
            Self::Parkour { name, .. } | Self::Freie { name, .. } => name,
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
}

// ---------------------------------------------------------------------------------------
// The aggregate
// ---------------------------------------------------------------------------------------

/// One logged Runde: who, the captured route, its name, and the state that names its kind
/// and lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RundeIn<S> {
    id: RundeId,
    erfasst_von: Erfasser,
    /// A free round's own name, or the captured Parkour's. Independent of the state, so a
    /// free round and a Parkour round answer the same question the same way.
    name: NonEmptyString,
    aufbau: Rundenaufbau,
    begonnen_am: DateTime<Utc>,
    state: S,
}

/// A running round started at a Parkour: its route is captured, never edited.
pub type RundeMitParkour = RundeIn<MitParkourInBearbeitung>;
/// A running free round: it owns its Ziel list and edits it just in time.
pub type FreieRunde = RundeIn<FreieRundeInBearbeitung>;
/// A closed round, whatever it was started from.
pub type AbgeschlosseneRunde = RundeIn<Abgeschlossen>;

/// The two open states of a Runde and its common closed state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Runde {
    MitParkour(RundeMitParkour),
    FreieRunde(FreieRunde),
    AbgeschlosseneRunde(AbgeschlosseneRunde),
}

impl<S> RundeIn<S> {
    pub fn id(&self) -> RundeId {
        self.id
    }

    pub fn aufbau(&self) -> &Rundenaufbau {
        &self.aufbau
    }

    /// The round's name: a free round's own, or the captured Parkour's.
    pub fn name(&self) -> &NonEmptyString {
        &self.name
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
    pub fn enthält_ziel(&self, ziel_id: ZielId) -> bool {
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
                    ziel_id,
                    eigene
                        .iter()
                        .filter(|schuss| schuss.ziel_id() == ziel_id)
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
            ZielSchüsse::für_ziel(
                ziel_id,
                eigener
                    .iter()
                    .filter(|schuss| schuss.ziel_id() == ziel_id)
                    .cloned(),
            )
            .ok()
        });
        Rundenwertung::aus_zielen(je_ziel.map(|menge| menge.wertung()))
    }
}

impl RundeMitParkour {
    /// Start a Parkour round from its captured route.
    ///
    /// The parameters are the Parkour's own identities, which a free round does not have, so
    /// a free round cannot reach this constructor at all — not even by mistake.
    fn aus_neu(
        id: RundeId,
        erfasser: Erfasser,
        anlage_id: AnlageId,
        parkour_id: ParkourId,
        name: NonEmptyString,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            erfasst_von: erfasser,
            name,
            aufbau,
            begonnen_am,
            state: MitParkourInBearbeitung::neu(RundenHerkunft {
                anlage_id,
                parkour_id,
            }),
        }
    }

    /// Rebuild a persisted Parkour round.
    fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        name: NonEmptyString,
        herkunft: RundenHerkunft,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            erfasst_von: erfasser,
            name,
            aufbau,
            begonnen_am,
            state: MitParkourInBearbeitung::neu(herkunft),
        }
    }

    /// Where this round came from.
    pub fn herkunft(&self) -> &RundenHerkunft {
        self.state.herkunft()
    }

    /// Whether this position may be captured at this Ziel (AC-9, AC-23).
    ///
    /// A Parkour round only accepts a Pflock its captured route carries at that Ziel; a
    /// distance is always a position, because the captured route cannot know how far away a
    /// target is from a shot it never placed. The route decides, and an unknown Ziel is
    /// refused before the Pflock is looked at.
    pub fn pruefe_position(
        &self,
        ziel_id: ZielId,
        position: Option<Schussposition>,
    ) -> Result<(), PositionNichtImAufbau> {
        let ziel = self.aufbau.ziel(ziel_id).ok_or(PositionNichtImAufbau)?;
        match position {
            None | Some(Schussposition::Distanz(_)) => Ok(()),
            Some(Schussposition::Pflock(farbe)) if ziel.hat_pflock(farbe) => Ok(()),
            Some(Schussposition::Pflock(_)) => Err(PositionNichtImAufbau),
        }
    }

    /// Close a Parkour-backed Runde: no further Schuss may be registered.
    pub fn abschliessen(self, am: DateTime<Utc>) -> AbgeschlosseneRunde {
        RundeIn {
            id: self.id,
            erfasst_von: self.erfasst_von,
            name: self.name,
            aufbau: self.aufbau,
            begonnen_am: self.begonnen_am,
            state: Abgeschlossen {
                abgeschlossen_am: am,
                herkunft: Some(self.state.herkunft),
            },
        }
    }
}

impl FreieRunde {
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
            name,
            aufbau: Rundenaufbau::default(),
            begonnen_am,
            state: FreieRundeInBearbeitung,
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
            name,
            aufbau,
            begonnen_am,
            state: FreieRundeInBearbeitung,
        }
    }

    /// Append a Ziel with a fresh round-local identity and answer that identity.
    ///
    /// The round mints the id, so a caller cannot hand in one that already exists and the
    /// append cannot be refused: a duplicate is unrepresentable, not an error to report.
    pub fn ziel_hinzufügen(&mut self, name: Option<NonEmptyString>) -> ZielId {
        let id = ZielId(uuid::Uuid::new_v4());
        self.aufbau.ziel_hinzufügen(id, name);
        id
    }

    pub fn ziel_entfernen(&mut self, id: ZielId) -> Option<Ziel> {
        self.aufbau.ziel_entfernen(id)
    }

    pub fn ziel_umbenennen(&mut self, id: ZielId, name: NonEmptyString) -> bool {
        let Some(ziel) = self.aufbau.ziel_mut(id) else {
            return false;
        };
        ziel.umbenennen(name);
        true
    }

    /// Whether this Ziel belongs to this free round's route.
    pub fn pruefe_position(&self, ziel_id: ZielId) -> Result<(), PositionNichtImAufbau> {
        if !self.aufbau.enthält_ziel(ziel_id) {
            return Err(PositionNichtImAufbau);
        }
        Ok(())
    }

    /// Close the free round: its route and its names are the round's historical record.
    pub fn abschliessen(self, am: DateTime<Utc>) -> AbgeschlosseneRunde {
        RundeIn {
            id: self.id,
            erfasst_von: self.erfasst_von,
            name: self.name,
            aufbau: self.aufbau,
            begonnen_am: self.begonnen_am,
            state: Abgeschlossen {
                abgeschlossen_am: am,
                herkunft: None,
            },
        }
    }
}

impl AbgeschlosseneRunde {
    /// Rebuild a persisted, closed Runde.
    fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        name: NonEmptyString,
        herkunft: Option<RundenHerkunft>,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
        abgeschlossen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            erfasst_von: erfasser,
            name,
            aufbau,
            begonnen_am,
            state: Abgeschlossen {
                abgeschlossen_am,
                herkunft,
            },
        }
    }

    pub fn herkunft(&self) -> Option<&RundenHerkunft> {
        self.state.herkunft()
    }

    pub fn abgeschlossen_am(&self) -> DateTime<Utc> {
        self.state.abgeschlossen_am
    }

    // No `pruefe_position` and no mutator: that is the point of the typestate.
}

impl From<RundeMitParkour> for Runde {
    fn from(runde: RundeMitParkour) -> Self {
        Self::MitParkour(runde)
    }
}

impl From<FreieRunde> for Runde {
    fn from(runde: FreieRunde) -> Self {
        Self::FreieRunde(runde)
    }
}

impl From<AbgeschlosseneRunde> for Runde {
    fn from(runde: AbgeschlosseneRunde) -> Self {
        Self::AbgeschlosseneRunde(runde)
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
                parkour_id,
                name,
                aufbau,
                erfasser,
                begonnen_am,
            } => Self::MitParkour(RundeMitParkour::aus_neu(
                id,
                erfasser,
                anlage_id,
                parkour_id,
                name,
                aufbau,
                begonnen_am,
            )),
            NeueRunde::Freie {
                name,
                erfasser,
                begonnen_am,
            } => Self::FreieRunde(FreieRunde::aus_neu(id, erfasser, name, begonnen_am)),
        }
    }

    /// Rebuild a persisted Runde in either state.
    ///
    /// The closing timestamp *is* the lifecycle, so the adapter passes what it read and the
    /// typestate follows: `None` is a running Runde, `Some` a closed one. The provenance is
    /// the same value the state is chosen from, so a loaded round cannot disagree with itself.
    pub fn from_parts(
        id: RundeId,
        erfasser: Erfasser,
        name: NonEmptyString,
        herkunft: Option<RundenHerkunft>,
        aufbau: Rundenaufbau,
        begonnen_am: DateTime<Utc>,
        abgeschlossen_am: Option<DateTime<Utc>>,
    ) -> Self {
        match (herkunft, abgeschlossen_am) {
            (Some(herkunft), None) => Self::MitParkour(RundeMitParkour::from_parts(
                id,
                erfasser,
                name,
                herkunft,
                aufbau,
                begonnen_am,
            )),
            (None, None) => Self::FreieRunde(FreieRunde::from_parts(
                id,
                erfasser,
                name,
                aufbau,
                begonnen_am,
            )),
            (herkunft, Some(am)) => Self::AbgeschlosseneRunde(AbgeschlosseneRunde::from_parts(
                id,
                erfasser,
                name,
                herkunft,
                aufbau,
                begonnen_am,
                am,
            )),
        }
    }

    pub fn id(&self) -> RundeId {
        match self {
            Self::MitParkour(runde) => runde.id(),
            Self::FreieRunde(runde) => runde.id(),
            Self::AbgeschlosseneRunde(runde) => runde.id(),
        }
    }

    pub fn aufbau(&self) -> &Rundenaufbau {
        match self {
            Self::MitParkour(runde) => runde.aufbau(),
            Self::FreieRunde(runde) => runde.aufbau(),
            Self::AbgeschlosseneRunde(runde) => runde.aufbau(),
        }
    }

    /// Where this round came from: the Parkour pair, or `None` for a free round — closed or
    /// not, a free round never gains a provenance.
    pub fn herkunft(&self) -> Option<&RundenHerkunft> {
        match self {
            Self::MitParkour(runde) => Some(runde.herkunft()),
            Self::FreieRunde(_) => None,
            Self::AbgeschlosseneRunde(runde) => runde.herkunft(),
        }
    }

    /// The round's name: a free round's own, or the captured Parkour's.
    pub fn name(&self) -> &NonEmptyString {
        match self {
            Self::MitParkour(runde) => runde.name(),
            Self::FreieRunde(runde) => runde.name(),
            Self::AbgeschlosseneRunde(runde) => runde.name(),
        }
    }

    pub fn erfasst_von(&self) -> Erfasser {
        match self {
            Self::MitParkour(runde) => runde.erfasst_von(),
            Self::FreieRunde(runde) => runde.erfasst_von(),
            Self::AbgeschlosseneRunde(runde) => runde.erfasst_von(),
        }
    }

    /// The stored lifecycle spelling, taken from the active variant itself.
    pub fn zustand(&self) -> &'static str {
        match self {
            Self::MitParkour(_) => <MitParkourInBearbeitung as RundeTypestate>::zustand(),
            Self::FreieRunde(_) => <FreieRundeInBearbeitung as RundeTypestate>::zustand(),
            Self::AbgeschlosseneRunde(_) => <Abgeschlossen as RundeTypestate>::zustand(),
        }
    }

    pub fn begonnen_am(&self) -> DateTime<Utc> {
        match self {
            Self::MitParkour(runde) => runde.begonnen_am(),
            Self::FreieRunde(runde) => runde.begonnen_am(),
            Self::AbgeschlosseneRunde(runde) => runde.begonnen_am(),
        }
    }

    pub fn abgeschlossen_am(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::MitParkour(_) | Self::FreieRunde(_) => None,
            Self::AbgeschlosseneRunde(runde) => Some(runde.abgeschlossen_am()),
        }
    }

    pub fn gehört_zu(&self, benutzer_id: BenutzerId) -> bool {
        match self {
            Self::MitParkour(runde) => runde.gehört_zu(benutzer_id),
            Self::FreieRunde(runde) => runde.gehört_zu(benutzer_id),
            Self::AbgeschlosseneRunde(runde) => runde.gehört_zu(benutzer_id),
        }
    }

    pub fn ist_in_bearbeitung(&self) -> bool {
        matches!(self, Self::MitParkour(_) | Self::FreieRunde(_))
    }

    /// Whether this round is a free one, whether it still runs or is closed.
    pub fn ist_freie_runde(&self) -> bool {
        self.herkunft().is_none()
    }

    pub fn enthält_ziel(&self, ziel_id: ZielId) -> bool {
        match self {
            Self::MitParkour(runde) => runde.enthält_ziel(ziel_id),
            Self::FreieRunde(runde) => runde.enthält_ziel(ziel_id),
            Self::AbgeschlosseneRunde(runde) => runde.enthält_ziel(ziel_id),
        }
    }

    pub fn anonymisiere(&mut self, am: DateTime<Utc>) {
        match self {
            Self::MitParkour(runde) => runde.anonymisiere(am),
            Self::FreieRunde(runde) => runde.anonymisiere(am),
            Self::AbgeschlosseneRunde(runde) => runde.anonymisiere(am),
        }
    }

    pub fn wertung(&self, schüsse: impl IntoIterator<Item = Schuss>) -> Rundenwertung {
        match self {
            Self::MitParkour(runde) => runde.wertung(schüsse),
            Self::FreieRunde(runde) => runde.wertung(schüsse),
            Self::AbgeschlosseneRunde(runde) => runde.wertung(schüsse),
        }
    }

    pub fn ziel_wertungen(
        &self,
        schüsse: impl IntoIterator<Item = Schuss>,
    ) -> Vec<Option<Zielwertung>> {
        match self {
            Self::MitParkour(runde) => runde.ziel_wertungen(schüsse),
            Self::FreieRunde(runde) => runde.ziel_wertungen(schüsse),
            Self::AbgeschlosseneRunde(runde) => runde.ziel_wertungen(schüsse),
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
    use crate::domain::Ländercode;
    use crate::domain::anlage::{
        AnlageAdresse, AnlageAktiv, AnlageKontakt, NeueAnlage, NeuerParkour, ParkourPflock,
        ParkourPflockId, Zielplatzierung, Zustand,
    };
    use crate::domain::schuss::{NeuerSchuss, SchussId, Wertungszone};
    use uuid::Uuid;

    fn name(wert: &str) -> NonEmptyString {
        NonEmptyString::neu(wert).unwrap()
    }

    /// The master-data id space the fixture captures from. It is deliberately unrelated to
    /// the round-local ids the round mints, so a leak of one into the other is visible.
    const ANLAGE_ZIEL_BASIS: u128 = 10_000;

    fn jetzt() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn ziel_id(id: u128) -> AnlageZielId {
        AnlageZielId(Uuid::from_u128(id))
    }

    fn pflock(id: u128, farbe: &str) -> ParkourPflock {
        ParkourPflock::neu(
            ParkourPflockId(Uuid::from_u128(id)),
            Farbe::neu(farbe).unwrap(),
        )
    }

    fn anlage() -> AnlageAktiv {
        AnlageAktiv::neu(
            AnlageId(Uuid::nil()),
            NeueAnlage::neu(
                name("Breitenstein"),
                AnlageAdresse::neu("Waldweg 1", "3100", "St. Pölten", Ländercode::AT).unwrap(),
                AnlageKontakt::leer(),
            ),
        )
    }

    /// One Anlage with a two-Pflock Parkour over the given targets.
    ///
    /// The master-data targets are named after the caller's label, so a case can tell which
    /// captured Ziel is which without touching the round-local identity.
    fn anlage_mit(ziele: &[(u128, &str)], abstände: &[(u128, u32)]) -> (AnlageAktiv, ParkourId) {
        let mut anlage = anlage();
        for &(id, tiername) in ziele {
            anlage.ziel_hinzufügen(ziel_id(ANLAGE_ZIEL_BASIS + id), name(tiername));
        }
        let parkour_id = ParkourId(Uuid::from_u128(7));
        let platzierungen = ziele
            .iter()
            .map(|&(id, _)| {
                Zielplatzierung::neu(
                    ziel_id(ANLAGE_ZIEL_BASIS + id),
                    abstände
                        .iter()
                        .map(|&(pflock_id, meter)| {
                            (
                                ParkourPflockId(Uuid::from_u128(pflock_id)),
                                Abstand::neu(meter).unwrap(),
                            )
                        })
                        .collect(),
                )
                .unwrap()
            })
            .collect();
        anlage
            .parkour_hinzufügen(
                parkour_id,
                NeuerParkour::neu(
                    name("Waldrunde"),
                    vec![pflock(1, "#c0392b"), pflock(2, "#1b4f9c")],
                    platzierungen,
                )
                .unwrap(),
            )
            .unwrap();
        (anlage, parkour_id)
    }

    /// The captured route of one Parkour over the given targets.
    fn aufbau(ziele: &[(u128, &str)]) -> Rundenaufbau {
        let (anlage, parkour_id) = anlage_mit(ziele, &[(1, 20), (2, 30)]);
        let parkour = anlage.parkour_mit_id(parkour_id).unwrap();
        Rundenaufbau::aus_parkour(parkour, |id| anlage.ziel(id)).unwrap()
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
                ParkourId(Uuid::from_u128(7)),
                name("Waldrunde"),
                aufbau(ziele),
                Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                jetzt(),
            ),
        )
    }

    fn freie_runde() -> Runde {
        Runde::FreieRunde(FreieRunde::aus_neu(
            RundeId(Uuid::nil()),
            Erfasser::Benutzer(BenutzerId(Uuid::nil())),
            name("Wiese daheim"),
            jetzt(),
        ))
    }

    #[test_log::test]
    fn zustand_gibt_fuer_jeden_typestate_den_exakten_persistierten_wert_zurueck() {
        let mit_parkour = runde(&[(10, "Keiler")]);
        let freie = freie_runde();
        let Runde::MitParkour(laufend) = runde(&[(10, "Keiler")]) else {
            panic!("eine Parkour-Runde ist offen");
        };
        let abgeschlossen: Runde = laufend.abschliessen(jetzt()).into();

        assert_eq!(mit_parkour.zustand(), "InBearbeitung");
        assert_eq!(freie.zustand(), "InBearbeitung");
        assert_eq!(abgeschlossen.zustand(), "Abgeschlossen");
    }

    /// The round-local id of the target captured from the master-data target `anlage_ziel`.
    fn ziel_id_zu(runde: &Runde, anlage_ziel: u128) -> ZielId {
        runde
            .aufbau()
            .ziele()
            .iter()
            .find(|ziel| ziel.anlage_ziel_id() == Some(ziel_id(ANLAGE_ZIEL_BASIS + anlage_ziel)))
            .expect("the round captured this target")
            .id()
    }

    fn schuss(runde: &Runde, anlage_ziel: u128, pfeil: usize, zone: Wertungszone) -> Schuss {
        schuss_auf(runde.id(), ziel_id_zu(runde, anlage_ziel), pfeil, zone)
    }

    fn schuss_auf(runde_id: RundeId, ziel_id: ZielId, pfeil: usize, zone: Wertungszone) -> Schuss {
        Schuss::neu(
            SchussId(Uuid::from_u128(pfeil as u128)),
            NeuerSchuss::neu(
                runde_id,
                Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                ziel_id,
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
            ZielId(Uuid::from_u128(99)),
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
                ZielId(Uuid::from_u128(99)),
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
        let Runde::MitParkour(runde) = runde(&[(10, "Keiler")]) else {
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
        assert!(!runde.enthält_ziel(ZielId(Uuid::from_u128(11))));
    }

    #[test_log::test]
    fn anonymisieren_laesst_aufbau_und_herkunft_unberuehrt() {
        let mut runde = runde(&[(10, "Keiler")]);
        let vorher = runde.aufbau().clone();
        let herkunft = runde.herkunft().copied();
        runde.anonymisiere(jetzt());
        assert_eq!(runde.erfasst_von(), Erfasser::Anonymisiert { am: jetzt() });
        assert_eq!(runde.aufbau(), &vorher);
        assert_eq!(runde.herkunft(), herkunft.as_ref());
    }

    #[test_log::test]
    fn from_parts_roundtrip_beider_aufbauten_und_zustaende() {
        let offen = runde(&[(10, "Keiler")]);
        let geladen = Runde::from_parts(
            offen.id(),
            offen.erfasst_von(),
            offen.name().clone(),
            offen.herkunft().copied(),
            offen.aufbau().clone(),
            offen.begonnen_am(),
            offen.abgeschlossen_am(),
        );
        assert_eq!(offen, geladen);

        let freie = freie_runde();
        let geladen = Runde::from_parts(
            freie.id(),
            freie.erfasst_von(),
            freie.name().clone(),
            freie.herkunft().copied(),
            freie.aufbau().clone(),
            freie.begonnen_am(),
            freie.abgeschlossen_am(),
        );
        assert_eq!(freie, geladen);

        let Runde::MitParkour(aktiv) = offen else {
            panic!("eine Parkour-Runde ist offen");
        };
        let geschlossen: Runde = aktiv.abschliessen(jetzt()).into();
        let geladen = Runde::from_parts(
            geschlossen.id(),
            geschlossen.erfasst_von(),
            geschlossen.name().clone(),
            geschlossen.herkunft().copied(),
            geschlossen.aufbau().clone(),
            geschlossen.begonnen_am(),
            geschlossen.abgeschlossen_am(),
        );
        assert_eq!(geschlossen, geladen);

        // The closed *free* round is its own case: its route is the free one's, and its
        // provenance must survive the round trip just as a Parkour round's does (#13).
        let Runde::FreieRunde(mut aktiv) = freie_runde() else {
            panic!("eine freie Runde ist offen");
        };
        aktiv.ziel_hinzufügen(Some(name("Baumstumpf")));
        let geschlossen: Runde = aktiv.abschliessen(jetzt()).into();
        assert_eq!(geschlossen.zustand(), "Abgeschlossen");
        assert!(geschlossen.ist_freie_runde());
        let geladen = Runde::from_parts(
            geschlossen.id(),
            geschlossen.erfasst_von(),
            geschlossen.name().clone(),
            geschlossen.herkunft().copied(),
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
            offen.name().clone(),
            offen.herkunft().copied(),
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
        let Some(herkunft) = runde.herkunft() else {
            panic!("ein Parkour-Aufbau trägt Referenzen");
        };
        assert_eq!(herkunft.anlage_id, AnlageId(Uuid::nil()));
        assert_eq!(herkunft.parkour_id, ParkourId(Uuid::from_u128(7)));
        assert_eq!(runde.name().as_str(), "Waldrunde");
        assert_eq!(runde.aufbau().ziele().len(), 1);
        assert_eq!(
            runde.aufbau().ziele()[0].anlage_ziel_id(),
            Some(ziel_id(ANLAGE_ZIEL_BASIS + 10))
        );
        // The route's own identity is minted by the round, never the master-data one (#12):
        // a leak of `AnlageZielId` into `ZielId` fails right here.
        assert_ne!(
            runde.aufbau().ziele()[0].id().0,
            Uuid::from_u128(ANLAGE_ZIEL_BASIS + 10)
        );
    }

    #[test_log::test]
    fn ac21_eine_freie_runde_hat_weder_anlage_noch_parkour_noch_pfloecke() {
        let runde = freie_runde();
        assert!(runde.herkunft().is_none());
        assert_eq!(runde.name().as_str(), "Wiese daheim");
        assert!(runde.aufbau().ziele().is_empty());
        // Both open states store the same spelling, and the store filters on it (#8).
        assert_eq!(
            runde.zustand(),
            <FreieRundeInBearbeitung as RundeTypestate>::zustand()
        );
    }

    #[test_log::test]
    fn ac22_freie_ziele_werden_fortlaufend_nummeriert_und_nie_wiederverwendet() {
        let Runde::FreieRunde(mut runde) = freie_runde() else {
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
        assert!(runde.ziel_entfernen(ZielId(Uuid::from_u128(99))).is_none());

        let abgeschlossen = runde.abschliessen(jetzt());
        let abgeschlossen: Runde = abgeschlossen.into();
        assert!(!matches!(abgeschlossen, Runde::FreieRunde(_)));
    }

    #[test_log::test]
    fn ac22_abschliessen_behaelt_route_und_herkunft() {
        // The compile-time half of the AC-22 promise — a Parkour round has no route mutator
        // — is `tests/compile_fail/parkour_hat_keinen_ziel_mutator.rs`, because "this method
        // does not exist" can only be proven by failing to compile.
        let Runde::MitParkour(runde) = runde(&[(10, "Keiler")]) else {
            panic!("eine neue Parkour-Runde ist in Bearbeitung");
        };
        let ziel_id = runde.aufbau().ziele()[0].id();
        let aufbau = runde.aufbau().clone();
        let herkunft = *runde.herkunft();
        let geschlossen: Runde = runde.abschliessen(jetzt()).into();
        assert_eq!(geschlossen.zustand(), "Abgeschlossen");
        // Closing erases nothing: the route and the provenance are the round's record.
        assert_eq!(geschlossen.aufbau(), &aufbau);
        assert!(geschlossen.enthält_ziel(ziel_id));
        assert_eq!(geschlossen.herkunft(), Some(&herkunft));
        assert!(!geschlossen.ist_freie_runde());
    }

    #[test_log::test]
    fn ac3_ziele_werden_in_einer_round_localen_route_abgebildet() {
        let Runde::FreieRunde(mut freie) = freie_runde() else {
            panic!("eine freie Runde hat ihren eigenen offenen Typ");
        };
        let id = freie.ziel_hinzufügen(None);
        let ziele = freie.aufbau().ziele();
        assert_eq!(ziele.len(), 1);
        assert_eq!(ziele[0].id(), id);
        assert_eq!(ziele[0].name().as_str(), "Ziel 1");
        assert_eq!(ziele[0].ordinal().get(), 1);
        assert!(ziele[0].pflöcke().is_empty());

        let parkour = runde(&[(10, "Keiler")]);
        let ziel = &parkour.aufbau().ziele()[0];
        assert_eq!(ziel.name().as_str(), "Keiler");
        assert_eq!(ziel.ordinal().get(), 1);
        assert_eq!(ziel.anlage_ziel_id(), Some(ziel_id(ANLAGE_ZIEL_BASIS + 10)));
    }

    #[test_log::test]
    fn ac23_parkour_pflock_wird_gegen_das_ziel_geprueft() {
        let Runde::MitParkour(runde) = runde(&[(10, "Keiler")]) else {
            panic!("eine neue Parkour-Runde ist in Bearbeitung");
        };
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
        assert!(runde.pruefe_position(ziel, None).is_ok());
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
                    ZielId(Uuid::from_u128(99)),
                    Some(Schussposition::Pflock(farbe))
                )
                .is_err()
        );
    }

    #[test_log::test]
    fn ac23_eine_freie_runde_nimmt_jede_farbe_und_jede_distanz() {
        let Runde::FreieRunde(mut runde) = freie_runde() else {
            panic!("eine freie Runde hat ihren eigenen offenen Typ");
        };
        let ziel = runde.ziel_hinzufügen(None);
        // A free round configures no Pflock set, so the colour the archer picked is a
        // position; `Farbe` has already proven it is a colour.
        // Position acceptance is checked by the use case; this typestate only answers
        // whether the target belongs to the route.
        assert!(runde.pruefe_position(ziel).is_ok());
        assert!(runde.pruefe_position(ZielId(Uuid::from_u128(99))).is_err());
    }

    #[test_log::test]
    fn ac24_die_wertung_faltet_ueber_die_freien_ziele() {
        let Runde::FreieRunde(mut runde) = freie_runde() else {
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

    // -------------------------------------------------------------------------------
    // Capture: what a Parkour round freezes at start (AC-6, AC-7, AC-9)
    // -------------------------------------------------------------------------------

    /// An Anlage whose one Ziel carries its own Pflock set, plus a second Ziel that
    /// inherits the Parkour-wide set.
    fn anlage_mit_eigenem_satz() -> (AnlageAktiv, ParkourId) {
        let mut anlage = anlage();
        anlage.ziel_hinzufügen(ziel_id(ANLAGE_ZIEL_BASIS + 10), name("Keiler"));
        anlage.ziel_hinzufügen(ziel_id(ANLAGE_ZIEL_BASIS + 11), name("Reh"));
        let parkour_id = ParkourId(Uuid::from_u128(7));
        anlage
            .parkour_hinzufügen(
                parkour_id,
                NeuerParkour::neu(
                    name("Waldrunde"),
                    vec![pflock(1, "#c0392b"), pflock(2, "#1b4f9c")],
                    vec![
                        // Own set: near-to-far, replaces the Parkour-wide order.
                        Zielplatzierung::neu_mit_eigenen_pflöcken(
                            ziel_id(ANLAGE_ZIEL_BASIS + 10),
                            vec![pflock(11, "#1b4f9c"), pflock(12, "#e8b81c")],
                            vec![
                                (
                                    ParkourPflockId(Uuid::from_u128(11)),
                                    Abstand::neu(15).unwrap(),
                                ),
                                (
                                    ParkourPflockId(Uuid::from_u128(12)),
                                    Abstand::neu(45).unwrap(),
                                ),
                            ],
                        )
                        .unwrap(),
                        Zielplatzierung::neu(ziel_id(ANLAGE_ZIEL_BASIS + 11), Vec::new()).unwrap(),
                    ],
                )
                .unwrap(),
            )
            .unwrap();
        (anlage, parkour_id)
    }

    fn aufbau_von(anlage: &AnlageAktiv, parkour_id: ParkourId) -> Rundenaufbau {
        let parkour = anlage.parkour_mit_id(parkour_id).unwrap();
        Rundenaufbau::aus_parkour(parkour, |id| anlage.ziel(id)).unwrap()
    }

    #[test_log::test]
    fn der_aufbau_traegt_namen_ziele_und_abstaende() {
        let (anlage, parkour_id) = anlage_mit(&[(10, "Keiler")], &[(1, 20), (2, 30)]);
        let aufbau = aufbau_von(&anlage, parkour_id);

        let ziel = &aufbau.ziele()[0];
        assert_eq!(ziel.name().as_str(), "Keiler");
        assert_eq!(ziel.ordinal().get(), 1);
        // Near to far, each distance folded into its own Pflock.
        let farben: Vec<(String, Option<u32>)> = ziel
            .pflöcke()
            .iter()
            .map(|pflock| {
                (
                    pflock.farbe().to_hex(),
                    pflock.abstand().map(|abstand| abstand.meter()),
                )
            })
            .collect();
        assert_eq!(
            farben,
            [
                ("#c0392b".to_owned(), Some(20)),
                ("#1b4f9c".to_owned(), Some(30)),
            ]
        );
    }

    #[test_log::test]
    fn der_aufbau_enthaelt_nur_platzierte_ziele() {
        let (anlage, parkour_id) = anlage_mit(&[(10, "Keiler")], &[(1, 20)]);
        let aufbau = aufbau_von(&anlage, parkour_id);
        assert_eq!(aufbau.ziele().len(), 1);
        assert_eq!(
            aufbau.ziele()[0].anlage_ziel_id(),
            Some(ziel_id(ANLAGE_ZIEL_BASIS + 10))
        );
        assert!(!aufbau.enthält_ziel(ZielId(Uuid::from_u128(99))));
    }

    #[test_log::test]
    fn ein_pflock_ohne_abstand_bleibt_ohne_abstand() {
        // Only the near Pflock carries a distance: the far one keeps `None`, never a zero.
        let (anlage, parkour_id) = anlage_mit(&[(10, "Keiler")], &[(1, 20)]);
        let aufbau = aufbau_von(&anlage, parkour_id);
        let pflöcke = aufbau.ziele()[0].pflöcke();
        assert_eq!(pflöcke.len(), 2);
        assert_eq!(pflöcke[0].abstand(), Some(Abstand::neu(20).unwrap()));
        assert_eq!(pflöcke[1].abstand(), None);
    }

    #[test_log::test]
    fn ein_unbekanntes_platziertes_ziel_wird_abgelehnt() {
        let (anlage, parkour_id) = anlage_mit(&[(10, "Keiler")], &[]);
        let parkour = anlage.parkour_mit_id(parkour_id).unwrap();
        let ergebnis = Rundenaufbau::aus_parkour(parkour, |_| None);
        assert!(matches!(ergebnis, Err(ParkourFehler::ZielNichtGefunden)));
    }

    #[test_log::test]
    fn ac6_der_aufbau_traegt_die_zielpfloecke() {
        let (anlage, parkour_id) = anlage_mit_eigenem_satz();
        let aufbau = aufbau_von(&anlage, parkour_id);

        let eigenes = &aufbau.ziele()[0];
        let eigene: Vec<String> = eigenes
            .pflöcke()
            .iter()
            .map(|pflock| pflock.farbe().to_hex())
            .collect();
        // Replacing, not merging: the Parkour-wide "#c0392b" is not in this Ziel's set.
        assert_eq!(eigene, ["#1b4f9c", "#e8b81c"]);
        assert_eq!(
            eigenes.pflöcke()[0].abstand(),
            Some(Abstand::neu(15).unwrap())
        );

        let erbe = &aufbau.ziele()[1];
        let geerbte: Vec<String> = erbe
            .pflöcke()
            .iter()
            .map(|pflock| pflock.farbe().to_hex())
            .collect();
        assert_eq!(geerbte, ["#c0392b", "#1b4f9c"]);
    }

    #[test_log::test]
    fn ac6_ein_pflock_gilt_nur_an_seinem_ziel() {
        let (anlage, parkour_id) = anlage_mit_eigenem_satz();
        let aufbau = aufbau_von(&anlage, parkour_id);
        let eigen_blau = Farbe::neu("#1b4f9c").unwrap();
        let eigen_gelb = Farbe::neu("#e8b81c").unwrap();
        let parkour_rot = Farbe::neu("#c0392b").unwrap();

        // The own-set Ziel sees both colours of its own set, not the Parkour-wide one.
        assert!(aufbau.ziele()[0].hat_pflock(eigen_blau));
        assert!(!aufbau.ziele()[0].hat_pflock(parkour_rot));
        // The inheriting Ziel still sees the Parkour-wide set only.
        assert!(aufbau.ziele()[1].hat_pflock(parkour_rot));
        // ...and a colour that exists only as another Ziel's own Pflock is not selectable
        // there, even though it sits outside every captured Parkour-wide set.
        assert!(!aufbau.ziele()[1].hat_pflock(eigen_gelb));
    }

    #[test_log::test]
    fn ac9_der_aufbau_bleibt_von_einer_spaeteren_satzänderung_unberuehrt() {
        let (mut anlage, parkour_id) = anlage_mit_eigenem_satz();
        let aufbau = aufbau_von(&anlage, parkour_id);

        // Swap the Ziel's own set for the inherited one after capture.
        anlage
            .parkour_setup_ändern(
                parkour_id,
                vec![pflock(1, "#c0392b"), pflock(2, "#1b4f9c")],
                vec![
                    Zielplatzierung::neu(ziel_id(ANLAGE_ZIEL_BASIS + 10), Vec::new()).unwrap(),
                    Zielplatzierung::neu(ziel_id(ANLAGE_ZIEL_BASIS + 11), Vec::new()).unwrap(),
                ],
            )
            .unwrap();

        assert_eq!(
            aufbau.ziele()[0].pflöcke().len(),
            2,
            "AC-9: ein späterer Satz ändert den gefangenen Aufbau nicht"
        );
        assert!(
            !aufbau.ziele()[0].hat_pflock(Farbe::neu("#c0392b").unwrap()),
            "der eigene Satz bleibt der eigene Satz"
        );
        assert_eq!(aufbau.ziele()[0].pflöcke()[0].farbe().to_hex(), "#1b4f9c");
        // The freshly captured route *does* follow the change: the copy is the point.
        let neu = aufbau_von(&anlage, parkour_id);
        assert!(neu.ziele()[0].hat_pflock(Farbe::neu("#c0392b").unwrap()));
    }

    #[test_log::test]
    fn ein_archiviertes_ziel_bleibt_im_aufbau() {
        let (mut anlage, parkour_id) = anlage_mit(&[(10, "Keiler")], &[(1, 20)]);
        let aufbau = aufbau_von(&anlage, parkour_id);
        anlage
            .ziel_archivieren(ziel_id(ANLAGE_ZIEL_BASIS + 10))
            .unwrap();
        assert_eq!(
            aufbau.ziele()[0].name().as_str(),
            "Keiler",
            "ein archiviertes Ziel bleibt im gefangenen Aufbau"
        );
        assert_eq!(aufbau.ziele()[0].ordinal().get(), 1);
        let _ = Zustand::Aktiv;
    }
}
