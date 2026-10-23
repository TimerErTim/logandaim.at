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

fn nonzero(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("Pfeil-Positionen sind 1-basiert")
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

/// Schüsse at one Ziel in shooting order. Non-empty by construction: a Ziel without
/// Schüsse has no result yet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zielwertung {
    schüsse: Vec<Wertungszone>,
}

impl Zielwertung {
    pub fn aus_schüssen(schüsse: Vec<Wertungszone>) -> Option<Self> {
        (!schüsse.is_empty()).then_some(Self { schüsse })
    }

    pub fn schüsse(&self) -> &[Wertungszone] {
        &self.schüsse
    }

    fn punkte_je_schuss(&self) -> impl Iterator<Item = (Wertungszone, Punkte)> + '_ {
        self.schüsse
            .iter()
            .enumerate()
            .map(|(index, &zone)| (zone, Punkte::für_schuss(nonzero(index + 1), zone)))
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

    pub fn erster_pfeil_getroffen(&self) -> bool {
        self.schüsse[0].ist_treffer()
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

    fn ziel(schüsse: &[Wertungszone]) -> Zielwertung {
        Zielwertung::aus_schüssen(schüsse.to_vec()).unwrap()
    }

    #[test_log::test]
    fn ac1_punktetabelle() {
        let tabelle = [
            (1, [20, 18, 16, 0]),
            (2, [14, 12, 10, 0]),
            (3, [8, 6, 4, 0]),
        ];
        for (pfeil, erwartet) in tabelle {
            let pfeil = nonzero(pfeil);
            for (zone, punkte) in [Spot, Kill, Body, Miss].into_iter().zip(erwartet) {
                assert_eq!(
                    Punkte::für_schuss(pfeil, zone).value(),
                    punkte,
                    "Pfeil {pfeil} {zone:?}"
                );
            }
        }
    }

    #[test_log::test]
    fn ac2_drei_fehlschüsse_null() {
        let z = ziel(&[Miss, Miss, Miss]);
        assert_eq!(z.erster_treffer(), Punkte::NULL);
        assert_eq!(z.mehrschuss(), Punkte::NULL);
    }

    #[test_log::test]
    fn ac3_erster_treffer() {
        assert_eq!(ziel(&[Kill]).erster_treffer().value(), 18);
        assert_eq!(ziel(&[Miss, Spot]).erster_treffer().value(), 14);
        assert_eq!(ziel(&[Miss, Miss, Body]).erster_treffer().value(), 4);
        assert_eq!(ziel(&[Body, Kill]).erster_treffer().value(), 16);
    }

    #[test_log::test]
    fn ac4_mehrschuss() {
        assert_eq!(ziel(&[Kill, Kill, Spot]).mehrschuss().value(), 38);
        assert_eq!(ziel(&[Body, Kill]).mehrschuss().value(), 28);
        assert_eq!(ziel(&[Miss, Spot]).mehrschuss().value(), 14);
    }

    #[test_log::test]
    fn ac5_ab_viertem_pfeil_null() {
        let z = ziel(&[Miss, Miss, Miss, Kill]);
        assert_eq!(z.erster_treffer(), Punkte::NULL);
        assert_eq!(z.mehrschuss(), Punkte::NULL);
    }

    #[test_log::test]
    fn ac6_leer_ohne_ergebnis_sonst_beliebig_lang() {
        assert!(Zielwertung::aus_schüssen(vec![]).is_none());
        let lang = vec![Miss; 12];
        assert_eq!(Zielwertung::aus_schüssen(lang).unwrap().schüsse().len(), 12);
    }

    #[test_log::test]
    fn ac7_erster_pfeil_getroffen() {
        assert!(ziel(&[Body]).erster_pfeil_getroffen());
        assert!(ziel(&[Spot, Miss]).erster_pfeil_getroffen());
        assert!(!ziel(&[Miss, Spot]).erster_pfeil_getroffen());
    }

    #[test_log::test]
    fn ac8_rundenergebnis_summiert_und_überspringt_ziele_ohne_ergebnis() {
        let ziele = [
            Zielwertung::aus_schüssen(vec![Kill, Kill, Spot]),
            Zielwertung::aus_schüssen(vec![]),
            Zielwertung::aus_schüssen(vec![Miss, Spot]),
        ];
        let ergebnis = Rundenergebnis::aus_zielen(ziele.iter().map(Option::as_ref));
        assert_eq!(ergebnis.erster_treffer.value(), 18 + 14);
        assert_eq!(ergebnis.mehrschuss.value(), 38 + 14);
    }
}
