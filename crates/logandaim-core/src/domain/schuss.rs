use uuid::Uuid;

use super::Wertungszone;

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

/// One arrow shot at one Ziel.
///
/// Fields are private; the only ways in are [`Schuss::neu`] and [`Schuss::from_parts`],
/// which validate. `pfeil` is 1-based and unbounded: Pfeil 4 and later are allowed and
/// simply score 0 in every Wertung (see [`super::Zielwertung`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schuss {
    id: SchussId,
    ziel_id: ZielId,
    pfeil: usize,
    wertungszone: Wertungszone,
}

impl Schuss {
    /// Fresh Schuss with a caller-provided identity (the repository assigns the final ID).
    pub fn neu(
        id: SchussId,
        ziel_id: ZielId,
        pfeil: usize,
        wertungszone: Wertungszone,
    ) -> Result<Self, SchussFehler> {
        Self::from_parts(id, ziel_id, pfeil, wertungszone)
    }

    /// Rebuild a persisted Schuss. Fails only when the stored Pfeil is invalid (`0`).
    pub fn from_parts(
        id: SchussId,
        ziel_id: ZielId,
        pfeil: usize,
        wertungszone: Wertungszone,
    ) -> Result<Self, SchussFehler> {
        if pfeil == 0 {
            return Err(SchussFehler::PfeilNull);
        }
        Ok(Self {
            id,
            ziel_id,
            pfeil,
            wertungszone,
        })
    }

    pub fn id(&self) -> SchussId {
        self.id
    }

    pub fn ziel_id(&self) -> ZielId {
        self.ziel_id
    }

    /// 1-based position of the arrow at its Ziel.
    pub fn pfeil(&self) -> usize {
        self.pfeil
    }

    pub fn wertungszone(&self) -> Wertungszone {
        self.wertungszone
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ziel() -> ZielId {
        ZielId(Uuid::nil())
    }

    #[test_log::test]
    fn pfeil_ist_eins_basiert() {
        let schuss = Schuss::neu(SchussId(Uuid::nil()), ziel(), 1, Wertungszone::Kill).unwrap();
        assert_eq!(schuss.pfeil(), 1);
        assert_eq!(schuss.wertungszone(), Wertungszone::Kill);
        assert_eq!(schuss.ziel_id(), ziel());
    }

    #[test_log::test]
    fn pfeil_null_wird_abgelehnt() {
        let ergebnis = Schuss::neu(SchussId(Uuid::nil()), ziel(), 0, Wertungszone::Spot);
        assert!(matches!(ergebnis, Err(SchussFehler::PfeilNull)));
    }

    #[test_log::test]
    fn pfeil_vier_und_später_sind_erlaubt() {
        for pfeil in [4, 5, 12] {
            let schuss =
                Schuss::neu(SchussId(Uuid::nil()), ziel(), pfeil, Wertungszone::Miss).unwrap();
            assert_eq!(schuss.pfeil(), pfeil);
        }
    }
}
