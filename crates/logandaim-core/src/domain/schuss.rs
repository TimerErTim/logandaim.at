//! One logged arrow and the per-Ziel Wertung folded from a set of them.
//!
//! Scoring lives here because this aggregate owns the data it folds: a `Zielergebnis`
//! is a summary of one Ziel's `Schuss` rows, and those rows belong to `Schuss`. The
//! Runde-wide total belongs to `Runde`, which owns the ordered Ziel list
//! (SPEC-wertung rule 6, AC-8).

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

/// Points one arrow yields under the classic 3-arrow table.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Punkte(u32);

impl Punkte {
    pub const NULL: Self = Self(0);

    pub fn value(self) -> u32 {
        self.0
    }

    /// Classic 3-arrow table. `pfeil` is 1-based; Pfeil 4 and later score 0.
    pub fn für_schuss(pfeil: NonZeroUsize, zone: Wertungszone) -> Self {
        let punkte = match (pfeil.get(), zone) {
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
        };
        Self(punkte)
    }
}

impl std::ops::Add for Punkte {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::iter::Sum for Punkte {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::NULL, std::ops::Add::add)
    }
}

/// Who a logged row belongs to.
///
/// Two variants with no path back to [`Benutzer`](super::konto::Benutzer): once a row is
/// anonymised, re-attribution cannot be expressed (SPEC-datenschutz retention rule).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zuordnung {
    Benutzer(BenutzerId),
    Anonymisiert { am: DateTime<Utc> },
}

impl Zuordnung {
    /// The owning Benutzer, while the row still has one.
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
    #[error("der Schuss gehört nicht zur angegebenen Runde")]
    FalscheRunde,
}

/// Creation input: everything of a Schuss except its identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuerSchuss {
    runde_id: RundeId,
    zuordnung: Zuordnung,
    ziel_id: ZielId,
    pfeil: NonZeroUsize,
    wertungszone: Wertungszone,
}

impl NeuerSchuss {
    /// `pfeil` is 1-based. `0` is rejected as [`SchussFehler::PfeilNull`] (AC-6).
    pub fn neu(
        runde_id: RundeId,
        zuordnung: Zuordnung,
        ziel_id: ZielId,
        pfeil: usize,
        wertungszone: Wertungszone,
    ) -> Result<Self, SchussFehler> {
        let pfeil = NonZeroUsize::new(pfeil).ok_or(SchussFehler::PfeilNull)?;
        Ok(Self {
            runde_id,
            zuordnung,
            ziel_id,
            pfeil,
            wertungszone,
        })
    }

    pub fn runde_id(&self) -> RundeId {
        self.runde_id
    }

    pub fn zuordnung(&self) -> Zuordnung {
        self.zuordnung
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
/// concern, validated against that Runde's snapshot (AC-9).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schuss {
    id: SchussId,
    runde_id: RundeId,
    zuordnung: Zuordnung,
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
            zuordnung: neu.zuordnung,
            ziel_id: neu.ziel_id,
            pfeil: neu.pfeil,
            wertungszone: neu.wertungszone,
        }
    }

    /// Rebuild a persisted Schuss. Fails only when the stored Pfeil is invalid (`0`).
    pub fn from_parts(
        id: SchussId,
        runde_id: RundeId,
        zuordnung: Zuordnung,
        ziel_id: ZielId,
        pfeil: usize,
        wertungszone: Wertungszone,
    ) -> Result<Self, SchussFehler> {
        let neu = NeuerSchuss::neu(runde_id, zuordnung, ziel_id, pfeil, wertungszone)?;
        Ok(Self::neu(id, neu))
    }

    pub fn id(&self) -> SchussId {
        self.id
    }

    pub fn runde_id(&self) -> RundeId {
        self.runde_id
    }

    pub fn zuordnung(&self) -> Zuordnung {
        self.zuordnung
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

    /// Set the owning Benutzer explicitly (used when a Runde is re-read from storage).
    pub fn zuordnen(&mut self, benutzer_id: BenutzerId) {
        self.zuordnung = Zuordnung::Benutzer(benutzer_id);
    }

    /// Replace the owner with [`Zuordnung::Anonymisiert`] (SPEC-datenschutz retention).
    pub fn anonymisiere(&mut self, am: DateTime<Utc>) {
        self.zuordnung = Zuordnung::Anonymisiert { am };
    }
}

