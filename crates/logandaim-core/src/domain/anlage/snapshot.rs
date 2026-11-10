//! The immutable shooting setup a `Runde` captures at start.
//!
//! A snapshot is a value copy, not a reference into the `Anlage`: archiving a Ziel or
//! editing a Parkour may not change a past Runde (SPEC-stammdaten rules 7–8, AC-7/AC-9).
//! It is the only value object that crosses the aggregate boundary.

use std::num::NonZeroU32;

use super::Anlage;
use super::parkour::{Parkour, ParkourId, ZielPflöcke};
use super::ziel::{Ziel, ZielId};
use crate::domain::common::{Abstand, Farbe, NonEmptyString};

/// One Pflock as captured: its colour and its position in the near-to-far order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PflockSnapshot {
    farbe: Farbe,
    /// 0-based position in the captured near-to-far order.
    position: usize,
}

impl PflockSnapshot {
    pub fn neu(farbe: Farbe, position: usize) -> Self {
        Self { farbe, position }
    }

    pub fn farbe(&self) -> Farbe {
        self.farbe
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

/// The immutable mirror of [`ZielPflöcke`]: which Pflock order a Ziel was captured with.
///
/// Positions are assigned from the array order, so a snapshot never carries a position and
/// an array index that disagree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZielPflöckeSnapshot {
    /// The Parkour-wide captured order applies to this Ziel.
    InheritFromParkour,
    /// This Ziel's own captured order, near to far.
    Explicit(Vec<PflockSnapshot>),
}

impl ZielPflöckeSnapshot {
    /// The Ziel's own captured Pflöcke, `None` when it inherits the Parkour-wide order.
    pub fn eigene(&self) -> Option<&[PflockSnapshot]> {
        match self {
            Self::InheritFromParkour => None,
            Self::Explicit(eigene) => Some(eigene),
        }
    }

    /// The captured Pflöcke that actually apply to this Ziel.
    pub fn effektive<'a>(&'a self, parkour: &'a [PflockSnapshot]) -> &'a [PflockSnapshot] {
        match self {
            Self::InheritFromParkour => parkour,
            Self::Explicit(eigene) => eigene,
        }
    }

    /// Capture the domain enum, assigning each own Pflock its array position.
    fn aus_domain(pflöcke: &ZielPflöcke) -> Self {
        match pflöcke.eigene() {
            None => Self::InheritFromParkour,
            Some(eigene) => Self::Explicit(
                eigene
                    .iter()
                    .enumerate()
                    .map(|(position, pflock)| PflockSnapshot::neu(pflock.farbe(), position))
                    .collect(),
            ),
        }
    }
}

/// One Ziel as captured: identity, ordinal, animal name, its Pflock order, and per-Pflock
/// distances.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZielSnapshot {
    ziel_id: ZielId,
    zielnummer: NonZeroU32,
    tiername: NonEmptyString,
    /// Which captured Pflock order applies to this Ziel.
    pflöcke: ZielPflöckeSnapshot,
    /// Pflock colour to metres, for the distances that were configured.
    abstände: Vec<(Farbe, Abstand)>,
}

impl ZielSnapshot {
    pub fn neu(
        ziel_id: ZielId,
        zielnummer: NonZeroU32,
        tiername: NonEmptyString,
        pflöcke: ZielPflöckeSnapshot,
        abstände: Vec<(Farbe, Abstand)>,
    ) -> Self {
        Self {
            ziel_id,
            zielnummer,
            tiername,
            pflöcke,
            abstände,
        }
    }

    pub fn ziel_id(&self) -> ZielId {
        self.ziel_id
    }

    /// The 1-based ordinal that tells duplicate animal names apart.
    pub fn zielnummer(&self) -> NonZeroU32 {
        self.zielnummer
    }

    pub fn tiername(&self) -> &NonEmptyString {
        &self.tiername
    }

    /// Whether this Ziel inherits the Parkour-wide captured order or carries its own.
    pub fn pflöcke(&self) -> &ZielPflöckeSnapshot {
        &self.pflöcke
    }

