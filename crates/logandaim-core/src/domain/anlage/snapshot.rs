//! The immutable shooting setup a `Runde` captures at start.
//!
//! A snapshot is a value copy, not a reference into the `Anlage`: archiving a Ziel or
//! editing a Parkour may not change a past Runde (SPEC-stammdaten rules 7–8, AC-7/AC-9).
//! It is the only value object that crosses the aggregate boundary.

use std::num::NonZeroU32;

use super::Anlage;
use super::parkour::{Parkour, ParkourId, PflockId};
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

/// One Ziel as captured: identity, ordinal, animal name, and per-Pflock distances.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZielSnapshot {
    ziel_id: ZielId,
    zielnummer: NonZeroU32,
    tiername: NonEmptyString,
    /// Pflock colour to metres, for the distances that were configured.
    abstände: Vec<(Farbe, Abstand)>,
}

impl ZielSnapshot {
    pub fn neu(
        ziel_id: ZielId,
        zielnummer: NonZeroU32,
        tiername: NonEmptyString,
        abstände: Vec<(Farbe, Abstand)>,
    ) -> Self {
        Self {
            ziel_id,
            zielnummer,
            tiername,
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

    pub fn abstände(&self) -> &[(Farbe, Abstand)] {
        &self.abstände
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
    pub fn enthält(&self, ziel_id: ZielId) -> bool {
        self.ziel(ziel_id).is_some()
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
                    let farbe = pflock_farbe(parkour, *pflock_id)
                        .ok_or(SnapshotFehler::ZielNichtGefunden)?;
                    Ok((farbe, *abstand))
                })
                .collect::<Result<Vec<_>, SnapshotFehler>>()?;
            Ok(ZielSnapshot::neu(
                ziel.id(),
                ziel.zielnummer(),
                ziel.tiername().clone(),
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

fn pflock_farbe(parkour: &super::parkour::Parkour, pflock_id: PflockId) -> Option<Farbe> {
    parkour
        .pflöcke()
        .iter()
        .find(|pflock| pflock.id() == pflock_id)
        .map(|pflock| pflock.farbe())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Ländercode;
    use crate::domain::anlage::{
        AnlageAdresse, AnlageAktiv, AnlageId, AnlageKontakt, AnlageName, NeueAnlage, NeuerParkour,
        Pflock, Zielplatzierung, Zustand,
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
                AnlageName::neu("Breitenstein").unwrap(),
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
        assert!(snapshot.enthält(ziel_id(10)));
        assert!(!snapshot.enthält(ziel_id(11)));
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
}
