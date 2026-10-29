//! A physical 3D target belonging to exactly one `Anlage`.
//!
//! A `Ziel` is an entity inside the `Anlage` aggregate, not a root of its own: it has no
//! lifecycle outside its Anlage and no repository loads or saves it alone. Callers
//! outside the aggregate hold [`ZielId`] and nothing else (SPEC-stammdaten rule 9).

use uuid::Uuid;

/// Identity of one physical Ziel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZielId(pub Uuid);

/// Lifecycle of an entity inside the aggregate.
///
/// A plain enum rather than a phantom type: a child is never loaded or saved on its own,
/// so a phantom parameter would buy compile-time safety where no caller can act anyway.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Zustand {
    Aktiv,
    Archiviert,
}

impl Zustand {
    pub fn ist_aktiv(self) -> bool {
        matches!(self, Self::Aktiv)
    }

    /// Single stored spelling, shared by every adapter.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Aktiv => "Aktiv",
            Self::Archiviert => "Archiviert",
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("der Zustand ist unbekannt")]
pub struct ZustandUnbekannt;

impl std::str::FromStr for Zustand {
    type Err = ZustandUnbekannt;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Aktiv" => Ok(Self::Aktiv),
            "Archiviert" => Ok(Self::Archiviert),
            _ => Err(ZustandUnbekannt),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ZielFehler {
    #[error("der Tiername darf nicht leer sein")]
    TiernameLeer,
    #[error("die Zielgruppe muss zwischen 1 und 4 liegen, war aber {0}")]
    ZielgruppeUngültig(u8),
    #[error("das Ziel ist archiviert und kann nicht mehr geändert werden")]
    Archiviert,
}

/// Non-empty animal name of a physical Ziel.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tiername(String);

impl Tiername {
    pub fn neu<S: AsRef<str>>(wert: S) -> Result<Self, ZielFehler> {
        let wert = wert.as_ref().trim();
        if wert.is_empty() {
            return Err(ZielFehler::TiernameLeer);
        }
        Ok(Self(wert.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Tiername {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Target group of a 3D target, exactly 1, 2, 3, or 4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Zielgruppe(u8);

impl Zielgruppe {
    pub fn neu(wert: u8) -> Result<Self, ZielFehler> {
        match wert {
            1..=4 => Ok(Self(wert)),
            _ => Err(ZielFehler::ZielgruppeUngültig(wert)),
        }
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

/// Creation input of a Ziel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuesZiel {
    tiername: Tiername,
    zielgruppe: Zielgruppe,
}

impl NeuesZiel {
    pub fn neu(tiername: Tiername, zielgruppe: Zielgruppe) -> Self {
        Self {
            tiername,
            zielgruppe,
        }
    }

    pub fn tiername(&self) -> &Tiername {
        &self.tiername
    }

    pub fn zielgruppe(&self) -> Zielgruppe {
        self.zielgruppe
    }
}

/// One physical target at one Anlage.
///
/// Its identity survives route edits and Parcours changes; only replacing the physical
/// target creates a new identity (SPEC-stammdaten rule 5, AC-5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ziel {
    id: ZielId,
    tiername: Tiername,
    zielgruppe: Zielgruppe,
    zustand: Zustand,
}

impl Ziel {
    pub fn neu(id: ZielId, neu: NeuesZiel) -> Self {
        Self {
            id,
            tiername: neu.tiername,
            zielgruppe: neu.zielgruppe,
            zustand: Zustand::Aktiv,
        }
    }

    /// Rebuild a persisted Ziel, including its lifecycle state.
    pub fn from_parts(
        id: ZielId,
        tiername: Tiername,
        zielgruppe: Zielgruppe,
        zustand: Zustand,
    ) -> Self {
        Self {
            id,
            tiername,
            zielgruppe,
            zustand,
        }
    }

    pub fn id(&self) -> ZielId {
        self.id
    }

    pub fn tiername(&self) -> &Tiername {
        &self.tiername
    }

    pub fn zielgruppe(&self) -> Zielgruppe {
        self.zielgruppe
    }

    pub fn zustand(&self) -> Zustand {
        self.zustand
    }

    pub fn ist_archiviert(&self) -> bool {
        matches!(self.zustand, Zustand::Archiviert)
    }

    fn nur_aktiv(&self) -> Result<(), ZielFehler> {
        match self.zustand {
            Zustand::Aktiv => Ok(()),
            Zustand::Archiviert => Err(ZielFehler::Archiviert),
        }
    }

    /// Edit the descriptive fields. `Err(ZielFehler::Archiviert)` when archived (AC-11).
    pub fn ändern(
        &mut self,
        tiername: Tiername,
        zielgruppe: Zielgruppe,
    ) -> Result<(), ZielFehler> {
        self.nur_aktiv()?;
        self.tiername = tiername;
        self.zielgruppe = zielgruppe;
        Ok(())
    }

    /// Archive the Ziel: it leaves every current arrangement and keeps its history.
    /// Archiving twice is the same refusal as any other mutation of an archived child.
    pub fn archiviere(&mut self) -> Result<(), ZielFehler> {
        self.nur_aktiv()?;
        self.zustand = Zustand::Archiviert;
        Ok(())
    }

    /// A new identity for the same physical slot; the old one stays archived (AC-5).
    pub fn duplicate(&self) -> Result<NeuesZiel, ZielFehler> {
        self.nur_aktiv()?;
        Ok(NeuesZiel::neu(self.tiername.clone(), self.zielgruppe))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ziel() -> Ziel {
        Ziel::neu(
            ZielId(Uuid::nil()),
            NeuesZiel::neu(
                Tiername::neu("Keiler").unwrap(),
                Zielgruppe::neu(2).unwrap(),
            ),
        )
    }

    #[test_log::test]
    fn neues_ziel_ist_aktiv() {
        assert_eq!(ziel().zustand(), Zustand::Aktiv);
    }

    #[test_log::test]
    fn leere_tiernamen_werden_abgelehnt() {
        assert!(matches!(Tiername::neu("  "), Err(ZielFehler::TiernameLeer)));
        assert!(matches!(Tiername::neu(""), Err(ZielFehler::TiernameLeer)));
    }

    #[test_log::test]
    fn zielgruppe_ist_eins_bis_vier() {
        for wert in 1..=4 {
            assert_eq!(Zielgruppe::neu(wert).unwrap().value(), wert);
        }
        assert!(matches!(
            Zielgruppe::neu(0),
            Err(ZielFehler::ZielgruppeUngültig(0))
        ));
        assert!(matches!(
            Zielgruppe::neu(5),
            Err(ZielFehler::ZielgruppeUngültig(5))
        ));
    }

    #[test_log::test]
    fn archiviertes_ziel_verweigert_jede_aenderung() {
        let mut ziel = ziel();
        ziel.archiviere().unwrap();
        assert_eq!(ziel.zustand(), Zustand::Archiviert);

        assert!(matches!(ziel.archiviere(), Err(ZielFehler::Archiviert)));
        assert!(matches!(
            ziel.ändern(Tiername::neu("Reh").unwrap(), Zielgruppe::neu(1).unwrap()),
            Err(ZielFehler::Archiviert)
        ));
        assert!(matches!(ziel.duplicate(), Err(ZielFehler::Archiviert)));
    }

    #[test_log::test]
    fn aendern_haelt_die_identitaet() {
        let mut ziel = ziel();
        let id = ziel.id();
        ziel.ändern(Tiername::neu("Reh").unwrap(), Zielgruppe::neu(4).unwrap())
            .unwrap();
        assert_eq!(ziel.id(), id);
        assert_eq!(ziel.tiername().as_str(), "Reh");
        assert_eq!(ziel.zielgruppe().value(), 4);
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let original = ziel();
        let geladen = Ziel::from_parts(
            original.id(),
            original.tiername().clone(),
            original.zielgruppe(),
            original.zustand(),
        );
        assert_eq!(original, geladen);
    }
}
