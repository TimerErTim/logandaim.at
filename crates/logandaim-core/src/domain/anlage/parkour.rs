//! A named route at one `Anlage`: its Pflock setup and its ordered Ziel arrangement.
//!
//! A `Parkour` is an entity inside the `Anlage` aggregate. Its Pflock colours and its
//! near-to-far order are its own, and there is no product-wide palette (SPEC-stammdaten
//! rule 6, AC-6).

use uuid::Uuid;

use super::ziel::{ZielId, Zustand};
use crate::domain::common::{Abstand, Farbe, NonEmptyString};

/// Identity of one Parkour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParkourId(pub Uuid);

/// Identity of one Pflock setup (a shooting position at one Parkour).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PflockId(pub Uuid);

#[derive(Debug, thiserror::Error)]
pub enum ParkourFehler {
    #[error("die Pflock-Farbe ist in diesem Parkour doppelt")]
    FarbeDoppelt,
    #[error("ein Parkour braucht mindestens einen Pflock")]
    KeinePflöcke,
    #[error("die Reihenfolge der Pflöcke ist widersprüchlich")]
    PflockReihenfolgeUngültig,
    #[error("die Abstände müssen vom ersten Pflock zum letzten streng zunehmen")]
    AbständeNichtSteigend,
    #[error("ein Abstand gehört zu einem Pflock, der nicht in diesem Parkour liegt")]
    PflockNichtImParkour,
    #[error("dasselbe Ziel darf in einem Parkour nur einmal vorkommen")]
    ZielDoppelt,
    #[error("das Ziel gehört nicht zu dieser Anlage")]
    ZielFremdeAnlage,
    #[error("die neue Reihenfolge deckt nicht genau die platzierten Ziele ab")]
    ReihenfolgeUngültig,
    #[error("der Parkour wurde nicht gefunden")]
    ParkourNichtGefunden,
    #[error("der Parkour ist archiviert und kann nicht mehr geändert werden")]
    Archiviert,
}

/// One shooting position in a Parkour's setup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pflock {
    id: PflockId,
    farbe: Farbe,
}

impl Pflock {
    pub fn neu(id: PflockId, farbe: Farbe) -> Self {
        Self { id, farbe }
    }

    pub fn id(&self) -> PflockId {
        self.id
    }

    pub fn farbe(&self) -> Farbe {
        self.farbe
    }
}

/// Whether one Ziel uses the Parkour-wide Pflock order or its own.
///
/// The enum *is* the field: there is no `Vec<Pflock>` with an empty-means-inherit sentinel,
/// so "this Ziel has no Pflöcke of its own" cannot be confused with "this Ziel has not been
/// configured". An `Explicit` set always has at least one Pflock and replaces the
/// Parkour-wide order for that Ziel alone (SPEC-stammdaten rule 6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZielPflöcke {
    /// The Parkour's own near-to-far Pflock order applies to this Ziel.
    InheritFromParkour,
    /// This Ziel's own near-to-far order; always non-empty and replaces the Parkour order.
    Explicit(Vec<Pflock>),
}

impl ZielPflöcke {
    /// This Ziel uses the Parkour-wide order.
    pub fn erbe_von_parkour() -> Self {
        Self::InheritFromParkour
    }

    /// Replace the Parkour-wide order with this Ziel's own non-empty set.
    pub fn überschreibe(pflöcke: Vec<Pflock>) -> Result<Self, ParkourFehler> {
        if pflöcke.is_empty() {
            return Err(ParkourFehler::KeinePflöcke);
        }
        for (index, pflock) in pflöcke.iter().enumerate() {
            if pflöcke[..index]
                .iter()
                .any(|früher| früher.farbe == pflock.farbe)
            {
                return Err(ParkourFehler::FarbeDoppelt);
            }
        }
        Ok(Self::Explicit(pflöcke))
    }

    /// The Pflöcke that actually apply to this Ziel: its own set when it has one, the
    /// Parkour-wide set otherwise.
    pub fn effektive<'a>(&'a self, parkour: &'a [Pflock]) -> &'a [Pflock] {
        match self {
            Self::InheritFromParkour => parkour,
            Self::Explicit(eigene) => eigene,
        }
    }
}

