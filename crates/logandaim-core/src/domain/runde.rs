//! One logged Runde at one Parkour, holding its captured snapshot.
//!
//! A `Runde` is a standalone aggregate root. It never holds a collection of `Schuss`:
//! each `Schuss` carries `runde_id`, so two aggregates stay two aggregates and no use case
//! writes both. The Runde-wide Wertung folds the per-Ziel results once, in the order of the
//! captured route (SPEC-wertung rule 6, AC-8).
//!
//! The root is a **typestate**: only `RundeIn<InBearbeitung>` can be closed, and the closing
//! timestamp lives in the `Abgeschlossen` marker, so "when was it closed?" is not an
//! `Option` a reader must interpret.

use chrono::{DateTime, NaiveDate, Utc};

use super::anlage::{AnlageId, ParkourId, ParkourSnapshot, ZielId};
use super::konto::BenutzerId;
use super::schuss::{Erfasser, Schuss, ZielSchüsse, Zielwertung};

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

/// Creation input of a Runde: everything except its identity and lifecycle state.
///
/// The snapshot is a value copy taken from the `Anlage` in the same transaction, so a
/// later master-data edit cannot reach it (AC-7, AC-9).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeueRunde {
    anlage_id: AnlageId,
    parkour_id: ParkourId,
    erfasst_von: Erfasser,
    snapshot: ParkourSnapshot,
    datum: NaiveDate,
    begonnen_am: DateTime<Utc>,
}

impl NeueRunde {
    pub fn neu(
        anlage_id: AnlageId,
        erfasst_von: Erfasser,
        snapshot: ParkourSnapshot,
        datum: NaiveDate,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            anlage_id,
            parkour_id: snapshot.parkour_id(),
            erfasst_von,
            snapshot,
            datum,
            begonnen_am,
        }
    }

    pub fn anlage_id(&self) -> AnlageId {
        self.anlage_id
    }

    pub fn parkour_id(&self) -> ParkourId {
        self.parkour_id
    }

    pub fn erfasst_von(&self) -> Erfasser {
        self.erfasst_von
    }

    pub fn snapshot(&self) -> &ParkourSnapshot {
        &self.snapshot
    }

    pub fn datum(&self) -> NaiveDate {
        self.datum
    }

    pub fn begonnen_am(&self) -> DateTime<Utc> {
        self.begonnen_am
    }
}

/// One logged Runde: who and where, the frozen setup, and whether it is still running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RundeIn<S> {
    id: RundeId,
    anlage_id: AnlageId,
    parkour_id: ParkourId,
    erfasst_von: Erfasser,
    snapshot: ParkourSnapshot,
    datum: NaiveDate,
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

    pub fn anlage_id(&self) -> AnlageId {
        self.anlage_id
    }

    pub fn parkour_id(&self) -> ParkourId {
        self.parkour_id
    }

    pub fn erfasst_von(&self) -> Erfasser {
        self.erfasst_von
    }

    pub fn snapshot(&self) -> &ParkourSnapshot {
        &self.snapshot
    }

    pub fn datum(&self) -> NaiveDate {
        self.datum
    }

    pub fn begonnen_am(&self) -> DateTime<Utc> {
        self.begonnen_am
    }

    pub fn gehört_zu(&self, benutzer_id: BenutzerId) -> bool {
        self.erfasst_von.benutzer() == Some(benutzer_id)
    }

    /// Whether the captured route contains this Ziel (the AC-9 contract a Schuss obeys).
    pub fn enthält_ziel(&self, ziel_id: ZielId) -> bool {
        self.snapshot.enthält_ziel(ziel_id)
    }

    /// Replace the owner with [`Erfasser::Anonymisiert`] (SPEC-datenschutz retention).
    /// The snapshot and every Schuss stay untouched (NFR-3).
    pub fn anonymisiere(&mut self, am: DateTime<Utc>) {
        self.erfasst_von = Erfasser::Anonymisiert { am };
    }

    /// Fold the given Schüsse into one Wertung per captured Ziel, in route order.
    ///
    /// `None` marks a Ziel no Schuss was logged at (AC-6). The slice pairs with the
    /// snapshot's own Ziel order, so a reader zips the two instead of re-deriving a fold.
    pub fn ziel_wertungen(
        &self,
        schüsse: impl IntoIterator<Item = Schuss>,
    ) -> Vec<Option<Zielwertung>> {
        let eigene: Vec<Schuss> = schüsse
            .into_iter()
            .filter(|schuss| schuss.runde_id() == self.id)
            .collect();
        self.snapshot
            .ziele()
            .iter()
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
        let je_ziel = self.snapshot.ziele().iter().filter_map(|ziel| {
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
            anlage_id: neu.anlage_id,
            parkour_id: neu.parkour_id,
            erfasst_von: neu.erfasst_von,
            snapshot: neu.snapshot,
            datum: neu.datum,
            begonnen_am: neu.begonnen_am,
            state: InBearbeitung,
        }
    }

    /// Rebuild a persisted, still running Runde.
    pub fn from_parts(
        id: RundeId,
        anlage_id: AnlageId,
        erfasst_von: Erfasser,
        snapshot: ParkourSnapshot,
        datum: NaiveDate,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            anlage_id,
            parkour_id: snapshot.parkour_id(),
            erfasst_von,
            snapshot,
            datum,
            begonnen_am,
            state: InBearbeitung,
        }
    }

    /// Close the Runde: no further Schuss may be registered.
    pub fn abschliessen(self, am: DateTime<Utc>) -> RundeAbgeschlossen {
        RundeIn {
            id: self.id,
            anlage_id: self.anlage_id,
            parkour_id: self.parkour_id,
            erfasst_von: self.erfasst_von,
            snapshot: self.snapshot,
            datum: self.datum,
            begonnen_am: self.begonnen_am,
            state: Abgeschlossen {
                abgeschlossen_am: am,
            },
        }
    }
}

