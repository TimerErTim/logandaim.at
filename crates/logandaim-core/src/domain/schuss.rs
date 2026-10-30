//! One logged arrow and the per-Ziel Wertung folded from a set of them.
//!
//! Scoring lives here because this aggregate owns the data it folds: a `Zielwertung` is a
//! summary of one Ziel's `Schuss` rows, and those rows belong to `Schuss`. The Runde-wide
//! total belongs to `Runde`, which owns the ordered Ziel list (SPEC-wertung rule 6, AC-8).

use std::num::NonZeroUsize;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::konto::BenutzerId;

/// Identity of one physical Ziel.
///
/// Re-exported from the `anlage` aggregate: a `Schuss` references a Ziel by identity
/// only, never by loading the entity (SPEC-stammdaten rule 9).
pub use super::anlage::ZielId;

/// Identity of one logged Schuss.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchussId(pub Uuid);

/// Identity of the Runde a Schuss belongs to. The Schuss carries the link, so a Runde
/// never holds a mutable child collection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RundeId(pub Uuid);

/// Scoring zone hit by one Schuss on a 3D target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wertungszone {
    Spot,
    Kill,
    Body,
    Miss,
}

impl Wertungszone {
    pub fn ist_treffer(self) -> bool {
        !matches!(self, Self::Miss)
    }

    /// Single stored spelling, shared by every adapter.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Spot => "Spot",
            Self::Kill => "Kill",
            Self::Body => "Body",
            Self::Miss => "Miss",
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("die Wertungszone ist unbekannt")]
pub struct WertungszoneUnbekannt;

impl std::str::FromStr for Wertungszone {
    type Err = WertungszoneUnbekannt;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Spot" => Ok(Self::Spot),
            "Kill" => Ok(Self::Kill),
            "Body" => Ok(Self::Body),
            "Miss" => Ok(Self::Miss),
            _ => Err(WertungszoneUnbekannt),
        }
    }
}

/// Who logged a Schuss. The writing identity, not an assignment that can change later.
///
/// Two variants with no path back to [`Benutzer`](super::konto::Benutzer): once a row is
/// anonymised, re-attribution cannot be expressed (SPEC-datenschutz retention rule).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Erfasser {
    Benutzer(BenutzerId),
    Anonymisiert { am: DateTime<Utc> },
}

impl Erfasser {
    /// The writing Benutzer, while the row still has one.
    pub fn benutzer(self) -> Option<BenutzerId> {
        match self {
            Self::Benutzer(id) => Some(id),
            Self::Anonymisiert { .. } => None,
        }
    }

    /// When the row was anonymised, if it was.
    pub fn anonymisiert_am(self) -> Option<DateTime<Utc>> {
        match self {
            Self::Benutzer(_) => None,
            Self::Anonymisiert { am } => Some(am),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SchussFehler {
    #[error("der Pfeil ist 1-basiert, war aber 0")]
    PfeilNull,
}

/// Creation input: everything of a Schuss except its identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuerSchuss {
    runde_id: RundeId,
    erfasser: Erfasser,
    ziel_id: ZielId,
    pfeil: NonZeroUsize,
    wertungszone: Wertungszone,
}

impl NeuerSchuss {
    /// `pfeil` is 1-based. `0` is rejected as [`SchussFehler::PfeilNull`] (AC-6).
    pub fn neu(
        runde_id: RundeId,
        erfasser: Erfasser,
        ziel_id: ZielId,
        pfeil: usize,
        wertungszone: Wertungszone,
    ) -> Result<Self, SchussFehler> {
        let pfeil = NonZeroUsize::new(pfeil).ok_or(SchussFehler::PfeilNull)?;
        Ok(Self {
            runde_id,
            erfasser,
            ziel_id,
            pfeil,
            wertungszone,
        })
    }

    pub fn runde_id(&self) -> RundeId {
        self.runde_id
    }

    pub fn erfasser(&self) -> Erfasser {
        self.erfasser
    }

    pub fn ziel_id(&self) -> ZielId {
        self.ziel_id
    }

    pub fn pfeil(&self) -> NonZeroUsize {
        self.pfeil
    }

    pub fn wertungszone(&self) -> Wertungszone {
        self.wertungszone
    }
}

/// One arrow shot at one Ziel.
///
/// The only aggregate invariant is `pfeil != 0`; membership in a Runde is a use-case
/// concern, validated against that Runde's snapshot (AC-9). The Erfasser is fixed at
/// construction: there is no mutator that could reassign it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schuss {
    id: SchussId,
    runde_id: RundeId,
    erfasser: Erfasser,
    ziel_id: ZielId,
    pfeil: NonZeroUsize,
    wertungszone: Wertungszone,
}

impl Schuss {
    /// Fresh Schuss with a caller-provided identity (the repository assigns the ID).
    pub fn neu(id: SchussId, neu: NeuerSchuss) -> Self {
        Self {
            id,
            runde_id: neu.runde_id,
            erfasser: neu.erfasser,
            ziel_id: neu.ziel_id,
            pfeil: neu.pfeil,
            wertungszone: neu.wertungszone,
        }
    }