/// One Ziel's use in one Parkour, with an optional distance per Pflock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zielplatzierung {
    ziel_id: ZielId,
    /// Which Pflock order applies to this Ziel: the Parkour-wide one or its own.
    pflöcke: ZielPflöcke,
    /// Near to far in the effective Pflock order. A missing entry is a missing distance,
    /// not a zero.
    abstände: Vec<(PflockId, Abstand)>,
}

impl Zielplatzierung {
    /// A placement that inherits the Parkour-wide Pflock order.
    pub fn neu(
        ziel_id: ZielId, abstände: Vec<(PflockId, Abstand)>
    ) -> Result<Self, ParkourFehler> {
        Self::neu_mit_abständen_und_pflöcken(
            ziel_id,
            ZielPflöcke::InheritFromParkour,
            abstände,
            &[],
        )
    }

    /// A placement with its own near-to-far Pflock order, which replaces the Parkour's.
    pub fn neu_mit_eigenen_pflöcken(
        ziel_id: ZielId,
        pflöcke: Vec<Pflock>,
        abstände: Vec<(PflockId, Abstand)>,
    ) -> Result<Self, ParkourFehler> {
        Self::neu_mit_abständen_und_pflöcken(
            ziel_id,
            ZielPflöcke::überschreibe(pflöcke)?,
            abstände,
            &[],
        )
    }

    /// Validate distances against the effective Pflock set and store them in its order.
    pub fn neu_mit_abständen_und_pflöcken(
        ziel_id: ZielId,
        pflöcke: ZielPflöcke,
        mut abstände: Vec<(PflockId, Abstand)>,
        parkour_pflöcke: &[Pflock],
    ) -> Result<Self, ParkourFehler> {
        let pflöcke = match pflöcke {
            ZielPflöcke::InheritFromParkour => ZielPflöcke::InheritFromParkour,
            ZielPflöcke::Explicit(eigene) => ZielPflöcke::überschreibe(eigene)?,
        };
        let effektive = pflöcke.effektive(parkour_pflöcke);
        if !effektive.is_empty() {
            for (pflock_id, _) in &abstände {
                if !effektive.iter().any(|pflock| pflock.id == *pflock_id) {
                    return Err(ParkourFehler::PflockNichtImParkour);
                }
            }
            // Near to far: strictly increasing in Pflock order. Only configured
            // distances participate, because a missing one has no estimate.
            let reihe: Vec<u32> = effektive
                .iter()
                .filter_map(|pflock| {
                    abstände
                        .iter()
                        .find(|(id, _)| *id == pflock.id)
                        .map(|(_, abstand)| abstand.meter())
                })
                .collect();
            if reihe.windows(2).any(|paar| paar[0] >= paar[1]) {
                return Err(ParkourFehler::AbständeNichtSteigend);
            }
            // Normalise to the effective set's own near-to-far order, so the list always
            // reads first Pflock first whatever order the caller supplied.
            abstände = effektive
                .iter()
                .filter_map(|pflock| abstände.iter().find(|(id, _)| *id == pflock.id).copied())
                .collect();
        }
        Ok(Self {
            ziel_id,
            pflöcke,
            abstände,
        })
    }

    pub fn ziel_id(&self) -> ZielId {
        self.ziel_id
    }

    /// Whether this Ziel inherits the Parkour-wide Pflock order or carries its own.
    pub fn pflöcke(&self) -> &ZielPflöcke {
        &self.pflöcke
    }

    /// Which Pflock order applies to this Ziel (see [`ZielPflöcke::effektive`]).
    pub fn effektive_pflöcke<'a>(&'a self, parkour: &'a [Pflock]) -> &'a [Pflock] {
        self.pflöcke.effektive(parkour)
    }

    pub fn abstände(&self) -> &[(PflockId, Abstand)] {
        &self.abstände
    }

    /// The configured distance for one Pflock, if any.
    pub fn abstand_für(&self, pflock_id: PflockId) -> Option<Abstand> {
        self.abstände
            .iter()
            .find(|(id, _)| *id == pflock_id)
            .map(|(_, abstand)| *abstand)
    }

