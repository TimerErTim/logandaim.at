//! A named route at one `Anlage`: its Pflock setup and its ordered Ziel arrangement.
//!
//! A `Parcours` is an entity inside the `Anlage` aggregate. Its Pflock labels and
//! far-to-near order are its own, and there is no product-wide palette (SPEC-stammdaten
//! rule 6, AC-6).

use uuid::Uuid;

use super::ziel::{ZielId, Zustand};

/// Identity of one Parcours.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ParcoursId(pub Uuid);

/// Identity of one Pflock setup (a shooting-position label at one Parcours).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PflockId(pub Uuid);

#[derive(Debug, thiserror::Error)]
pub enum ParcoursFehler {
    #[error("der Parcoursname darf nicht leer sein")]
    NameLeer,
    #[error("die Pflock-Bezeichnung darf nicht leer sein")]
    PflockBezeichnungLeer,
    #[error("die Pflock-Bezeichnung „{0}“ ist in diesem Parcours doppelt")]
    PflockBezeichnungDoppelt(String),
    #[error("die Pflock-Farbe darf nicht leer sein")]
    PflockFarbeLeer,
    #[error("ein Parcours braucht mindestens einen Pflock")]
    KeinePflöcke,
    #[error("die Reihenfolge der Pflöcke ist widersprüchlich")]
    PflockReihenfolgeUngültig,
    #[error("der Abstand muss positiv sein, war aber {0} m")]
    AbstandNichtPositiv(u32),
    #[error("die Abstände müssen von fern nach nah streng abnehmen")]
    AbständeNichtAbnehmend,
    #[error("ein Abstand gehört zu einem Pflock, der nicht in diesem Parcours liegt")]
    PflockNichtImParcours,
    #[error("dasselbe Ziel darf in einem Parcours nur einmal vorkommen")]
    ZielDoppelt,
    #[error("das Ziel gehört nicht zu dieser Anlage")]
    ZielFremdeAnlage,
    #[error("die neue Reihenfolge deckt nicht genau die platzierten Ziele ab")]
    ReihenfolgeUngültig,
    #[error("der Parcours wurde nicht gefunden")]
    ParcoursNichtGefunden,
    #[error("der Parcours ist archiviert und kann nicht mehr geändert werden")]
    Archiviert,
}

/// Non-empty name of one Parcours.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParcoursName(String);