    /// Rebuild a persisted Schuss, including its Erfasser. Fails only when the stored
    /// Pfeil is invalid (`0`).
    pub fn from_parts(
        id: SchussId,
        runde_id: RundeId,
        erfasser: Erfasser,
        ziel_id: ZielId,
        pfeil: usize,
        wertungszone: Wertungszone,
    ) -> Result<Self, SchussFehler> {
        let neu = NeuerSchuss::neu(runde_id, erfasser, ziel_id, pfeil, wertungszone)?;
        Ok(Self::neu(id, neu))
    }

    pub fn id(&self) -> SchussId {
        self.id
    }

    pub fn runde_id(&self) -> RundeId {
        self.runde_id
    }

    pub fn erfasser(&self) -> Erfasser {
        self.erfasser
    }

    pub fn ziel_id(&self) -> ZielId {
        self.ziel_id
    }

    /// 1-based position of the arrow at its Ziel.
    pub fn pfeil(&self) -> NonZeroUsize {
        self.pfeil
    }

    pub fn wertungszone(&self) -> Wertungszone {
        self.wertungszone
    }

    /// Classic 3-arrow table. Pfeil 4 and later score 0.
    pub fn punkte(&self) -> u32 {
        match (self.pfeil.get(), self.wertungszone) {
            (_, Wertungszone::Miss) => 0,
            (1, Wertungszone::Spot) => 20,
            (1, Wertungszone::Kill) => 18,
            (1, Wertungszone::Body) => 16,
            (2, Wertungszone::Spot) => 14,
            (2, Wertungszone::Kill) => 12,
            (2, Wertungszone::Body) => 10,
            (3, Wertungszone::Spot) => 8,
            (3, Wertungszone::Kill) => 6,
            (3, Wertungszone::Body) => 4,
            _ => 0,
        }
    }

    /// Replace the owner with [`Erfasser::Anonymisiert`] (SPEC-datenschutz retention).
    pub fn anonymisiere(&mut self, am: DateTime<Utc>) {
        self.erfasser = Erfasser::Anonymisiert { am };
    }
}

/// The Wertung of every Schuss at one Ziel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Zielwertung {
    erster_treffer: u32,
    mehrschuss: u32,
    erster_pfeil_getroffen: bool,
    anzahl_schüsse: usize,
}

impl Zielwertung {
    /// Only the first Treffer counts (AC-3).
    pub fn erster_treffer(self) -> u32 {
        self.erster_treffer
    }

    /// Every Schuss counts by its Pfeil position (AC-4).
    pub fn mehrschuss(self) -> u32 {
        self.mehrschuss
    }

    /// Whether Pfeil 1 hit the target (AC-7).
    pub fn erster_pfeil_getroffen(self) -> bool {
        self.erster_pfeil_getroffen
    }

    pub fn anzahl_schüsse(self) -> usize {
        self.anzahl_schüsse
    }
}

/// Every Schuss at one Ziel, ordered by Pfeil.
///
/// The collection is the aggregator: it knows the fold, so `Runde` never re-implements
/// the scoring rule. An empty collection has no Wertung at all (AC-6), which is why the
/// Wertung is an `Option` rather than a zeroed [`Zielwertung`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZielSchüsse {
    ziel_id: ZielId,
    schüsse: Vec<Schuss>,
}

#[derive(Debug, thiserror::Error)]
pub enum ZielSchüsseFehler {
    #[error("ein Schuss dieser Menge gehört zu einem anderen Ziel")]
    FremdesZiel,
}