impl RundeAbgeschlossen {
    /// Rebuild a persisted, closed Runde.
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: RundeId,
        anlage_id: AnlageId,
        erfasst_von: Erfasser,
        snapshot: ParkourSnapshot,
        datum: NaiveDate,
        begonnen_am: DateTime<Utc>,
        abgeschlossen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            anlage_id,
            parkour_id: snapshot.parkour_id(),
            erfasst_von,
            snapshot,
            datum,
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
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: RundeId,
        anlage_id: AnlageId,
        erfasst_von: Erfasser,
        snapshot: ParkourSnapshot,
        datum: NaiveDate,
        begonnen_am: DateTime<Utc>,
        abgeschlossen_am: Option<DateTime<Utc>>,
    ) -> Self {
        match abgeschlossen_am {
            None => Self::InBearbeitung(RundeInBearbeitung::from_parts(
                id,
                anlage_id,
                erfasst_von,
                snapshot,
                datum,
                begonnen_am,
            )),
            Some(am) => Self::Abgeschlossen(RundeAbgeschlossen::from_parts(
                id,
                anlage_id,
                erfasst_von,
                snapshot,
                datum,
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

    pub fn anlage_id(&self) -> AnlageId {
        match self {
            Self::InBearbeitung(runde) => runde.anlage_id(),
            Self::Abgeschlossen(runde) => runde.anlage_id(),
        }
    }

    pub fn parkour_id(&self) -> ParkourId {
        match self {
            Self::InBearbeitung(runde) => runde.parkour_id(),
            Self::Abgeschlossen(runde) => runde.parkour_id(),
        }
    }

    pub fn erfasst_von(&self) -> Erfasser {
        match self {
            Self::InBearbeitung(runde) => runde.erfasst_von(),
            Self::Abgeschlossen(runde) => runde.erfasst_von(),
        }
    }

    pub fn snapshot(&self) -> &ParkourSnapshot {
        match self {
            Self::InBearbeitung(runde) => runde.snapshot(),
            Self::Abgeschlossen(runde) => runde.snapshot(),
        }
    }

    /// The stored lifecycle spelling, taken from the active variant itself.
    pub fn zustand(&self) -> &'static str {
        match self {
            Self::InBearbeitung(runde) => runde.state.as_str(),
            Self::Abgeschlossen(runde) => runde.state.as_str(),
        }
    }

    pub fn datum(&self) -> NaiveDate {
        match self {
            Self::InBearbeitung(runde) => runde.datum(),
            Self::Abgeschlossen(runde) => runde.datum(),
        }
    }

    pub fn mit_datum(mut self, datum: NaiveDate) -> Self {
        match &mut self {
            Self::InBearbeitung(runde) => runde.datum = datum,
            Self::Abgeschlossen(runde) => runde.datum = datum,
        }
        self
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

    /// One Wertung per captured Ziel, in route order. Delegated like [`Runde::wertung`].
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
    use crate::domain::anlage::{PflockSnapshot, ZielSnapshot};
    use crate::domain::common::{Farbe, NonEmptyString};
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
                jetzt().date_naive(),
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
    fn anonymisieren_laesst_den_snapshot_unberuehrt() {
        let mut runde = runde(&[(10, "Keiler")]);
        let vorher = runde.snapshot().clone();
        runde.anonymisiere(jetzt());
        assert_eq!(runde.erfasst_von(), Erfasser::Anonymisiert { am: jetzt() });
        assert_eq!(runde.snapshot(), &vorher);
    }

    #[test_log::test]
    fn from_parts_roundtrip_beider_zustaende() {
        let offen = runde(&[(10, "Keiler")]);
        let geladen = Runde::from_parts(
            offen.id(),
            offen.anlage_id(),
            offen.erfasst_von(),
            offen.snapshot().clone(),
            offen.datum(),
            offen.begonnen_am(),
            offen.abgeschlossen_am(),
        );
        assert_eq!(offen, geladen);

        let Runde::InBearbeitung(aktiv) = offen else {
            panic!("offen");
        };
        let geschlossen: Runde = aktiv.abschliessen(jetzt()).into();
        let geladen = Runde::from_parts(
            geschlossen.id(),
            geschlossen.anlage_id(),
            geschlossen.erfasst_von(),
            geschlossen.snapshot().clone(),
            geschlossen.datum(),
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
            offen.anlage_id(),
            offen.erfasst_von(),
            offen.snapshot().clone(),
            offen.datum(),
            offen.begonnen_am(),
            Some(jetzt()),
        );
        assert_eq!(geladen.zustand(), "Abgeschlossen");
        assert_eq!(geladen.abgeschlossen_am(), Some(jetzt()));
    }
}