    /// Replace the distance map, re-validating against the effective set.
    fn setze_abstände(
        &mut self,
        abstände: Vec<(PflockId, Abstand)>,
        parkour_pflöcke: &[Pflock],
    ) -> Result<(), ParkourFehler> {
        let neu = Self::neu_mit_abständen_und_pflöcken(
            self.ziel_id,
            self.pflöcke.clone(),
            abstände,
            parkour_pflöcke,
        )?;
        self.abstände = neu.abstände;
        Ok(())
    }
}

/// Creation input of a Parkour: the Pflock setup plus the ordered Ziel list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuerParkour {
    name: NonEmptyString,
    pflöcke: Vec<Pflock>,
    platzierungen: Vec<Zielplatzierung>,
}

impl NeuerParkour {
    /// Validate the invariants: unique Pflock colours, at least one Pflock, no duplicate
    /// Ziel placement, and distances that increase near to far (AC-4, AC-6).
    pub fn neu(
        name: NonEmptyString,
        pflöcke: Vec<Pflock>,
        platzierungen: Vec<Zielplatzierung>,
    ) -> Result<Self, ParkourFehler> {
        if pflöcke.is_empty() {
            return Err(ParkourFehler::KeinePflöcke);
        }
        for (index, pflock) in pflöcke.iter().enumerate() {
            if pflöcke[..index]
                .iter()
                .any(|früher| früher.farbe == pflock.farbe)
            {
                return Err(ParkourFehler::FarbeDoppelt);
            }
        }
        let platzierungen = Self::pruefe_platzierungen(&pflöcke, platzierungen)?;
        Ok(Self {
            name,
            pflöcke,
            platzierungen,
        })
    }

    /// Validate the placements and return them re-expressed in Pflock order, so the stored
    /// list reads first Pflock first whatever order the caller supplied.
    ///
    /// Each placement is re-validated against its **effective** set: its own when it carries
    /// one (which is also checked for unique colours within itself), the Parkour-wide set
    /// otherwise.
    fn pruefe_platzierungen(
        pflöcke: &[Pflock],
        platzierungen: Vec<Zielplatzierung>,
    ) -> Result<Vec<Zielplatzierung>, ParkourFehler> {
        let mut geprüft = Vec::with_capacity(platzierungen.len());
        for (index, platzierung) in platzierungen.iter().enumerate() {
            if platzierungen[..index]
                .iter()
                .any(|früher| früher.ziel_id == platzierung.ziel_id)
            {
                return Err(ParkourFehler::ZielDoppelt);
            }
            // Re-validate: a caller may hand in a placement built with `neu` that never
            // saw the Pflock list, or one whose enum was built by hand.
            geprüft.push(Zielplatzierung::neu_mit_abständen_und_pflöcken(
                platzierung.ziel_id,
                platzierung.pflöcke.clone(),
                platzierung.abstände.clone(),
                pflöcke,
            )?);
        }
        Ok(geprüft)
    }

    pub fn name(&self) -> &NonEmptyString {
        &self.name
    }

    pub fn pflöcke(&self) -> &[Pflock] {
        &self.pflöcke
    }

    pub fn platzierungen(&self) -> &[Zielplatzierung] {
        &self.platzierungen
    }
}

/// One named route at one Anlage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parkour {
    id: ParkourId,
    name: NonEmptyString,
    /// Near to far: the first Pflock is the nearest. Order is never inferred from a colour.
    pflöcke: Vec<Pflock>,
    /// In route order.
    platzierungen: Vec<Zielplatzierung>,
    zustand: Zustand,
}

impl Parkour {
    pub fn neu(id: ParkourId, neu: NeuerParkour) -> Self {
        Self {
            id,
            name: neu.name,
            pflöcke: neu.pflöcke,
            platzierungen: neu.platzierungen,
            zustand: Zustand::Aktiv,
        }
    }

    /// Rebuild a persisted Parkour. The re-validation also normalises every placement's
    /// distance list into the Parkour's near-to-far Pflock order, so a stored row order the
    /// database did not guarantee cannot leak into the aggregate.
    pub fn from_parts(
        id: ParkourId,
        name: NonEmptyString,
        pflöcke: Vec<Pflock>,
        platzierungen: Vec<Zielplatzierung>,
        zustand: Zustand,
    ) -> Result<Self, ParkourFehler> {
        let neu = NeuerParkour::neu(name, pflöcke, platzierungen)?;
        Ok(Self {
            id,
            name: neu.name,
            pflöcke: neu.pflöcke,
            platzierungen: neu.platzierungen,
            zustand,
        })
    }

