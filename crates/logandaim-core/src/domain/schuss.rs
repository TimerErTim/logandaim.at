use std::num::NonZeroUsize;

use rustc_hash::FxHashMap;
use uuid::Uuid;

use super::{Punkte, Rundenergebnis, Wertungszone, Zielwertung};

/// Identity of one Ziel (3D target) within a Parcours.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZielId(pub Uuid);

/// Identity of one logged Schuss.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SchussId(pub Uuid);

#[derive(Debug, thiserror::Error)]
pub enum SchussFehler {
    #[error("der Pfeil ist 1-basiert, war aber 0")]
    PfeilNull,
}

/// Creation input: everything of a Schuss except its identity.
///
/// Holds no aggregate invariants; `pfeil` is already a [`NonZeroUsize`], so invalid
/// state cannot be represented further down the stack.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuerSchuss {
    ziel_id: ZielId,
    pfeil: NonZeroUsize,
    wertungszone: Wertungszone,
}

impl NeuerSchuss {
    /// `pfeil` is 1-based. `0` is rejected as [`SchussFehler::PfeilNull`].
    pub fn neu(
        ziel_id: ZielId,
        pfeil: usize,
        wertungszone: Wertungszone,
    ) -> Result<Self, SchussFehler> {
        let pfeil = NonZeroUsize::new(pfeil).ok_or(SchussFehler::PfeilNull)?;
        Ok(Self {
            ziel_id,
            pfeil,
            wertungszone,
        })
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
/// Fields are private; the only ways in are [`Schuss::neu`] and [`Schuss::from_parts`].
/// `pfeil` is 1-based and unbounded: Pfeil 4 and later are allowed and simply score 0 in
/// every Wertung (see [`super::Zielwertung`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schuss {
    id: SchussId,
    ziel_id: ZielId,
    pfeil: NonZeroUsize,
    wertungszone: Wertungszone,
}

impl Schuss {
    /// Fresh Schuss with a caller-provided identity (the repository assigns the final ID).
    ///
    /// All invariants live in [`NeuerSchuss`], so this cannot fail.
    pub fn neu(id: SchussId, neu: NeuerSchuss) -> Self {
        Self {
            id,
            ziel_id: neu.ziel_id,
            pfeil: neu.pfeil,
            wertungszone: neu.wertungszone,
        }
    }

    /// Rebuild a persisted Schuss. Fails only when the stored Pfeil is invalid (`0`).
    pub fn from_parts(
        id: SchussId,
        ziel_id: ZielId,
        pfeil: usize,
        wertungszone: Wertungszone,
    ) -> Result<Self, SchussFehler> {
        let neu = NeuerSchuss::neu(ziel_id, pfeil, wertungszone)?;
        Ok(Self::neu(id, neu))
    }

