//! A physical 3D target belonging to exactly one `Anlage`.
//!
//! A `Ziel` is an entity inside the `Anlage` aggregate, not a root of its own: it has no
//! lifecycle outside its Anlage and no repository loads or saves it alone. Callers
//! outside the aggregate hold [`AnlageZielId`] and nothing else (SPEC-stammdaten rule 9).

use std::num::NonZeroU32;

use uuid::Uuid;

use crate::domain::common::NonEmptyString;

/// Identity of one physical Ziel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AnlageZielId(pub Uuid);

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
    #[error("das Ziel ist archiviert und kann nicht mehr geändert werden")]
    Archiviert,
}

/// Creation input of a Ziel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuesZiel {
    tiername: NonEmptyString,
    zielnummer: NonZeroU32,
}

impl NeuesZiel {
    /// `zielnummer` is the Anlage's 1-based ordinal: two Ziele named "Reh" stay distinct
    /// because "Reh 1" is not "Reh 2".
    pub fn neu(tiername: NonEmptyString, zielnummer: NonZeroU32) -> Self {
        Self {
            tiername,
            zielnummer,
        }
    }

    pub fn tiername(&self) -> &NonEmptyString {
        &self.tiername
    }

    pub fn zielnummer(&self) -> NonZeroU32 {
        self.zielnummer
    }
}

/// One physical target at one Anlage.
///
/// Its identity survives route edits and Parkour changes; only replacing the physical
/// target creates a new identity (SPEC-stammdaten rule 5, AC-5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnlageZiel {
    id: AnlageZielId,
    tiername: NonEmptyString,
    zielnummer: NonZeroU32,
    zustand: Zustand,
}

impl AnlageZiel {
    pub fn neu(id: AnlageZielId, neu: NeuesZiel) -> Self {
        Self {
            id,
            tiername: neu.tiername,
            zielnummer: neu.zielnummer,
            zustand: Zustand::Aktiv,
        }
    }

    /// Rebuild a persisted Ziel, including its lifecycle state.
    pub fn from_parts(
        id: AnlageZielId,
        tiername: NonEmptyString,
        zielnummer: NonZeroU32,
        zustand: Zustand,
    ) -> Self {
        Self {
            id,
            tiername,
            zielnummer,
            zustand,
        }
    }

    pub fn id(&self) -> AnlageZielId {
        self.id
    }

    pub fn tiername(&self) -> &NonEmptyString {
        &self.tiername
    }

    /// The ordinal that tells two Ziele of the same Tiername apart.
    pub fn zielnummer(&self) -> NonZeroU32 {
        self.zielnummer
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

    /// Edit the kind of target behind a Ziel: its Tiername and its Zielnummer. Archived
    /// Ziele refuse the edit (AC-11); the identity is untouched.
    ///
    /// Both fields are part of how a Ziel is called out in a Runde ("Reh 2"), so staff must
    /// be able to correct either without replacing the physical target (AC-5).
    pub fn stammdaten_ändern(
        &mut self,
        tiername: NonEmptyString,
        zielnummer: NonZeroU32,
    ) -> Result<(), ZielFehler> {
        self.nur_aktiv()?;
        self.tiername = tiername;
        self.zielnummer = zielnummer;
        Ok(())
    }

    /// Change only the animal name, keeping the ordinal.
    pub fn tiername_ändern(&mut self, tiername: NonEmptyString) -> Result<(), ZielFehler> {
        self.nur_aktiv()?;
        self.tiername = tiername;
        Ok(())
    }

    /// Change only the ordinal, keeping the animal name.
    pub fn zielnummer_ändern(&mut self, zielnummer: NonZeroU32) -> Result<(), ZielFehler> {
        self.nur_aktiv()?;
        self.zielnummer = zielnummer;
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
    ///
    /// Duplication is a master-data read, not a mutation: an archived Ziel can be duplicated
    /// just like an active one, so the copy is available while the original is retired.
    pub fn duplicate(&self) -> NeuesZiel {
        NeuesZiel::neu(self.tiername.clone(), self.zielnummer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn name(wert: &str) -> NonEmptyString {
        NonEmptyString::neu(wert).unwrap()
    }

    fn nummer(wert: u32) -> NonZeroU32 {
        NonZeroU32::new(wert).unwrap()
    }

    fn ziel() -> AnlageZiel {
        AnlageZiel::neu(
            AnlageZielId(Uuid::nil()),
            NeuesZiel::neu(name("Keiler"), nummer(2)),
        )
    }

    #[test_log::test]
    fn neues_ziel_ist_aktiv() {
        assert_eq!(ziel().zustand(), Zustand::Aktiv);
    }

    #[test_log::test]
    fn die_zielnummer_unterscheidet_gleichnamige_ziele() {
        let erstes = AnlageZiel::neu(
            AnlageZielId(Uuid::from_u128(1)),
            NeuesZiel::neu(name("Reh"), nummer(1)),
        );
        let zweites = AnlageZiel::neu(
            AnlageZielId(Uuid::from_u128(2)),
            NeuesZiel::neu(name("Reh"), nummer(2)),
        );
        assert_eq!(erstes.tiername(), zweites.tiername());
        assert_ne!(erstes.zielnummer(), zweites.zielnummer());
        assert_ne!(erstes, zweites);
    }

    #[test_log::test]
    fn archiviertes_ziel_verweigert_jede_aenderung() {
        let mut ziel = ziel();
        ziel.archiviere().unwrap();
        assert_eq!(ziel.zustand(), Zustand::Archiviert);

        assert!(matches!(ziel.archiviere(), Err(ZielFehler::Archiviert)));
        assert!(matches!(
            ziel.tiername_ändern(name("Reh")),
            Err(ZielFehler::Archiviert)
        ));
        assert!(matches!(
            ziel.zielnummer_ändern(nummer(5)),
            Err(ZielFehler::Archiviert)
        ));
        assert!(matches!(
            ziel.stammdaten_ändern(name("Reh"), nummer(5)),
            Err(ZielFehler::Archiviert)
        ));
        // Duplication is a read: an archived Ziel may be duplicated like any other.
        assert_eq!(ziel.duplicate().tiername().as_str(), "Keiler");
    }

    #[test_log::test]
    fn aendern_haelt_identitaet_und_nummer() {
        let mut ziel = ziel();
        let id = ziel.id();
        ziel.tiername_ändern(name("Reh")).unwrap();
        assert_eq!(ziel.id(), id);
        assert_eq!(ziel.tiername().as_str(), "Reh");
        assert_eq!(ziel.zielnummer().get(), 2);
    }

    #[test_log::test]
    fn die_zielnummer_ist_aenderbar() {
        let mut ziel = ziel();
        let id = ziel.id();
        ziel.zielnummer_ändern(nummer(7)).unwrap();
        assert_eq!(ziel.id(), id);
        assert_eq!(ziel.zielnummer().get(), 7);
        assert_eq!(ziel.tiername().as_str(), "Keiler");

        ziel.stammdaten_ändern(name("Reh"), nummer(3)).unwrap();
        assert_eq!(ziel.tiername().as_str(), "Reh");
        assert_eq!(ziel.zielnummer().get(), 3);
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let original = ziel();
        let geladen = AnlageZiel::from_parts(
            original.id(),
            original.tiername().clone(),
            original.zielnummer(),
            original.zustand(),
        );
        assert_eq!(original, geladen);
    }
}