    pub fn id(&self) -> ParkourId {
        self.id
    }

    pub fn name(&self) -> &NonEmptyString {
        &self.name
    }

    pub fn pflöcke(&self) -> &[Pflock] {
        &self.pflöcke
    }

    /// One Pflock of this Parkour by identity, carrying its colour. `None` when the id does
    /// not belong to this Parkour.
    pub fn pflock(&self, pflock_id: PflockId) -> Option<&Pflock> {
        self.pflöcke.iter().find(|pflock| pflock.id == pflock_id)
    }

    pub fn platzierungen(&self) -> &[Zielplatzierung] {
        &self.platzierungen
    }

    pub fn zustand(&self) -> Zustand {
        self.zustand
    }

    pub fn ist_archiviert(&self) -> bool {
        matches!(self.zustand, Zustand::Archiviert)
    }

    fn nur_aktiv(&self) -> Result<(), ParkourFehler> {
        match self.zustand {
            Zustand::Aktiv => Ok(()),
            Zustand::Archiviert => Err(ParkourFehler::Archiviert),
        }
    }

    /// Rename the Parkour. Refused while archived.
    pub fn umbenennen(&mut self, name: NonEmptyString) -> Result<(), ParkourFehler> {
        self.nur_aktiv()?;
        self.name = name;
        Ok(())
    }

    /// Replace the whole shooting setup (Pflöcke and placement order). Refused while
    /// archived, and refuses a placement whose distances contradict the new Pflöcke.
    pub fn setup_ändern(
        &mut self,
        pflöcke: Vec<Pflock>,
        platzierungen: Vec<Zielplatzierung>,
    ) -> Result<(), ParkourFehler> {
        self.nur_aktiv()?;
        let geprüft = NeuerParkour::neu(self.name.clone(), pflöcke, platzierungen)?;
        self.pflöcke = geprüft.pflöcke;
        self.platzierungen = geprüft.platzierungen;
        Ok(())
    }

    /// Add one Ziel placement at the end of the route. A Ziel already placed is refused.
    pub fn platzierung_hinzufügen(
        &mut self,
        platzierung: Zielplatzierung,
    ) -> Result<(), ParkourFehler> {
        self.nur_aktiv()?;
        if self
            .platzierungen
            .iter()
            .any(|vorhanden| vorhanden.ziel_id == platzierung.ziel_id)
        {
            return Err(ParkourFehler::ZielDoppelt);
        }
        let geprüft = Zielplatzierung::neu_mit_abständen_und_pflöcken(
            platzierung.ziel_id,
            platzierung.pflöcke,
            platzierung.abstände,
            &self.pflöcke,
        )?;
        self.platzierungen.push(geprüft);
        Ok(())
    }

    /// Remove one Ziel from the current arrangement; its history is untouched (AC-11).
    pub fn platzierung_entfernen(&mut self, ziel_id: ZielId) -> Result<(), ParkourFehler> {
        self.nur_aktiv()?;
        self.platzierungen
            .retain(|platzierung| platzierung.ziel_id != ziel_id);
        Ok(())
    }

    /// Reorder the route. Every listed id must already be placed, and the result must
    /// still cover the same set.
    pub fn reihenfolge_ändern(&mut self, reihenfolge: &[ZielId]) -> Result<(), ParkourFehler> {
        self.nur_aktiv()?;
        if reihenfolge.len() != self.platzierungen.len() {
            return Err(ParkourFehler::ReihenfolgeUngültig);
        }
        let mut neu: Vec<Zielplatzierung> = Vec::with_capacity(reihenfolge.len());
        for ziel_id in reihenfolge {
            let Some(platzierung) = self
                .platzierungen
                .iter()
                .find(|platzierung| platzierung.ziel_id == *ziel_id)
            else {
                return Err(ParkourFehler::ReihenfolgeUngültig);
            };
            if neu.iter().any(|vorhanden| vorhanden.ziel_id == *ziel_id) {
                return Err(ParkourFehler::ReihenfolgeUngültig);
            }
            neu.push(platzierung.clone());
        }
        self.platzierungen = neu;
        Ok(())
    }