    pub fn id(&self) -> SchussId {
        self.id
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
}

/// Summary over a selection of Schüsse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wertungsergebnis {
    pub erster_treffer: Punkte,
    pub mehrschuss: Punkte,
    pub erster_pfeil_getroffen: bool,
    pub anzahl_schüsse: usize,
}

/// Domain service: summarize a selection of Schüsse.
///
/// Groups the Schüsse per Ziel, orders each group by `pfeil`, summarizes every Ziel with
/// [`Zielwertung`], and folds the results with [`Rundenergebnis`].
///
/// TODO(wertung): the scoring rules and [`Zielwertung`] are a provisional reference
/// implementation, not a final domain model; see `domain/wertung.rs`.
pub fn wertung_aus_schüssen(schüsse: &[Schuss]) -> Wertungsergebnis {
    let mut je_ziel: FxHashMap<ZielId, Vec<(usize, Wertungszone)>> = FxHashMap::default();
    for schuss in schüsse {
        je_ziel
            .entry(schuss.ziel_id())
            .or_default()
            .push((schuss.pfeil().get(), schuss.wertungszone()));
    }

    let mut zielwertungen = Vec::with_capacity(je_ziel.len());
    for (_, mut schüsse) in je_ziel {
        schüsse.sort_by_key(|(pfeil, _)| *pfeil);
        let zonen: Vec<Wertungszone> = schüsse.into_iter().map(|(_, zone)| zone).collect();
        zielwertungen.push(Zielwertung::aus_schüssen(zonen));
    }

    let gesamt = Rundenergebnis::aus_zielen(zielwertungen.iter().map(Option::as_ref));

    Wertungsergebnis {
        erster_treffer: gesamt.erster_treffer,
        mehrschuss: gesamt.mehrschuss,
        erster_pfeil_getroffen: zielwertungen
            .iter()
            .flatten()
            .any(Zielwertung::erster_pfeil_getroffen),
        anzahl_schüsse: schüsse.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ziel() -> ZielId {
        ZielId(Uuid::nil())
    }

    fn id() -> SchussId {
        SchussId(Uuid::nil())
    }

    fn schuss(ziel: u8, pfeil: usize, zone: Wertungszone) -> Schuss {
        Schuss::neu(
            SchussId(Uuid::from_u128(pfeil as u128)),
            NeuerSchuss::neu(ZielId(Uuid::from_u128(ziel as u128)), pfeil, zone).unwrap(),
        )
    }

    #[test_log::test]
    fn pfeil_ist_eins_basiert() {
        let neu = NeuerSchuss::neu(ziel(), 1, Wertungszone::Kill).unwrap();
        let schuss = Schuss::neu(id(), neu);
        assert_eq!(schuss.pfeil().get(), 1);
        assert_eq!(schuss.wertungszone(), Wertungszone::Kill);
        assert_eq!(schuss.ziel_id(), ziel());
    }

    #[test_log::test]
    fn pfeil_null_wird_abgelehnt() {
        let ergebnis = NeuerSchuss::neu(ziel(), 0, Wertungszone::Spot);
        assert!(matches!(ergebnis, Err(SchussFehler::PfeilNull)));

        let ergebnis = Schuss::from_parts(id(), ziel(), 0, Wertungszone::Spot);
        assert!(matches!(ergebnis, Err(SchussFehler::PfeilNull)));
    }

    #[test_log::test]
    fn pfeil_vier_und_später_sind_erlaubt() {
        for pfeil in [4, 5, 12] {
            let neu = NeuerSchuss::neu(ziel(), pfeil, Wertungszone::Miss).unwrap();
            assert_eq!(Schuss::neu(id(), neu).pfeil().get(), pfeil);
        }
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let neu = NeuerSchuss::neu(ziel(), 2, Wertungszone::Body).unwrap();
        let original = Schuss::neu(id(), neu);
        let geladen =
            Schuss::from_parts(original.id(), original.ziel_id(), 2, Wertungszone::Body).unwrap();
        assert_eq!(original, geladen);
    }

    #[test_log::test]
    fn wertung_summiert_je_ziel_und_ordnung_nach_pfeil() {
        let schüsse = vec![
            schuss(1, 1, Wertungszone::Kill),
            schuss(1, 2, Wertungszone::Kill),
            schuss(1, 3, Wertungszone::Spot),
        ];
        let ergebnis = wertung_aus_schüssen(&schüsse);
        assert_eq!(ergebnis.erster_treffer.value(), 18);
        assert_eq!(ergebnis.mehrschuss.value(), 38);
        assert!(ergebnis.erster_pfeil_getroffen);
        assert_eq!(ergebnis.anzahl_schüsse, 3);
    }

    #[test_log::test]
    fn wertung_ordnet_auch_bei_unsortierter_eingabe() {
        let schüsse = vec![
            schuss(1, 3, Wertungszone::Body),
            schuss(1, 1, Wertungszone::Miss),
            schuss(1, 2, Wertungszone::Spot),
        ];
        let ergebnis = wertung_aus_schüssen(&schüsse);
        // Ziel 1: Pfeil 1 Miss, Pfeil 2 Spot (14), Pfeil 3 Body (4).
        assert_eq!(ergebnis.erster_treffer.value(), 14);
        assert_eq!(ergebnis.mehrschuss.value(), 18);
        assert!(!ergebnis.erster_pfeil_getroffen);
    }

    #[test_log::test]
    fn wertung_hält_ziele_auseinander() {
        // Ziel 1: Pfeil 1 Spot (20), Pfeil 2 Kill (12).
        // Ziel 2: Pfeil 1 Miss (0), Pfeil 2 Body (10).
        let schüsse = vec![
            schuss(1, 1, Wertungszone::Spot),
            schuss(2, 1, Wertungszone::Miss),
            schuss(1, 2, Wertungszone::Kill),
            schuss(2, 2, Wertungszone::Body),
        ];
        let ergebnis = wertung_aus_schüssen(&schüsse);
        // Erster Treffer: 20 (Ziel 1) + 10 (Ziel 2, Pfeil 2).
        assert_eq!(ergebnis.erster_treffer.value(), 30);
        // Mehrschuss: 20 + 12 (Ziel 1) + 0 + 10 (Ziel 2).
        assert_eq!(ergebnis.mehrschuss.value(), 42);
        assert!(ergebnis.erster_pfeil_getroffen);
        assert_eq!(ergebnis.anzahl_schüsse, 4);
    }

    #[test_log::test]
    fn wertung_ab_pfeil_vier_zaehlt_null() {
        let schüsse = vec![
            schuss(1, 1, Wertungszone::Miss),
            schuss(1, 2, Wertungszone::Miss),
            schuss(1, 3, Wertungszone::Miss),
            schuss(1, 4, Wertungszone::Kill),
        ];
        let ergebnis = wertung_aus_schüssen(&schüsse);
        assert_eq!(ergebnis.erster_treffer.value(), 0);
        assert_eq!(ergebnis.mehrschuss.value(), 0);
    }
}