/// The result of folding every Schuss at one Ziel.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Zielergebnis {
    erster_treffer: Punkte,
    mehrschuss: Punkte,
    erster_pfeil_getroffen: bool,
    anzahl_schüsse: usize,
}

impl Zielergebnis {
    /// Only the first Treffer counts (AC-3).
    pub fn erster_treffer(self) -> Punkte {
        self.erster_treffer
    }

    /// Every Schuss counts by its Pfeil position (AC-4).
    pub fn mehrschuss(self) -> Punkte {
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
/// the scoring rule. An empty collection has no result at all (AC-6), which is why the
/// result is an `Option` rather than a zeroed [`Zielergebnis`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schussmenge {
    ziel_id: ZielId,
    schüsse: Vec<Schuss>,
}

impl Schussmenge {
    /// Sort the given Schüsse by Pfeil, so the fold is order-independent.
    pub fn für_ziel(ziel_id: ZielId, schüsse: impl IntoIterator<Item = Schuss>) -> Self {
        let mut schüsse: Vec<_> = schüsse.into_iter().collect();
        schüsse.sort_by_key(|schuss| schuss.pfeil());
        Self { ziel_id, schüsse }
    }

    pub fn ziel_id(&self) -> ZielId {
        self.ziel_id
    }

    pub fn schüsse(&self) -> &[Schuss] {
        &self.schüsse
    }