    /// Swap the physical Ziel of one placement in place, keeping its route position and its
    /// distances (AC-5: a replaced target is a new identity at the same slot).
    pub fn platzierung_ersetzen(&mut self, alt: ZielId, neu: ZielId) -> Result<(), ParkourFehler> {
        self.nur_aktiv()?;
        if self
            .platzierungen
            .iter()
            .any(|platzierung| platzierung.ziel_id == neu)
        {
            return Err(ParkourFehler::ZielDoppelt);
        }
        let Some(platzierung) = self
            .platzierungen
            .iter_mut()
            .find(|platzierung| platzierung.ziel_id == alt)
        else {
            return Ok(());
        };
        platzierung.ziel_id = neu;
        Ok(())
    }

    /// Set one placement's distance map, re-validated against this Parkour's Pflöcke.
    pub fn abstände_setzen(
        &mut self,
        ziel_id: ZielId,
        abstände: Vec<(PflockId, Abstand)>,
    ) -> Result<(), ParkourFehler> {
        self.nur_aktiv()?;
        let pflöcke = &self.pflöcke;
        let Some(platzierung) = self
            .platzierungen
            .iter_mut()
            .find(|platzierung| platzierung.ziel_id == ziel_id)
        else {
            return Err(ParkourFehler::ZielFremdeAnlage);
        };
        platzierung.setze_abstände(abstände, pflöcke)
    }

