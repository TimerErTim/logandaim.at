use std::iter::Sum;
use std::num::NonZeroUsize;
use std::ops::Add;

// TODO(wertung): rework pending. This file is inherited from the pre-slice code base and
// is neither verified against the product spec nor final: the classic 3-arrow table, the
// `Zielwertung` shape, and the Erster-Treffer / Mehrschuss split are a provisional
// reference for how a slice is built, exactly like the rest of the reference slice.
// Treat it as brittle, not as an approved domain model.

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
}

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

impl Add for Punkte {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sum for Punkte {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::NULL, Add::add)
    }
}

/// Schüsse at one Ziel, ordered by Pfeil. Non-empty by construction: a Ziel without
/// Schüsse has no result yet.
///
/// Each entry keeps the Pfeil it was shot as, so the 1-based contract is carried by the
/// data instead of being re-derived from a vector index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zielwertung {
    schüsse: Vec<(NonZeroUsize, Wertungszone)>,
}

impl Zielwertung {
    /// Order the given Schüsse by Pfeil. Returns `None` for an empty Ziel.
    pub fn aus_schüssen(
        schüsse: impl IntoIterator<Item = (NonZeroUsize, Wertungszone)>,
    ) -> Option<Self> {
        let mut schüsse: Vec<_> = schüsse.into_iter().collect();
        if schüsse.is_empty() {
            return None;
        }
        schüsse.sort_by_key(|(pfeil, _)| *pfeil);
        Some(Self { schüsse })
    }

    pub fn schüsse(&self) -> &[(NonZeroUsize, Wertungszone)] {
        &self.schüsse
    }

    fn punkte_je_schuss(&self) -> impl Iterator<Item = (Wertungszone, Punkte)> + '_ {
        self.schüsse
            .iter()
            .map(|&(pfeil, zone)| (zone, Punkte::für_schuss(pfeil, zone)))
    }

    /// Only the first Treffer counts.
    pub fn erster_treffer(&self) -> Punkte {
        self.punkte_je_schuss()
            .find(|(zone, _)| zone.ist_treffer())
            .map_or(Punkte::NULL, |(_, punkte)| punkte)
    }

    /// Every Schuss counts by its Pfeil position.
    pub fn mehrschuss(&self) -> Punkte {
        self.punkte_je_schuss().map(|(_, punkte)| punkte).sum()
    }

    /// Whether Pfeil 1 hit the target.
    pub fn erster_pfeil_getroffen(&self) -> bool {
        self.schüsse
            .iter()
            .any(|&(pfeil, zone)| pfeil.get() == 1 && zone.ist_treffer())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rundenergebnis {
    pub erster_treffer: Punkte,
    pub mehrschuss: Punkte,
}

impl Rundenergebnis {
    /// Sum per Wertung over all Ziel results. Ziele without result are skipped (AC-8).
    pub fn aus_zielen<'a>(ziele: impl IntoIterator<Item = Option<&'a Zielwertung>>) -> Self {
        ziele
            .into_iter()
            .flatten()
            .fold(Self::default(), |ergebnis, ziel| Self {
                erster_treffer: ergebnis.erster_treffer + ziel.erster_treffer(),
                mehrschuss: ergebnis.mehrschuss + ziel.mehrschuss(),
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

    fn ziel(schüsse: &[(usize, Wertungszone)]) -> Zielwertung {
        Zielwertung::aus_schüssen(
            schüsse
                .iter()
                .map(|&(pfeil, zone)| (self::pfeil(pfeil), zone)),
        )
        .expect("test fixture passes at least one Schuss")
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
        let z = ziel(&[(1, Miss), (2, Miss), (3, Miss)]);
        assert_eq!(z.erster_treffer(), Punkte::NULL);
        assert_eq!(z.mehrschuss(), Punkte::NULL);
    }

    #[test_log::test]
    fn ac3_erster_treffer() {
        assert_eq!(ziel(&[(1, Kill)]).erster_treffer().value(), 18);
        assert_eq!(ziel(&[(1, Miss), (2, Spot)]).erster_treffer().value(), 14);
        assert_eq!(
            ziel(&[(1, Miss), (2, Miss), (3, Body)])
                .erster_treffer()
                .value(),
            4
        );
        assert_eq!(ziel(&[(1, Body), (2, Kill)]).erster_treffer().value(), 16);
    }

    #[test_log::test]
    fn ac4_mehrschuss() {
        assert_eq!(
            ziel(&[(1, Kill), (2, Kill), (3, Spot)])
                .mehrschuss()
                .value(),
            38
        );
        assert_eq!(ziel(&[(1, Body), (2, Kill)]).mehrschuss().value(), 28);
        assert_eq!(ziel(&[(1, Miss), (2, Spot)]).mehrschuss().value(), 14);
    }

    #[test_log::test]
    fn ac5_ab_viertem_pfeil_null() {
        let z = ziel(&[(1, Miss), (2, Miss), (3, Miss), (4, Kill)]);
        assert_eq!(z.erster_treffer(), Punkte::NULL);
        assert_eq!(z.mehrschuss(), Punkte::NULL);
    }

    #[test_log::test]
    fn ac6_leer_ohne_ergebnis_sonst_beliebig_lang() {
        assert!(Zielwertung::aus_schüssen([]).is_none());
        let lang: Vec<_> = (1..=12).map(|nummer| (pfeil(nummer), Miss)).collect();
        assert_eq!(Zielwertung::aus_schüssen(lang).unwrap().schüsse().len(), 12);
    }

    #[test_log::test]
    fn ac7_erster_pfeil_getroffen() {
        assert!(ziel(&[(1, Body)]).erster_pfeil_getroffen());
        assert!(ziel(&[(1, Spot), (2, Miss)]).erster_pfeil_getroffen());
        assert!(!ziel(&[(1, Miss), (2, Spot)]).erster_pfeil_getroffen());
    }

    #[test_log::test]
    fn ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis() {
        let ziele = [
            Zielwertung::aus_schüssen(vec![(pfeil(1), Kill), (pfeil(2), Kill), (pfeil(3), Spot)]),
            Zielwertung::aus_schüssen([]),
            Zielwertung::aus_schüssen(vec![(pfeil(1), Miss), (pfeil(2), Spot)]),
        ];
        let ergebnis = Rundenergebnis::aus_zielen(ziele.iter().map(Option::as_ref));
        assert_eq!(ergebnis.erster_treffer.value(), 18 + 14);
        assert_eq!(ergebnis.mehrschuss.value(), 38 + 14);
    }

    #[test_log::test]
    fn aus_schüssen_sortiert_nach_pfeil() {
        let unsortiert =
            Zielwertung::aus_schüssen(vec![(pfeil(3), Body), (pfeil(1), Kill), (pfeil(2), Spot)])
                .unwrap();
        let pfeile: Vec<_> = unsortiert
            .schüsse()
            .iter()
            .map(|(pfeil, _)| pfeil.get())
            .collect();
        assert_eq!(pfeile, [1, 2, 3]);
        // Erster Treffer = Pfeil 1 Kill (18), Mehrschuss = 18 + 14 + 4.
        assert_eq!(unsortiert.erster_treffer().value(), 18);
        assert_eq!(unsortiert.mehrschuss().value(), 36);
    }
}