    /// `None` for an empty Ziel: a Ziel with no Schuss has no result yet.
    pub fn ergebnis(&self) -> Option<Zielergebnis> {
        if self.schüsse.is_empty() {
            return None;
        }
        let punkte: Vec<_> = self
            .schüsse
            .iter()
            .map(|schuss| (schuss.pfeil(), schuss.wertungszone()))
            .collect();

        let erster_treffer = punkte
            .iter()
            .find(|(_, zone)| zone.ist_treffer())
            .map_or(Punkte::NULL, |&(pfeil, zone)| {
                Punkte::für_schuss(pfeil, zone)
            });
        let mehrschuss = punkte
            .iter()
            .map(|&(pfeil, zone)| Punkte::für_schuss(pfeil, zone))
            .sum();
        let erster_pfeil_getroffen = punkte
            .iter()
            .any(|&(pfeil, zone)| pfeil.get() == 1 && zone.ist_treffer());

        Some(Zielergebnis {
            erster_treffer,
            mehrschuss,
            erster_pfeil_getroffen,
            anzahl_schüsse: punkte.len(),
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
            Zuordnung::Anonymisiert { am: Utc::now() },
            ziel(),
            pfeil,
            zone,
        )
        .unwrap();
        Schuss::neu(SchussId(Uuid::from_u128(pfeil as u128)), neu)
    }

    fn menge(schüsse: &[(usize, Wertungszone)]) -> Schussmenge {
        Schussmenge::für_ziel(
            ziel(),
            schüsse.iter().map(|&(pfeil, zone)| schuss(pfeil, zone)),
        )
    }

    fn ergebnis(schüsse: &[(usize, Wertungszone)]) -> Zielergebnis {
        menge(schüsse).ergebnis().expect("nicht leer")
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
                    Punkte::für_schuss(pfeil(nummer), zone).value(),
                    punkte,
                    "Pfeil {nummer} {zone:?}"
                );
            }
        }
    }

    #[test_log::test]
    fn ac2_drei_fehlschüsse_null() {
        let ergebnis = ergebnis(&[(1, Miss), (2, Miss), (3, Miss)]);
        assert_eq!(ergebnis.erster_treffer(), Punkte::NULL);
        assert_eq!(ergebnis.mehrschuss(), Punkte::NULL);
    }

    #[test_log::test]
    fn ac3_erster_treffer() {
        assert_eq!(ergebnis(&[(1, Kill)]).erster_treffer().value(), 18);
        assert_eq!(
            ergebnis(&[(1, Miss), (2, Spot)]).erster_treffer().value(),
            14
        );
        assert_eq!(
            ergebnis(&[(1, Miss), (2, Miss), (3, Body)])
                .erster_treffer()
                .value(),
            4
        );
        assert_eq!(
            ergebnis(&[(1, Body), (2, Kill)]).erster_treffer().value(),
            16
        );
    }

    #[test_log::test]
    fn ac4_mehrschuss() {
        assert_eq!(
            ergebnis(&[(1, Kill), (2, Kill), (3, Spot)])
                .mehrschuss()
                .value(),
            38
        );
        assert_eq!(ergebnis(&[(1, Body), (2, Kill)]).mehrschuss().value(), 28);
        assert_eq!(ergebnis(&[(1, Miss), (2, Spot)]).mehrschuss().value(), 14);
    }

    #[test_log::test]
    fn ac5_ab_viertem_pfeil_null() {
        let ergebnis = ergebnis(&[(1, Miss), (2, Miss), (3, Miss), (4, Kill)]);
        assert_eq!(ergebnis.erster_treffer(), Punkte::NULL);
        assert_eq!(ergebnis.mehrschuss(), Punkte::NULL);
        // A Schuss at Pfeil 4 still counts in the hit statistics.
        assert_eq!(ergebnis.anzahl_schüsse(), 4);
    }

    #[test_log::test]
    fn ac6_leer_ohne_ergebnis_sonst_beliebig_lang() {
        assert!(Schussmenge::für_ziel(ziel(), []).ergebnis().is_none());
        let lang: Vec<_> = (1..=12).map(|nummer| (nummer, Miss)).collect();
        assert_eq!(ergebnis(&lang).anzahl_schüsse(), 12);
    }

    #[test_log::test]
    fn ac7_erster_pfeil_getroffen() {
        assert!(ergebnis(&[(1, Body)]).erster_pfeil_getroffen());
        assert!(ergebnis(&[(1, Spot), (2, Miss)]).erster_pfeil_getroffen());
        assert!(!ergebnis(&[(1, Miss), (2, Spot)]).erster_pfeil_getroffen());
    }

    #[test_log::test]
    fn die_faltung_ist_unabhängig_von_der_eingabereihenfolge() {
        let sortiert = ergebnis(&[(1, Kill), (2, Kill), (3, Spot)]);
        let unsortiert = ergebnis(&[(3, Spot), (1, Kill), (2, Kill)]);
        assert_eq!(sortiert, unsortiert);
        assert_eq!(sortiert.erster_treffer().value(), 18);
        assert_eq!(sortiert.mehrschuss().value(), 38);
    }

    #[test_log::test]
    fn pfeil_null_wird_abgelehnt() {
        let ergebnis = NeuerSchuss::neu(
            RundeId(Uuid::nil()),
            Zuordnung::Anonymisiert { am: Utc::now() },
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
            Zuordnung::Benutzer(BenutzerId(Uuid::nil())),
            ziel(),
            2,
            Body,
        )
        .unwrap();
        let original = Schuss::neu(SchussId(Uuid::nil()), neu);
        let geladen = Schuss::from_parts(
            original.id(),
            original.runde_id(),
            original.zuordnung(),
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
                Zuordnung::Benutzer(BenutzerId(Uuid::nil())),
                ziel(),
                1,
                Kill,
            )
            .unwrap(),
        );
        let am = Utc::now();
        schuss.anonymisiere(am);
        assert_eq!(schuss.zuordnung(), Zuordnung::Anonymisiert { am });
        assert_eq!(schuss.zuordnung().benutzer(), None);
    }
}