impl ParcoursName {
    pub fn neu<S: AsRef<str>>(wert: S) -> Result<Self, ParcoursFehler> {
        let wert = wert.as_ref().trim();
        if wert.is_empty() {
            return Err(ParcoursFehler::NameLeer);
        }
        Ok(Self(wert.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ParcoursName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Label of one shooting position inside one Parcours.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PflockBezeichnung(String);

impl PflockBezeichnung {
    pub fn neu<S: AsRef<str>>(wert: S) -> Result<Self, ParcoursFehler> {
        let wert = wert.as_ref().trim();
        if wert.is_empty() {
            return Err(ParcoursFehler::PflockBezeichnungLeer);
        }
        Ok(Self(wert.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PflockBezeichnung {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Colour of one Pflock, chosen by Anlage staff.
///
/// Caller data, not a design token: the product has no fixed palette (rule 6). Stored
/// verbatim; the admin UI renders it as a swatch.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PflockFarbe(String);

impl PflockFarbe {
    pub fn neu<S: AsRef<str>>(wert: S) -> Result<Self, ParcoursFehler> {
        let wert = wert.as_ref().trim();
        if wert.is_empty() {
            return Err(ParcoursFehler::PflockFarbeLeer);
        }
        Ok(Self(wert.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PflockFarbe {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// One shooting position in a Parcours' setup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pflock {
    id: PflockId,
    bezeichnung: PflockBezeichnung,
    farbe: PflockFarbe,
}

impl Pflock {
    pub fn neu(id: PflockId, bezeichnung: PflockBezeichnung, farbe: PflockFarbe) -> Self {
        Self {
            id,
            bezeichnung,
            farbe,
        }
    }

    pub fn id(&self) -> PflockId {
        self.id
    }

    pub fn bezeichnung(&self) -> &PflockBezeichnung {
        &self.bezeichnung
    }

    pub fn farbe(&self) -> &PflockFarbe {
        &self.farbe
    }
}

/// A configured distance in metres. Positive by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Abstand(u32);

impl Abstand {
    pub fn neu(meter: u32) -> Result<Self, ParcoursFehler> {
        if meter == 0 {
            return Err(ParcoursFehler::AbstandNichtPositiv(0));
        }
        Ok(Self(meter))
    }

    pub fn meter(self) -> u32 {
        self.0
    }
}

/// One Ziel's use in one Parcours, with an optional distance per Pflock.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zielplatzierung {
    ziel_id: ZielId,
    /// In far-to-near order of the Parcours' Pflöcke. A missing entry is a missing
    /// distance, not a zero.
    abstände: Vec<(PflockId, Abstand)>,
}

impl Zielplatzierung {
    pub fn neu(
        ziel_id: ZielId,
        abstände: Vec<(PflockId, Abstand)>,
    ) -> Result<Self, ParcoursFehler> {
        Self::mit_pflöcken(ziel_id, abstände, &[])
    }

    /// Validate the distances against the Parcours' Pflock list: every key must be a
    /// Pflock of this Parcours, and the configured metres must strictly decrease.
    fn mit_pflöcken(
        ziel_id: ZielId,
        abstände: Vec<(PflockId, Abstand)>,
        pflöcke: &[Pflock],
    ) -> Result<Self, ParcoursFehler> {
        if !pflöcke.is_empty() {
            for (pflock_id, _) in &abstände {
                if !pflöcke.iter().any(|pflock| pflock.id == *pflock_id) {
                    return Err(ParcoursFehler::PflockNichtImParcours);
                }
            }
            // Far to near: strictly decreasing in Pflock order. Only configured
            // distances participate, because a missing one has no estimate.
            let reihe: Vec<u32> = pflöcke
                .iter()
                .filter_map(|pflock| {
                    abstände
                        .iter()
                        .find(|(id, _)| *id == pflock.id)
                        .map(|(_, abstand)| abstand.meter())
                })
                .collect();
            if reihe.windows(2).any(|paar| paar[0] <= paar[1]) {
                return Err(ParcoursFehler::AbständeNichtAbnehmend);
            }
        }
        Ok(Self { ziel_id, abstände })
    }

    pub fn ziel_id(&self) -> ZielId {
        self.ziel_id
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

    /// Replace the distance map, re-validating against `pflöcke`.
    fn setze_abstände(
        &mut self,
        abstände: Vec<(PflockId, Abstand)>,
        pflöcke: &[Pflock],
    ) -> Result<(), ParcoursFehler> {
        let neu = Self::mit_pflöcken(self.ziel_id, abstände, pflöcke)?;
        self.abstände = neu.abstände;
        Ok(())
    }
}

/// Creation input of a Parcours: the Pflock setup plus the ordered Ziel list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuerParcours {
    name: ParcoursName,
    pflöcke: Vec<Pflock>,
    platzierungen: Vec<Zielplatzierung>,
}

impl NeuerParcours {
    /// Validate the invariants: unique Pflock labels, at least one Pflock, no duplicate
    /// Ziel placement, and distances that decrease far to near (AC-4, AC-6).
    pub fn neu(
        name: ParcoursName,
        pflöcke: Vec<Pflock>,
        platzierungen: Vec<Zielplatzierung>,
    ) -> Result<Self, ParcoursFehler> {
        if pflöcke.is_empty() {
            return Err(ParcoursFehler::KeinePflöcke);
        }
        for (index, pflock) in pflöcke.iter().enumerate() {
            if pflöcke[..index]
                .iter()
                .any(|früher| früher.bezeichnung == pflock.bezeichnung)
            {
                return Err(ParcoursFehler::PflockBezeichnungDoppelt(
                    pflock.bezeichnung.0.clone(),
                ));
            }
        }
        Self::pruefe_platzierungen(&pflöcke, &platzierungen)?;
        Ok(Self {
            name,
            pflöcke,
            platzierungen,
        })
    }

    fn pruefe_platzierungen(
        pflöcke: &[Pflock],
        platzierungen: &[Zielplatzierung],
    ) -> Result<(), ParcoursFehler> {
        for (index, platzierung) in platzierungen.iter().enumerate() {
            if platzierungen[..index]
                .iter()
                .any(|früher| früher.ziel_id == platzierung.ziel_id)
            {
                return Err(ParcoursFehler::ZielDoppelt);
            }
            // Re-validate: a caller may hand in a placement built with `neu` that never
            // saw the Pflock list.
            Zielplatzierung::mit_pflöcken(
                platzierung.ziel_id,
                platzierung.abstände.clone(),
                pflöcke,
            )?;
        }
        Ok(())
    }

    pub fn name(&self) -> &ParcoursName {
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
pub struct Parcours {
    id: ParcoursId,
    name: ParcoursName,
    /// In far-to-near order. Order is never inferred from a colour.
    pflöcke: Vec<Pflock>,
    /// In route order.
    platzierungen: Vec<Zielplatzierung>,
    zustand: Zustand,
}

impl Parcours {
    pub fn neu(id: ParcoursId, neu: NeuerParcours) -> Self {
        Self {
            id,
            name: neu.name,
            pflöcke: neu.pflöcke,
            platzierungen: neu.platzierungen,
            zustand: Zustand::Aktiv,
        }
    }

    /// Rebuild a persisted Parcours.
    pub fn from_parts(
        id: ParcoursId,
        name: ParcoursName,
        pflöcke: Vec<Pflock>,
        platzierungen: Vec<Zielplatzierung>,
        zustand: Zustand,
    ) -> Result<Self, ParcoursFehler> {
        let neu = NeuerParcours::neu(name, pflöcke.clone(), platzierungen.clone())?;
        Ok(Self {
            id,
            name: neu.name,
            pflöcke,
            platzierungen,
            zustand,
        })
    }

    pub fn id(&self) -> ParcoursId {
        self.id
    }

    pub fn name(&self) -> &ParcoursName {
        &self.name
    }

    pub fn pflöcke(&self) -> &[Pflock] {
        &self.pflöcke
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

    fn nur_aktiv(&self) -> Result<(), ParcoursFehler> {
        match self.zustand {
            Zustand::Aktiv => Ok(()),
            Zustand::Archiviert => Err(ParcoursFehler::Archiviert),
        }
    }

    /// Rename the Parcours. Refused while archived.
    pub fn umbenennen(&mut self, name: ParcoursName) -> Result<(), ParcoursFehler> {
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
    ) -> Result<(), ParcoursFehler> {
        self.nur_aktiv()?;
        let geprüft = NeuerParcours::neu(self.name.clone(), pflöcke, platzierungen)?;
        self.pflöcke = geprüft.pflöcke;
        self.platzierungen = geprüft.platzierungen;
        Ok(())
    }

    /// Add one Ziel placement at the end of the route. A Ziel already placed is refused.
    pub fn platzierung_hinzufügen(
        &mut self,
        platzierung: Zielplatzierung,
    ) -> Result<(), ParcoursFehler> {
        self.nur_aktiv()?;
        if self
            .platzierungen
            .iter()
            .any(|vorhanden| vorhanden.ziel_id == platzierung.ziel_id)
        {
            return Err(ParcoursFehler::ZielDoppelt);
        }
        let geprüft = Zielplatzierung::mit_pflöcken(
            platzierung.ziel_id,
            platzierung.abstände,
            &self.pflöcke,
        )?;
        self.platzierungen.push(geprüft);
        Ok(())
    }

    /// Remove one Ziel from the current arrangement; its history is untouched (AC-11).
    pub fn platzierung_entfernen(&mut self, ziel_id: ZielId) -> Result<(), ParcoursFehler> {
        self.nur_aktiv()?;
        self.platzierungen
            .retain(|platzierung| platzierung.ziel_id != ziel_id);
        Ok(())
    }

    /// Reorder the route. Every listed id must already be placed, and the result must
    /// still cover the same set.
    pub fn reihenfolge_ändern(&mut self, reihenfolge: &[ZielId]) -> Result<(), ParcoursFehler> {
        self.nur_aktiv()?;
        if reihenfolge.len() != self.platzierungen.len() {
            return Err(ParcoursFehler::ReihenfolgeUngültig);
        }
        let mut neu: Vec<Zielplatzierung> = Vec::with_capacity(reihenfolge.len());
        for ziel_id in reihenfolge {
            let Some(platzierung) = self
                .platzierungen
                .iter()
                .find(|platzierung| platzierung.ziel_id == *ziel_id)
            else {
                return Err(ParcoursFehler::ReihenfolgeUngültig);
            };
            if neu.iter().any(|vorhanden| vorhanden.ziel_id == *ziel_id) {
                return Err(ParcoursFehler::ReihenfolgeUngültig);
            }
            neu.push(platzierung.clone());
        }
        self.platzierungen = neu;
        Ok(())
    }

    /// Swap the physical Ziel of one placement in place, keeping its route position and its
    /// distances (AC-5: a replaced target is a new identity at the same slot).
    pub fn platzierung_ersetzen(&mut self, alt: ZielId, neu: ZielId) -> Result<(), ParcoursFehler> {
        self.nur_aktiv()?;
        if self
            .platzierungen
            .iter()
            .any(|platzierung| platzierung.ziel_id == neu)
        {
            return Err(ParcoursFehler::ZielDoppelt);
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

    /// Set one placement's distance map, re-validated against this Parcours' Pflöcke.
    pub fn abstände_setzen(
        &mut self,
        ziel_id: ZielId,
        abstände: Vec<(PflockId, Abstand)>,
    ) -> Result<(), ParcoursFehler> {
        self.nur_aktiv()?;
        let pflöcke = &self.pflöcke;
        let Some(platzierung) = self
            .platzierungen
            .iter_mut()
            .find(|platzierung| platzierung.ziel_id == ziel_id)
        else {
            return Err(ParcoursFehler::ZielFremdeAnlage);
        };
        platzierung.setze_abstände(abstände, pflöcke)
    }

    /// Archive the Parcours: it leaves active selection and keeps its history.
    pub fn archiviere(&mut self) -> Result<(), ParcoursFehler> {
        self.nur_aktiv()?;
        self.zustand = Zustand::Archiviert;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::anlage::ziel::Zustand;

    fn pflock(id: u128, name: &str) -> Pflock {
        Pflock::neu(
            PflockId(Uuid::from_u128(id)),
            PflockBezeichnung::neu(name).unwrap(),
            PflockFarbe::neu("Rot").unwrap(),
        )
    }

    fn drei_pflöcke() -> Vec<Pflock> {
        vec![pflock(1, "Weit"), pflock(2, "Mittel"), pflock(3, "Nah")]
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

    fn parcours() -> Parcours {
        Parcours::neu(
            ParcoursId(Uuid::nil()),
            NeuerParcours::neu(
                ParcoursName::neu("Waldrunde").unwrap(),
                drei_pflöcke(),
                vec![
                    platzierung(ziel(10), &[(1, 40), (2, 30), (3, 20)]),
                    platzierung(ziel(11), &[]),
                ],
            )
            .unwrap(),
        )
    }

    #[test_log::test]
    fn ac6_pflock_bezeichnungen_sind_eindeutig() {
        let doppelt = vec![pflock(1, "Weit"), pflock(2, "Weit")];
        let ergebnis =
            NeuerParcours::neu(ParcoursName::neu("Waldrunde").unwrap(), doppelt, Vec::new());
        assert!(matches!(
            ergebnis,
            Err(ParcoursFehler::PflockBezeichnungDoppelt(_))
        ));
    }

    #[test_log::test]
    fn ac6_abstaende_nehmen_von_fern_nach_nah_streng_ab() {
        let gleich = NeuerParcours::neu(
            ParcoursName::neu("Waldrunde").unwrap(),
            drei_pflöcke(),
            vec![platzierung(ziel(10), &[(1, 30), (2, 30)])],
        );
        assert!(matches!(
            gleich,
            Err(ParcoursFehler::AbständeNichtAbnehmend)
        ));

        let steigend = NeuerParcours::neu(
            ParcoursName::neu("Waldrunde").unwrap(),
            drei_pflöcke(),
            vec![platzierung(ziel(10), &[(1, 20), (2, 30)])],
        );
        assert!(matches!(
            steigend,
            Err(ParcoursFehler::AbständeNichtAbnehmend)
        ));
    }

    #[test_log::test]
    fn ac6_fehlende_abstaende_sind_erlaubt() {
        let ergebnis = NeuerParcours::neu(
            ParcoursName::neu("Waldrunde").unwrap(),
            drei_pflöcke(),
            vec![platzierung(ziel(10), &[(1, 40), (3, 20)])],
        );
        assert!(ergebnis.is_ok());
    }

    #[test_log::test]
    fn ac6_abstand_muss_positiv_sein() {
        assert!(matches!(
            Abstand::neu(0),
            Err(ParcoursFehler::AbstandNichtPositiv(0))
        ));
    }

    #[test_log::test]
    fn ac4_dasselbe_ziel_darf_nur_einmal_vorkommen() {
        let ergebnis = NeuerParcours::neu(
            ParcoursName::neu("Waldrunde").unwrap(),
            drei_pflöcke(),
            vec![
                platzierung(ziel(10), &[]),
                platzierung(ziel(10), &[(1, 40)]),
            ],
        );
        assert!(matches!(ergebnis, Err(ParcoursFehler::ZielDoppelt)));
    }

    #[test_log::test]
    fn ein_parcours_braucht_mindestens_einen_pflock() {
        let ergebnis = NeuerParcours::neu(
            ParcoursName::neu("Waldrunde").unwrap(),
            Vec::new(),
            Vec::new(),
        );
        assert!(matches!(ergebnis, Err(ParcoursFehler::KeinePflöcke)));
    }

    #[test_log::test]
    fn ac6_reihenfolge_ist_unabhaengig_von_der_farbe() {
        // Same colour on every Pflock: order still comes from the list.
        let pflöcke: Vec<_> = drei_pflöcke()
            .into_iter()
            .map(|pflock| {
                Pflock::neu(
                    pflock.id(),
                    pflock.bezeichnung().clone(),
                    PflockFarbe::neu("Rot").unwrap(),
                )
            })
            .collect();
        let parcours = Parcours::neu(
            ParcoursId(Uuid::nil()),
            NeuerParcours::neu(ParcoursName::neu("Waldrunde").unwrap(), pflöcke, Vec::new())
                .unwrap(),
        );
        let namen: Vec<_> = parcours
            .pflöcke()
            .iter()
            .map(|pflock| pflock.bezeichnung().as_str())
            .collect();
        assert_eq!(namen, ["Weit", "Mittel", "Nah"]);
    }

    #[test_log::test]
    fn reihenfolge_aendern_haelt_die_zielmenge() {
        let mut parcours = parcours();
        parcours.reihenfolge_ändern(&[ziel(11), ziel(10)]).unwrap();
        let ids: Vec<_> = parcours
            .platzierungen()
            .iter()
            .map(|platzierung| platzierung.ziel_id())
            .collect();
        assert_eq!(ids, [ziel(11), ziel(10)]);

        assert!(parcours.reihenfolge_ändern(&[ziel(10)]).is_err());
        assert!(parcours.reihenfolge_ändern(&[ziel(10), ziel(10)]).is_err());
    }

    #[test_log::test]
    fn abstände_setzen_prueft_die_pflockliste() {
        let mut parcours = parcours();
        assert!(matches!(
            parcours.abstände_setzen(
                ziel(10),
                vec![(PflockId(Uuid::from_u128(99)), Abstand::neu(10).unwrap())]
            ),
            Err(ParcoursFehler::PflockNichtImParcours)
        ));
    }

    #[test_log::test]
    fn ac11_archivierter_parcours_verweigert_jede_aenderung() {
        let mut parcours = parcours();
        parcours.archiviere().unwrap();
        assert!(parcours.ist_archiviert());
        assert!(matches!(
            parcours.umbenennen(ParcoursName::neu("Neu").unwrap()),
            Err(ParcoursFehler::Archiviert)
        ));
        assert!(matches!(
            parcours.archiviere(),
            Err(ParcoursFehler::Archiviert)
        ));
        assert!(matches!(
            parcours.platzierung_entfernen(ziel(10)),
            Err(ParcoursFehler::Archiviert)
        ));
    }

    #[test_log::test]
    fn ein_ziel_hat_in_jedem_parcours_eigene_abstaende() {
        let a = parcours();
        let b = Parcours::neu(
            ParcoursId(Uuid::from_u128(2)),
            NeuerParcours::neu(
                ParcoursName::neu("Feldrunde").unwrap(),
                drei_pflöcke(),
                vec![platzierung(ziel(10), &[(1, 55), (2, 45), (3, 35)])],
            )
            .unwrap(),
        );
        assert_eq!(
            a.platzierungen()[0].abstand_für(PflockId(Uuid::from_u128(1))),
            Some(Abstand::neu(40).unwrap())
        );
        assert_eq!(
            b.platzierungen()[0].abstand_für(PflockId(Uuid::from_u128(1))),
            Some(Abstand::neu(55).unwrap())
        );
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let original = parcours();
        let geladen = Parcours::from_parts(
            original.id(),
            original.name().clone(),
            original.pflöcke().to_vec(),
            original.platzierungen().to_vec(),
            Zustand::Aktiv,
        )
        .unwrap();
        assert_eq!(original, geladen);
    }
}