    pub fn abstände(&self) -> &[(Farbe, Abstand)] {
        &self.abstände
    }

    /// Whether this Ziel explicitly carries a captured Pflock of `farbe`.
    pub fn hat_explizit_pflock(&self, farbe: Farbe) -> bool {
        self.pflöcke
            .eigene()
            .is_some_and(|eigene| eigene.iter().any(|pflock| pflock.farbe() == farbe))
    }

    /// The captured distance for one Pflock colour, if one was configured.
    pub fn abstand_für(&self, pflock_farbe: Farbe) -> Option<Abstand> {
        self.abstände
            .iter()
            .find(|(farbe, _)| *farbe == pflock_farbe)
            .map(|(_, abstand)| *abstand)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SnapshotFehler {
    #[error("der Parkour wurde nicht gefunden")]
    ParkourNichtGefunden,
    #[error("ein platziertes Ziel wurde nicht gefunden")]
    ZielNichtGefunden,
}

/// The complete shooting setup of one Parkour, frozen at the instant a Runde started.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParkourSnapshot {
    parkour_id: ParkourId,
    parkour_name: NonEmptyString,
    /// Near to far, positions assigned from the sequence.
    pflöcke: Vec<PflockSnapshot>,
    /// In route order.
    ziele: Vec<ZielSnapshot>,
}

impl ParkourSnapshot {
    /// Rebuild a stored snapshot or capture a fresh one: every field is already validated,
    /// so there is nothing left to refuse.
    pub fn from_parts(
        parkour_id: ParkourId,
        parkour_name: NonEmptyString,
        pflöcke: Vec<PflockSnapshot>,
        ziele: Vec<ZielSnapshot>,
    ) -> Self {
        Self {
            parkour_id,
            parkour_name,
            pflöcke,
            ziele,
        }
    }

    pub fn parkour_id(&self) -> ParkourId {
        self.parkour_id
    }

    pub fn parkour_name(&self) -> &NonEmptyString {
        &self.parkour_name
    }

    pub fn pflöcke(&self) -> &[PflockSnapshot] {
        &self.pflöcke
    }

    pub fn ziele(&self) -> &[ZielSnapshot] {
        &self.ziele
    }

    /// The captured Ziel with this identity, if it was part of the route.
    pub fn ziel(&self, ziel_id: ZielId) -> Option<&ZielSnapshot> {
        self.ziele.iter().find(|ziel| ziel.ziel_id == ziel_id)
    }

    /// Whether this identity was part of the captured route.
    pub fn enthält_ziel(&self, ziel_id: ZielId) -> bool {
        self.ziel(ziel_id).is_some()
    }

    pub fn hat_pflock(&self, farbe: Farbe) -> bool {
        self.pflöcke.iter().any(|pflock| pflock.farbe() == farbe)
    }