    /// Archive the Parkour: it leaves active selection and keeps its history.
    pub fn archiviere(&mut self) -> Result<(), ParkourFehler> {
        self.nur_aktiv()?;
        self.zustand = Zustand::Archiviert;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::anlage::ziel::Zustand;
    use Abstand;

    fn name(wert: &str) -> NonEmptyString {
        NonEmptyString::neu(wert).unwrap()
    }

    fn pflock(id: u128, farbe: &str) -> Pflock {
        Pflock::neu(PflockId(Uuid::from_u128(id)), Farbe::neu(farbe).unwrap())
    }

    fn drei_pflöcke() -> Vec<Pflock> {
        vec![
            pflock(1, "#c0392b"),
            pflock(2, "#1b4f9c"),
            pflock(3, "#e8b81c"),
        ]
    }

    fn ziel(id: u128) -> ZielId {
        ZielId(Uuid::from_u128(id))
    }

    fn platzierung(ziel_id: ZielId, abstände: &[(u128, u32)]) -> Zielplatzierung {
        Zielplatzierung::neu(
            ziel_id,
            abstände
                .iter()
                .map(|&(pflock_id, meter)| {
                    (
                        PflockId(Uuid::from_u128(pflock_id)),
                        Abstand::neu(meter).unwrap(),
                    )
                })
                .collect(),
        )
        .unwrap()
    }

    fn parkour() -> Parkour {
        Parkour::neu(
            ParkourId(Uuid::nil()),
            NeuerParkour::neu(
                name("Waldrunde"),
                drei_pflöcke(),
                vec![
                    platzierung(ziel(10), &[(1, 20), (2, 30), (3, 40)]),
                    platzierung(ziel(11), &[]),
                ],
            )
            .unwrap(),
        )
    }

    #[test_log::test]
    fn ac6_pflock_farben_sind_eindeutig() {
        let doppelt = vec![pflock(1, "#c0392b"), pflock(2, "#C0392B")];
        let ergebnis = NeuerParkour::neu(name("Waldrunde"), doppelt, Vec::new());
        assert!(matches!(ergebnis, Err(ParkourFehler::FarbeDoppelt)));
    }

    #[test_log::test]
    fn ac6_abstaende_nehmen_vom_ersten_zum_letzten_pflock_streng_zu() {
        let gleich = NeuerParkour::neu(
            name("Waldrunde"),
            drei_pflöcke(),
            vec![platzierung(ziel(10), &[(1, 30), (2, 30)])],
        );
        assert!(matches!(gleich, Err(ParkourFehler::AbständeNichtSteigend)));

        let fallend = NeuerParkour::neu(
            name("Waldrunde"),
            drei_pflöcke(),
            vec![platzierung(ziel(10), &[(1, 40), (2, 30)])],
        );
        assert!(matches!(fallend, Err(ParkourFehler::AbständeNichtSteigend)));

        let steigend = NeuerParkour::neu(
            name("Waldrunde"),
            drei_pflöcke(),
            vec![platzierung(ziel(10), &[(1, 20), (2, 30)])],
        );
        assert!(steigend.is_ok());
    }

    #[test_log::test]
    fn ac6_fehlende_abstaende_sind_erlaubt() {
        let ergebnis = NeuerParkour::neu(
            name("Waldrunde"),
            drei_pflöcke(),
            vec![platzierung(ziel(10), &[(3, 40)])],
        );
        assert!(ergebnis.is_ok());
    }

    #[test_log::test]
    fn ac4_dasselbe_ziel_darf_nur_einmal_vorkommen() {
        let ergebnis = NeuerParkour::neu(
            name("Waldrunde"),
            drei_pflöcke(),
            vec![
                platzierung(ziel(10), &[]),
                platzierung(ziel(10), &[(1, 40)]),
            ],
        );
        assert!(matches!(ergebnis, Err(ParkourFehler::ZielDoppelt)));
    }

    #[test_log::test]
    fn ein_parkour_braucht_mindestens_einen_pflock() {
        let ergebnis = NeuerParkour::neu(name("Waldrunde"), Vec::new(), Vec::new());
        assert!(matches!(ergebnis, Err(ParkourFehler::KeinePflöcke)));
    }

    #[test_log::test]
    fn ac6_reihenfolge_ist_unabhaengig_von_der_farbe() {
        // Distinct colours, but the order still comes only from the list.
        let parkour = Parkour::neu(
            ParkourId(Uuid::nil()),
            NeuerParkour::neu(name("Waldrunde"), drei_pflöcke(), Vec::new()).unwrap(),
        );
        let farben: Vec<_> = parkour
            .pflöcke()
            .iter()
            .map(|pflock| pflock.farbe().to_hex())
            .collect();
        assert_eq!(farben, ["#c0392b", "#1b4f9c", "#e8b81c"]);
    }

    #[test_log::test]
    fn reihenfolge_aendern_haelt_die_zielmenge() {
        let mut parkour = parkour();
        parkour.reihenfolge_ändern(&[ziel(11), ziel(10)]).unwrap();
        let ids: Vec<_> = parkour
            .platzierungen()
            .iter()
            .map(|platzierung| platzierung.ziel_id())
            .collect();
        assert_eq!(ids, [ziel(11), ziel(10)]);

        assert!(parkour.reihenfolge_ändern(&[ziel(10)]).is_err());
        assert!(parkour.reihenfolge_ändern(&[ziel(10), ziel(10)]).is_err());
    }

    #[test_log::test]
    fn abstände_setzen_prueft_die_pflockliste() {
        let mut parkour = parkour();
        assert!(matches!(
            parkour.abstände_setzen(
                ziel(10),
                vec![(PflockId(Uuid::from_u128(99)), Abstand::neu(10).unwrap())]
            ),
            Err(ParkourFehler::PflockNichtImParkour)
        ));
    }

    #[test_log::test]
    fn ac11_archivierter_parkour_verweigert_jede_aenderung() {
        let mut parkour = parkour();
        parkour.archiviere().unwrap();
        assert!(parkour.ist_archiviert());
        assert!(matches!(
            parkour.umbenennen(name("Neu")),
            Err(ParkourFehler::Archiviert)
        ));
        assert!(matches!(
            parkour.archiviere(),
            Err(ParkourFehler::Archiviert)
        ));
        assert!(matches!(
            parkour.platzierung_entfernen(ziel(10)),
            Err(ParkourFehler::Archiviert)
        ));
    }

    #[test_log::test]
    fn ein_ziel_hat_in_jedem_parkour_eigene_abstaende() {
        let a = parkour();
        let b = Parkour::neu(
            ParkourId(Uuid::from_u128(2)),
            NeuerParkour::neu(
                name("Feldrunde"),
                drei_pflöcke(),
                vec![platzierung(ziel(10), &[(1, 35), (2, 45), (3, 55)])],
            )
            .unwrap(),
        );
        assert_eq!(
            a.platzierungen()[0].abstand_für(PflockId(Uuid::from_u128(1))),
            Some(Abstand::neu(20).unwrap())
        );
        assert_eq!(
            b.platzierungen()[0].abstand_für(PflockId(Uuid::from_u128(1))),
            Some(Abstand::neu(35).unwrap())
        );
    }

    #[test_log::test]
    fn from_parts_ordnet_die_abstaende_nah_zu_fern() {
        // A stored row order the database did not guarantee: the rebuild still reads first
        // Pflock first.
        let ungeordnet = parkour();
        let geladen = Parkour::from_parts(
            ungeordnet.id(),
            ungeordnet.name().clone(),
            ungeordnet.pflöcke().to_vec(),
            vec![platzierung(ziel(10), &[(2, 30), (1, 20)])],
            Zustand::Aktiv,
        )
        .unwrap();
        let reihenfolge: Vec<PflockId> = geladen.platzierungen()[0]
            .abstände()
            .iter()
            .map(|(pflock_id, _)| *pflock_id)
            .collect();
        assert_eq!(
            reihenfolge,
            vec![PflockId(Uuid::from_u128(1)), PflockId(Uuid::from_u128(2))]
        );
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let original = parkour();
        let geladen = Parkour::from_parts(
            original.id(),
            original.name().clone(),
            original.pflöcke().to_vec(),
            original.platzierungen().to_vec(),
            Zustand::Aktiv,
        )
        .unwrap();
        assert_eq!(original, geladen);
    }

    fn eigene_platzierung() -> (Vec<Pflock>, Zielplatzierung) {
        // Colours deliberately reused from the Parkour-wide set: allowed across sets.
        let eigene = vec![pflock(11, "#1b4f9c"), pflock(12, "#e8b81c")];
        let platzierung = Zielplatzierung::neu_mit_eigenen_pflöcken(
            ziel(10),
            eigene.clone(),
            vec![
                (PflockId(Uuid::from_u128(11)), Abstand::neu(15).unwrap()),
                (PflockId(Uuid::from_u128(12)), Abstand::neu(45).unwrap()),
            ],
        )
        .unwrap();
        (eigene, platzierung)
    }

    #[test_log::test]
    fn zielpfloecke_eigen_refuses_empty_and_duplicate_colours() {
        assert!(matches!(
            ZielPflöcke::überschreibe(Vec::new()),
            Err(ParkourFehler::KeinePflöcke)
        ));
        assert!(matches!(
            ZielPflöcke::überschreibe(vec![pflock(1, "#c0392b"), pflock(2, "#C0392B")]),
            Err(ParkourFehler::FarbeDoppelt)
        ));
        assert!(ZielPflöcke::überschreibe(vec![pflock(1, "#c0392b")]).is_ok());
    }

    #[test_log::test]
    fn zielpfloecke_erbe_carries_nothing() {
        let erbe = ZielPflöcke::erbe_von_parkour();
        assert!(matches!(erbe, ZielPflöcke::InheritFromParkour));
        let parkour = drei_pflöcke();
        assert_eq!(erbe.effektive(&parkour).len(), 3);
    }

    #[test_log::test]
    fn ac6_eigene_pfloecke_ersetzen_die_parkourreihenfolge() {
        let (eigene, platzierung) = eigene_platzierung();
        assert!(matches!(platzierung.pflöcke(), ZielPflöcke::Explicit(_)));
        assert_eq!(platzierung.effektive_pflöcke(&drei_pflöcke()), &eigene[..]);

        let parkour = Parkour::neu(
            ParkourId(Uuid::nil()),
            NeuerParkour::neu(name("Waldrunde"), drei_pflöcke(), vec![platzierung]).unwrap(),
        );
        let gespeichert = &parkour.platzierungen()[0];
        // Replace semantics: the Parkour-wide order does not appear in this Ziel's distances.
        assert_eq!(
            gespeichert.abstand_für(PflockId(Uuid::from_u128(1))),
            None,
            "ein eigener Pflock-Satz ignoriert die Parkour-Pflöcke"
        );
        assert_eq!(
            gespeichert.abstand_für(PflockId(Uuid::from_u128(11))),
            Some(Abstand::neu(15).unwrap())
        );
        assert_eq!(gespeichert.abstände().len(), 2);
    }

    #[test_log::test]
    fn ac6_eigene_pfloecke_validieren_abstaende_gegen_ihre_eigene_reihenfolge() {
        // 45 m at the second own Pflock but 15 m at the first: increasing, accepted.
        let (_, gueltig) = eigene_platzierung();
        assert!(NeuerParkour::neu(name("Waldrunde"), drei_pflöcke(), vec![gueltig]).is_ok());

        // The Parkour-wide Pflock is not part of this Ziel's own set, so the distance is
        // refused at construction already.
        let fremd = Zielplatzierung::neu_mit_eigenen_pflöcken(
            ziel(10),
            vec![pflock(11, "#1b4f9c")],
            vec![(PflockId(Uuid::from_u128(2)), Abstand::neu(20).unwrap())],
        );
        assert!(matches!(fremd, Err(ParkourFehler::PflockNichtImParkour)));

        // Falling own order is refused just like the inherited one — at construction, since
        // the own set is its own effective set.
        let fallend = Zielplatzierung::neu_mit_eigenen_pflöcken(
            ziel(10),
            vec![pflock(11, "#1b4f9c"), pflock(12, "#e8b81c")],
            vec![
                (PflockId(Uuid::from_u128(11)), Abstand::neu(40).unwrap()),
                (PflockId(Uuid::from_u128(12)), Abstand::neu(30).unwrap()),
            ],
        );
        assert!(matches!(fallend, Err(ParkourFehler::AbständeNichtSteigend)));
    }

    #[test_log::test]
    fn ac6_gleiche_farbe_ueber_saetze_hinweg_ist_erlaubt() {
        // "#1b4f9c" is both the Parkour-wide second Pflock and the first own one.
        let (_, platzierung) = eigene_platzierung();
        assert!(NeuerParkour::neu(name("Waldrunde"), drei_pflöcke(), vec![platzierung]).is_ok());
    }

    #[test_log::test]
    fn ac6_ein_handgebauter_leerer_satz_wird_abgelehnt() {
        // The variant is public, so every constructor path re-asserts the invariant.
        let leer = Zielplatzierung::neu_mit_abständen_und_pflöcken(
            ziel(10),
            ZielPflöcke::Explicit(Vec::new()),
            Vec::new(),
            &drei_pflöcke(),
        );
        assert!(matches!(leer, Err(ParkourFehler::KeinePflöcke)));
    }

    #[test_log::test]
    fn abstände_setzen_prueft_die_effektive_liste() {
        let (_, mit_eigene) = eigene_platzierung();
        let mut parkour = Parkour::neu(
            ParkourId(Uuid::nil()),
            NeuerParkour::neu(
                name("Waldrunde"),
                drei_pflöcke(),
                vec![mit_eigene, platzierung(ziel(11), &[])],
            )
            .unwrap(),
        );
        // An inherited-set Pflock is not part of the own set.
        assert!(matches!(
            parkour.abstände_setzen(
                ziel(10),
                vec![(PflockId(Uuid::from_u128(1)), Abstand::neu(10).unwrap())]
            ),
            Err(ParkourFehler::PflockNichtImParkour)
        ));
        parkour
            .abstände_setzen(
                ziel(10),
                vec![(PflockId(Uuid::from_u128(12)), Abstand::neu(25).unwrap())],
            )
            .unwrap();
        assert_eq!(
            parkour.platzierungen()[0].abstand_für(PflockId(Uuid::from_u128(12))),
            Some(Abstand::neu(25).unwrap())
        );
    }

    #[test_log::test]
    fn platzierung_hinzufuegen_validiert_gegen_den_eigenen_satz() {
        let mut parkour = parkour();
        let eigene = vec![pflock(11, "#1b4f9c"), pflock(12, "#e8b81c")];
        let hinzuzufügen = Zielplatzierung::neu_mit_eigenen_pflöcken(
            ziel(12),
            eigene,
            vec![(PflockId(Uuid::from_u128(11)), Abstand::neu(15).unwrap())],
        )
        .unwrap();
        parkour.platzierung_hinzufügen(hinzuzufügen).unwrap();
        let hinzugefügt = &parkour.platzierungen()[2];
        assert!(matches!(hinzugefügt.pflöcke(), ZielPflöcke::Explicit(_)));
        assert_eq!(hinzugefügt.abstände().len(), 1);
    }
}