impl ZielSchüsse {
    /// Sort the given Schüsse by Pfeil, so the fold is order-independent. Every Schuss
    /// must name this Ziel: a wrong row is refused rather than silently counted.
    pub fn für_ziel(
        ziel_id: ZielId,
        schüsse: impl IntoIterator<Item = Schuss>,
    ) -> Result<Self, ZielSchüsseFehler> {
        let mut schüsse: Vec<_> = schüsse.into_iter().collect();
        if schüsse.iter().any(|schuss| schuss.ziel_id() != ziel_id) {
            return Err(ZielSchüsseFehler::FremdesZiel);
        }
        schüsse.sort_by_key(|schuss| schuss.pfeil());
        Ok(Self { ziel_id, schüsse })
    }

    pub fn ziel_id(&self) -> ZielId {
        self.ziel_id
    }

    pub fn schüsse(&self) -> &[Schuss] {
        &self.schüsse
    }

    /// `None` for an empty Ziel: a Ziel with no Schuss has no Wertung yet.
    pub fn wertung(&self) -> Option<Zielwertung> {
        if self.schüsse.is_empty() {
            return None;
        }
        let erster_treffer = self
            .schüsse
            .iter()
            .find(|schuss| schuss.wertungszone().ist_treffer())
            .map_or(0, Schuss::punkte);
        let mehrschuss = self.schüsse.iter().map(Schuss::punkte).sum();
        let erster_pfeil_getroffen = self
            .schüsse
            .iter()
            .any(|schuss| schuss.pfeil().get() == 1 && schuss.wertungszone().ist_treffer());

        Some(Zielwertung {
            erster_treffer,
            mehrschuss,
            erster_pfeil_getroffen,
            anzahl_schüsse: self.schüsse.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Wertungszone::{Body, Kill, Miss, Spot};
    use super::*;

    fn pfeil(n: usize) -> NonZeroUsize {
        NonZeroUsize::new(n).expect("test fixture uses 1-based Pfeile")
    }

    fn ziel() -> ZielId {
        ZielId(Uuid::from_u128(1))
    }

    fn schuss(pfeil: usize, zone: Wertungszone) -> Schuss {
        let neu = NeuerSchuss::neu(
            RundeId(Uuid::nil()),
            Erfasser::Anonymisiert { am: Utc::now() },
            ziel(),
            pfeil,
            zone,
        )
        .unwrap();
        Schuss::neu(SchussId(Uuid::from_u128(pfeil as u128)), neu)
    }

    fn mengen(schüsse: &[(usize, Wertungszone)]) -> ZielSchüsse {
        ZielSchüsse::für_ziel(
            ziel(),
            schüsse.iter().map(|&(pfeil, zone)| schuss(pfeil, zone)),
        )
        .unwrap()
    }

    fn wertung(schüsse: &[(usize, Wertungszone)]) -> Zielwertung {
        mengen(schüsse).wertung().expect("nicht leer")
    }

    #[test_log::test]
    fn ac1_punktetabelle() {
        let tabelle = [
            (1, [20, 18, 16, 0]),
            (2, [14, 12, 10, 0]),
            (3, [8, 6, 4, 0]),
        ];
        for (nummer, erwartet) in tabelle {
            for (zone, punkte) in [Spot, Kill, Body, Miss].into_iter().zip(erwartet) {
                assert_eq!(
                    schuss(nummer, zone).punkte(),
                    punkte,
                    "Pfeil {nummer} {zone:?}"
                );
            }
        }
        assert_eq!(schuss(4, Kill).punkte(), 0);
        assert_eq!(pfeil(2).get(), 2);
    }

    #[test_log::test]
    fn ac2_drei_fehlschüsse_null() {
        let wertung = wertung(&[(1, Miss), (2, Miss), (3, Miss)]);
        assert_eq!(wertung.erster_treffer(), 0);
        assert_eq!(wertung.mehrschuss(), 0);
    }

    #[test_log::test]
    fn ac3_erster_treffer() {
        assert_eq!(wertung(&[(1, Kill)]).erster_treffer(), 18);
        assert_eq!(wertung(&[(1, Miss), (2, Spot)]).erster_treffer(), 14);
        assert_eq!(
            wertung(&[(1, Miss), (2, Miss), (3, Body)]).erster_treffer(),
            4
        );
        assert_eq!(wertung(&[(1, Body), (2, Kill)]).erster_treffer(), 16);
    }

    #[test_log::test]
    fn ac4_mehrschuss() {
        assert_eq!(wertung(&[(1, Kill), (2, Kill), (3, Spot)]).mehrschuss(), 38);
        assert_eq!(wertung(&[(1, Body), (2, Kill)]).mehrschuss(), 28);
        assert_eq!(wertung(&[(1, Miss), (2, Spot)]).mehrschuss(), 14);
    }

    #[test_log::test]
    fn ac5_ab_viertem_pfeil_null() {
        let wertung = wertung(&[(1, Miss), (2, Miss), (3, Miss), (4, Kill)]);
        assert_eq!(wertung.erster_treffer(), 0);
        assert_eq!(wertung.mehrschuss(), 0);
        // A Schuss at Pfeil 4 still counts in the hit statistics.
        assert_eq!(wertung.anzahl_schüsse(), 4);
    }

    #[test_log::test]
    fn ac6_leer_ohne_wertung_sonst_beliebig_lang() {
        assert!(
            ZielSchüsse::für_ziel(ziel(), [])
                .unwrap()
                .wertung()
                .is_none()
        );
        let lang: Vec<_> = (1..=12).map(|nummer| (nummer, Miss)).collect();
        assert_eq!(wertung(&lang).anzahl_schüsse(), 12);
    }

    #[test_log::test]
    fn ac7_erster_pfeil_getroffen() {
        assert!(wertung(&[(1, Body)]).erster_pfeil_getroffen());
        assert!(wertung(&[(1, Spot), (2, Miss)]).erster_pfeil_getroffen());
        assert!(!wertung(&[(1, Miss), (2, Spot)]).erster_pfeil_getroffen());
    }

    #[test_log::test]
    fn die_faltung_ist_unabhängig_von_der_eingabereihenfolge() {
        let sortiert = wertung(&[(1, Kill), (2, Kill), (3, Spot)]);
        let unsortiert = wertung(&[(3, Spot), (1, Kill), (2, Kill)]);
        assert_eq!(sortiert, unsortiert);
        assert_eq!(sortiert.erster_treffer(), 18);
        assert_eq!(sortiert.mehrschuss(), 38);
    }

    #[test_log::test]
    fn ein_schuss_eines_anderen_ziels_wird_abgelehnt() {
        let fremd = Schuss::neu(
            SchussId(Uuid::from_u128(2)),
            NeuerSchuss::neu(
                RundeId(Uuid::nil()),
                Erfasser::Anonymisiert { am: Utc::now() },
                ZielId(Uuid::from_u128(99)),
                1,
                Kill,
            )
            .unwrap(),
        );
        assert!(matches!(
            ZielSchüsse::für_ziel(ziel(), [fremd]),
            Err(ZielSchüsseFehler::FremdesZiel)
        ));
    }

    #[test_log::test]
    fn pfeil_null_wird_abgelehnt() {
        let ergebnis = NeuerSchuss::neu(
            RundeId(Uuid::nil()),
            Erfasser::Anonymisiert { am: Utc::now() },
            ziel(),
            0,
            Spot,
        );
        assert!(matches!(ergebnis, Err(SchussFehler::PfeilNull)));
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let neu = NeuerSchuss::neu(
            RundeId(Uuid::nil()),
            Erfasser::Benutzer(BenutzerId(Uuid::nil())),
            ziel(),
            2,
            Body,
        )
        .unwrap();
        let original = Schuss::neu(SchussId(Uuid::nil()), neu);
        let geladen = Schuss::from_parts(
            original.id(),
            original.runde_id(),
            original.erfasser(),
            original.ziel_id(),
            2,
            Body,
        )
        .unwrap();
        assert_eq!(original, geladen);
    }

    #[test_log::test]
    fn anonymisieren_entfernt_den_benutzer() {
        let mut schuss = Schuss::neu(
            SchussId(Uuid::nil()),
            NeuerSchuss::neu(
                RundeId(Uuid::nil()),
                Erfasser::Benutzer(BenutzerId(Uuid::nil())),
                ziel(),
                1,
                Kill,
            )
            .unwrap(),
        );
        let am = Utc::now();
        schuss.anonymisiere(am);
        assert_eq!(schuss.erfasser(), Erfasser::Anonymisiert { am });
        assert_eq!(schuss.erfasser().benutzer(), None);
    }
}