    /// Whether `farbe` is selectable at `ziel_id`: the Ziel's own captured set when it has
    /// one, the Parkour-wide captured set otherwise.
    ///
    /// An unknown `ziel_id` is never a match. A colour that exists only in the Parkour-wide
    /// set does not select on a Ziel whose own set replaces it.
    pub fn hat_pflock_am_ziel(&self, ziel_id: ZielId, farbe: Farbe) -> bool {
        self.ziel(ziel_id).is_some_and(|ziel| {
            ziel.pflöcke()
                .effektive(&self.pflöcke)
                .iter()
                .any(|pflock| pflock.farbe() == farbe)
        })
    }
}

/// Capture one Parkour's current setup, resolving every Pflock id to its colour so the
/// snapshot no longer depends on any master-data row.
pub fn snapshot(anlage: &Anlage, parkour_id: ParkourId) -> Result<ParkourSnapshot, SnapshotFehler> {
    let parkour = anlage
        .parkour_mit_id(parkour_id)
        .ok_or(SnapshotFehler::ParkourNichtGefunden)?;
    snapshot_aus_parkour(parkour, |ziel_id| anlage.ziel(ziel_id))
}

/// The capture rule once the Parkour is known, shared with the active typestate.
pub(crate) fn snapshot_aus_parkour<'a>(
    parkour: &Parkour,
    ziel: impl Fn(ZielId) -> Option<&'a Ziel>,
) -> Result<ParkourSnapshot, SnapshotFehler> {
    let pflöcke = parkour
        .pflöcke()
        .iter()
        .enumerate()
        .map(|(position, pflock)| PflockSnapshot::neu(pflock.farbe(), position))
        .collect::<Vec<_>>();

    let ziele = parkour
        .platzierungen()
        .iter()
        .map(|platzierung| {
            let ziel = ziel(platzierung.ziel_id()).ok_or(SnapshotFehler::ZielNichtGefunden)?;
            let abstände = platzierung
                .abstände()
                .iter()
                .map(|(pflock_id, abstand)| {
                    // A placement's distances are validated against its effective Pflock set,
                    // so an unknown id here is corruption, not a user error.
                    let farbe = platzierung
                        .effektive_pflöcke(parkour.pflöcke())
                        .iter()
                        .find(|pflock| pflock.id() == *pflock_id)
                        .ok_or(SnapshotFehler::ZielNichtGefunden)?
                        .farbe();
                    Ok((farbe, *abstand))
                })
                .collect::<Result<Vec<_>, SnapshotFehler>>()?;
            Ok(ZielSnapshot::neu(
                ziel.id(),
                ziel.zielnummer(),
                ziel.tiername().clone(),
                ZielPflöckeSnapshot::aus_domain(platzierung.pflöcke()),
                abstände,
            ))
        })
        .collect::<Result<Vec<_>, SnapshotFehler>>()?;

    Ok(ParkourSnapshot::from_parts(
        parkour.id(),
        parkour.name().clone(),
        pflöcke,
        ziele,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Ländercode;
    use crate::domain::anlage::{
        AnlageAdresse, AnlageAktiv, AnlageId, AnlageKontakt, NeueAnlage, NeuerParkour, Pflock,
        PflockId, Zielplatzierung, Zustand,
    };
    use uuid::Uuid;

    fn name(wert: &str) -> NonEmptyString {
        NonEmptyString::neu(wert).unwrap()
    }

    fn ziel_id(id: u128) -> ZielId {
        ZielId(Uuid::from_u128(id))
    }

    fn pflock(id: u128, farbe: &str) -> Pflock {
        Pflock::neu(PflockId(Uuid::from_u128(id)), Farbe::neu(farbe).unwrap())
    }

    fn anlage() -> (AnlageAktiv, ParkourId) {
        let mut anlage = AnlageAktiv::neu(
            AnlageId(Uuid::nil()),
            NeueAnlage::neu(
                name("Breitenstein"),
                AnlageAdresse::neu("Waldweg 1", "3100", "St. Pölten", Ländercode::AT).unwrap(),
                AnlageKontakt::leer(),
            ),
        );
        anlage.ziel_hinzufügen(ziel_id(10), name("Keiler"));
        let parkour_id = ParkourId(Uuid::from_u128(7));
        anlage
            .parkour_hinzufügen(
                parkour_id,
                NeuerParkour::neu(
                    name("Waldrunde"),
                    vec![pflock(1, "#c0392b"), pflock(2, "#1b4f9c")],
                    vec![
                        Zielplatzierung::neu(
                            ziel_id(10),
                            vec![
                                (PflockId(Uuid::from_u128(1)), Abstand::neu(20).unwrap()),
                                (PflockId(Uuid::from_u128(2)), Abstand::neu(30).unwrap()),
                            ],
                        )
                        .unwrap(),
                    ],
                )
                .unwrap(),
            )
            .unwrap();
        (anlage, parkour_id)
    }

    #[test_log::test]
    fn snapshot_traegt_namen_ziele_und_abstaende() {
        let (anlage, parkour_id) = anlage();
        let snapshot = snapshot(&anlage.into(), parkour_id).unwrap();
        assert_eq!(snapshot.parkour_name().as_str(), "Waldrunde");
        assert_eq!(snapshot.pflöcke().len(), 2);
        assert_eq!(snapshot.pflöcke()[0].position(), 0);
        assert_eq!(snapshot.pflöcke()[1].position(), 1);

        let ziel = &snapshot.ziele()[0];
        assert_eq!(ziel.tiername().as_str(), "Keiler");
        assert_eq!(ziel.zielnummer().get(), 1);
        assert_eq!(
            ziel.abstand_für(Farbe::neu("#c0392b").unwrap()),
            Some(Abstand::neu(20).unwrap())
        );
        assert_eq!(
            ziel.abstand_für(Farbe::neu("#1b4f9c").unwrap()),
            Some(Abstand::neu(30).unwrap())
        );
    }

    #[test_log::test]
    fn snapshot_enthaelt_nur_platzierte_ziele() {
        let (anlage, parkour_id) = anlage();
        let snapshot = snapshot(&anlage.into(), parkour_id).unwrap();
        assert!(snapshot.enthält_ziel(ziel_id(10)));
        assert!(!snapshot.enthält_ziel(ziel_id(11)));
        assert!(snapshot.hat_pflock(Farbe::neu("#c0392b").unwrap()));
        assert!(!snapshot.hat_pflock(Farbe::neu("#ffffff").unwrap()));
    }

    #[test_log::test]
    fn unbekannter_parkour_wird_abgelehnt() {
        let (anlage, _) = anlage();
        let ergebnis = snapshot(&anlage.into(), ParkourId(Uuid::from_u128(99)));
        assert!(matches!(
            ergebnis,
            Err(SnapshotFehler::ParkourNichtGefunden)
        ));
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let (anlage, parkour_id) = anlage();
        let original = snapshot(&anlage.into(), parkour_id).unwrap();
        let geladen = ParkourSnapshot::from_parts(
            original.parkour_id(),
            original.parkour_name().clone(),
            original.pflöcke().to_vec(),
            original.ziele().to_vec(),
        );
        assert_eq!(original, geladen);
        assert_eq!(geladen.ziele()[0].zielnummer().get(), 1);
        assert_eq!(geladen.pflöcke()[0].position(), 0);
    }

    #[test_log::test]
    fn archivistand_bleibt_im_snapshot_unberuehrt() {
        let (mut anlage, parkour_id) = anlage();
        let snapshot = snapshot(&anlage.clone().into(), parkour_id).unwrap();
        anlage.ziel_archivieren(ziel_id(10)).unwrap();
        assert_eq!(
            snapshot.ziele()[0].tiername().as_str(),
            "Keiler",
            "ein archiviertes Ziel bleibt im gefrorenen Snapshot"
        );
        assert_eq!(snapshot.ziele()[0].zielnummer().get(), 1);
        let _ = Zustand::Aktiv;
    }

    /// An Anlage whose one Ziel carries its own Pflock set, plus a second Ziel that
    /// inherits the Parkour-wide set.
    fn anlage_mit_eigenem_satz() -> (AnlageAktiv, ParkourId) {
        let mut anlage = AnlageAktiv::neu(
            AnlageId(Uuid::nil()),
            NeueAnlage::neu(
                name("Breitenstein"),
                AnlageAdresse::neu("Waldweg 1", "3100", "St. Pölten", Ländercode::AT).unwrap(),
                AnlageKontakt::leer(),
            ),
        );
        anlage.ziel_hinzufügen(ziel_id(10), name("Keiler"));
        anlage.ziel_hinzufügen(ziel_id(11), name("Reh"));
        let parkour_id = ParkourId(Uuid::from_u128(7));
        anlage
            .parkour_hinzufügen(
                parkour_id,
                NeuerParkour::neu(
                    name("Waldrunde"),
                    vec![pflock(1, "#c0392b"), pflock(2, "#1b4f9c")],
                    vec![
                        // Own set: near-to-far, replaces the Parkour-wide order.
                        Zielplatzierung::neu_mit_eigenen_pflöcken(
                            ziel_id(10),
                            vec![pflock(11, "#1b4f9c"), pflock(12, "#e8b81c")],
                            vec![
                                (PflockId(Uuid::from_u128(11)), Abstand::neu(15).unwrap()),
                                (PflockId(Uuid::from_u128(12)), Abstand::neu(45).unwrap()),
                            ],
                        )
                        .unwrap(),
                        Zielplatzierung::neu(ziel_id(11), Vec::new()).unwrap(),
                    ],
                )
                .unwrap(),
            )
            .unwrap();
        (anlage, parkour_id)
    }

    #[test_log::test]
    fn ac6_der_snapshot_traegt_die_zielpfloecke() {
        let (anlage, parkour_id) = anlage_mit_eigenem_satz();
        let snapshot = snapshot(&anlage.into(), parkour_id).unwrap();

        let eigenes = &snapshot.ziele()[0];
        assert!(eigenes.pflöcke().eigene().is_some());
        let eigene = eigenes.pflöcke().eigene().expect("ein eigener Satz");
        assert_eq!(eigene.len(), 2);
        assert_eq!(eigene[0].farbe().to_hex(), "#1b4f9c");
        assert_eq!(eigene[0].position(), 0);
        assert_eq!(eigene[1].position(), 1);
        // Replacing, not merging: the Parkour-wide "#c0392b" is not in this Ziel's set.
        assert!(!eigenes.hat_explizit_pflock(Farbe::neu("#c0392b").unwrap()));

        let erbe = &snapshot.ziele()[1];
        assert!(erbe.pflöcke().eigene().is_none());
        assert!(erbe.pflöcke().eigene().is_none());
        assert_eq!(erbe.pflöcke().effektive(snapshot.pflöcke()).len(), 2);
    }

    #[test_log::test]
    fn ac6_hat_pflock_am_ziel_fragt_die_effektive_menge() {
        let (anlage, parkour_id) = anlage_mit_eigenem_satz();
        let snapshot = snapshot(&anlage.into(), parkour_id).unwrap();
        let eigen_blau = Farbe::neu("#1b4f9c").unwrap();
        let eigen_gelb = Farbe::neu("#e8b81c").unwrap();
        let parkour_rot = Farbe::neu("#c0392b").unwrap();

        // The own-set Ziel sees both colours of its own set, not the Parkour-wide one.
        assert!(snapshot.hat_pflock_am_ziel(ziel_id(10), eigen_blau));
        assert!(!snapshot.hat_pflock_am_ziel(ziel_id(10), parkour_rot));
        // The inheriting Ziel still sees the Parkour-wide set only.
        assert!(snapshot.hat_pflock_am_ziel(ziel_id(11), parkour_rot));
        // ...and a colour that exists only as another Ziel's own Pflock is not selectable
        // there, even though it sits outside every captured Parkour-wide set.
        assert!(!snapshot.hat_pflock_am_ziel(ziel_id(11), eigen_gelb));
        // An unknown Ziel never matches.
        assert!(!snapshot.hat_pflock_am_ziel(ziel_id(99), parkour_rot));
    }

    #[test_log::test]
    fn ac6_der_snapshot_bleibt_von_einer_spaeteren_satzänderung_unberuehrt() {
        let (mut anlage, parkour_id) = anlage_mit_eigenem_satz();
        let snapshot = snapshot(&anlage.clone().into(), parkour_id).unwrap();

        // Swap the Ziel's own set for the inherited one after capture.
        anlage
            .parkour_setup_ändern(
                parkour_id,
                vec![pflock(1, "#c0392b"), pflock(2, "#1b4f9c")],
                vec![
                    Zielplatzierung::neu(ziel_id(10), Vec::new()).unwrap(),
                    Zielplatzierung::neu(ziel_id(11), Vec::new()).unwrap(),
                ],
            )
            .unwrap();

        assert!(snapshot.ziele()[0].pflöcke().eigene().is_some());
        assert_eq!(
            snapshot.ziele()[0].pflöcke().eigene().unwrap().len(),
            2,
            "AC-9: ein späterer Satz ändert den gefrorenen Snapshot nicht"
        );
    }
}
