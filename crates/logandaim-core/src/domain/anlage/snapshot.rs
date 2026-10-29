//! The immutable shooting setup a `Runde` captures at start.
//!
//! A snapshot is a value copy, not a reference into the `Anlage`: archiving a Ziel or
//! editing a Parcours may not change a past Runde (SPEC-stammdaten rules 7–8, AC-7/AC-9).
//! It is the only value object that crosses the aggregate boundary.

use uuid::Uuid;

use super::parcours::{ParcoursId, PflockId};
use super::ziel::{Tiername, ZielId, Zielgruppe};

/// A persisted Pflock row: label, colour, position.
type PflockRoh = (String, String, usize);
/// A persisted Ziel row: identity, animal name, group, label-to-metre distances.
type ZielRoh = (Uuid, String, u8, Vec<(String, u32)>);
/// A child Pflock before capture: identity, label, colour.
type QuellePflock = (PflockId, String, String);
/// A child Ziel before capture: identity, animal name, group, per-Pflock distances.
type QuelleZiel = (ZielId, Tiername, Zielgruppe, Vec<(PflockId, u32)>);

/// One Pflock as captured: label, colour, and its position in the far-to-near order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotPflock {
    bezeichnung: String,
    farbe: String,
    /// 0-based position in the captured far-to-near order.
    position: usize,
}

impl SnapshotPflock {
    pub fn neu(
        bezeichnung: String,
        farbe: String,
        position: usize,
    ) -> Result<Self, SnapshotFehler> {
        if bezeichnung.trim().is_empty() {
            return Err(SnapshotFehler::PflockBezeichnungLeer);
        }
        if farbe.trim().is_empty() {
            return Err(SnapshotFehler::PflockFarbeLeer);
        }
        Ok(Self {
            bezeichnung,
            farbe,
            position,
        })
    }

    pub fn bezeichnung(&self) -> &str {
        &self.bezeichnung
    }

    pub fn farbe(&self) -> &str {
        &self.farbe
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

/// One Ziel as captured: identity, animal name, group, and per-Pflock distances.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotZiel {
    ziel_id: ZielId,
    tiername: String,
    zielgruppe: u8,
    /// Label to metres, for the distances that were configured.
    abstände: Vec<(String, u32)>,
}

impl SnapshotZiel {
    pub fn neu(
        ziel_id: ZielId,
        tiername: String,
        zielgruppe: u8,
        abstände: Vec<(String, u32)>,
    ) -> Result<Self, SnapshotFehler> {
        if tiername.trim().is_empty() {
            return Err(SnapshotFehler::TiernameLeer);
        }
        if !(1..=4).contains(&zielgruppe) {
            return Err(SnapshotFehler::ZielgruppeUngültig(zielgruppe));
        }
        Ok(Self {
            ziel_id,
            tiername,
            zielgruppe,
            abstände,
        })
    }

    pub fn ziel_id(&self) -> ZielId {
        self.ziel_id
    }

    pub fn tiername(&self) -> &str {
        &self.tiername
    }

    pub fn zielgruppe(&self) -> u8 {
        self.zielgruppe
    }

    pub fn abstände(&self) -> &[(String, u32)] {
        &self.abstände
    }

    /// The captured distance for one Pflock label, if one was configured.
    pub fn abstand_für(&self, bezeichnung: &str) -> Option<u32> {
        self.abstände
            .iter()
            .find(|(label, _)| label == bezeichnung)
            .map(|(_, meter)| *meter)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SnapshotFehler {
    #[error("der Parcoursname darf nicht leer sein")]
    ParcoursNameLeer,
    #[error("die Pflock-Bezeichnung darf nicht leer sein")]
    PflockBezeichnungLeer,
    #[error("die Pflock-Farbe darf nicht leer sein")]
    PflockFarbeLeer,
    #[error("der Tiername darf nicht leer sein")]
    TiernameLeer,
    #[error("die Zielgruppe muss zwischen 1 und 4 liegen, war aber {0}")]
    ZielgruppeUngültig(u8),
}

/// The complete shooting setup of one Parcours, frozen at the instant a Runde started.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParcoursSnapshot {
    parcours_id: ParcoursId,
    parcours_name: String,
    /// In far-to-near order, positions assigned from the sequence.
    pflöcke: Vec<SnapshotPflock>,
    /// In route order.
    ziele: Vec<SnapshotZiel>,
}

impl ParcoursSnapshot {
    /// Capture the snapshot from the pieces it is made of. The caller (the `Anlage`
    /// aggregate) supplies the current values; this constructor only validates.
    pub fn neu(
        parcours_id: ParcoursId,
        parcours_name: impl AsRef<str>,
        pflöcke: Vec<SnapshotPflock>,
        ziele: Vec<SnapshotZiel>,
    ) -> Result<Self, SnapshotFehler> {
        let parcours_name = parcours_name.as_ref().trim();
        if parcours_name.is_empty() {
            return Err(SnapshotFehler::ParcoursNameLeer);
        }
        Ok(Self {
            parcours_id,
            parcours_name: parcours_name.to_owned(),
            pflöcke,
            ziele,
        })
    }

    /// Rebuild a persisted snapshot; the same shape `neu` takes, so a stored row obeys
    /// the same invariants as a captured one.
    pub fn from_parts(
        parcours_id: Uuid,
        parcours_name: String,
        pflöcke: Vec<PflockRoh>,
        ziele: Vec<ZielRoh>,
    ) -> Result<Self, SnapshotFehler> {
        let pflöcke = pflöcke
            .into_iter()
            .map(|(bezeichnung, farbe, position)| SnapshotPflock::neu(bezeichnung, farbe, position))
            .collect::<Result<Vec<_>, _>>()?;
        let ziele = ziele
            .into_iter()
            .map(|(ziel_id, tiername, zielgruppe, abstände)| {
                SnapshotZiel::neu(ZielId(ziel_id), tiername, zielgruppe, abstände)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Self::neu(ParcoursId(parcours_id), parcours_name, pflöcke, ziele)
    }

    pub fn parcours_id(&self) -> ParcoursId {
        self.parcours_id
    }

    pub fn parcours_name(&self) -> &str {
        &self.parcours_name
    }

    pub fn pflöcke(&self) -> &[SnapshotPflock] {
        &self.pflöcke
    }

    pub fn ziele(&self) -> &[SnapshotZiel] {
        &self.ziele
    }

    /// The captured Ziel with this identity, if it was part of the route.
    pub fn ziel(&self, ziel_id: ZielId) -> Option<&SnapshotZiel> {
        self.ziele.iter().find(|ziel| ziel.ziel_id == ziel_id)
    }

    /// Whether this identity was part of the captured route.
    pub fn enthält(&self, ziel_id: ZielId) -> bool {
        self.ziel(ziel_id).is_some()
    }
}

/// What capturing one Parcours from an `Anlage` needs: the child's current values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotQuelle {
    parcours_id: ParcoursId,
    parcours_name: String,
    pflöcke: Vec<QuellePflock>,
    ziele: Vec<QuelleZiel>,
}

impl SnapshotQuelle {
    pub fn neu(
        parcours_id: ParcoursId,
        parcours_name: impl Into<String>,
        pflöcke: Vec<QuellePflock>,
        ziele: Vec<QuelleZiel>,
    ) -> Self {
        Self {
            parcours_id,
            parcours_name: parcours_name.into(),
            pflöcke,
            ziele,
        }
    }

    /// Turn the source into a snapshot, resolving Pflock ids to labels so the snapshot
    /// does not depend on any row still existing.
    pub fn in_snapshot(&self) -> Result<ParcoursSnapshot, SnapshotFehler> {
        let pflöcke = self
            .pflöcke
            .iter()
            .enumerate()
            .map(|(position, (_, bezeichnung, farbe))| {
                SnapshotPflock::neu(bezeichnung.clone(), farbe.clone(), position)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let ziele = self
            .ziele
            .iter()
            .map(|(ziel_id, tiername, zielgruppe, abstände)| {
                let abstände = abstände
                    .iter()
                    .map(|(pflock_id, meter)| {
                        let label = self
                            .pflöcke
                            .iter()
                            .find(|(id, _, _)| id == pflock_id)
                            .map(|(_, bezeichnung, _)| bezeichnung.clone())
                            .ok_or(SnapshotFehler::PflockBezeichnungLeer)?;
                        Ok((label, *meter))
                    })
                    .collect::<Result<Vec<_>, SnapshotFehler>>()?;
                SnapshotZiel::neu(
                    *ziel_id,
                    tiername.as_str().to_owned(),
                    zielgruppe.value(),
                    abstände,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        ParcoursSnapshot::neu(self.parcours_id, &self.parcours_name, pflöcke, ziele)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pflock(id: u128, label: &str, farbe: &str) -> (PflockId, String, String) {
        (
            PflockId(Uuid::from_u128(id)),
            label.to_owned(),
            farbe.to_owned(),
        )
    }

    fn quelle() -> SnapshotQuelle {
        SnapshotQuelle::neu(
            ParcoursId(Uuid::from_u128(7)),
            "Waldrunde",
            vec![pflock(1, "Weit", "Rot"), pflock(2, "Nah", "Blau")],
            vec![(
                ZielId(Uuid::from_u128(10)),
                Tiername::neu("Keiler").unwrap(),
                Zielgruppe::neu(2).unwrap(),
                vec![(PflockId(Uuid::from_u128(1)), 40)],
            )],
        )
    }

    #[test_log::test]
    fn snapshot_traegt_namen_ziele_und_abstaende() {
        let snapshot = quelle().in_snapshot().unwrap();
        assert_eq!(snapshot.parcours_name(), "Waldrunde");
        assert_eq!(snapshot.pflöcke().len(), 2);
        assert_eq!(snapshot.pflöcke()[0].position(), 0);
        assert_eq!(snapshot.pflöcke()[1].position(), 1);
        let ziel = snapshot.ziele()[0].clone();
        assert_eq!(ziel.tiername(), "Keiler");
        assert_eq!(ziel.zielgruppe(), 2);
        assert_eq!(ziel.abstand_für("Weit"), Some(40));
        assert_eq!(ziel.abstand_für("Nah"), None);
    }

    #[test_log::test]
    fn snapshot_enthält_nur_platzierte_ziele() {
        let snapshot = quelle().in_snapshot().unwrap();
        assert!(snapshot.enthält(ZielId(Uuid::from_u128(10))));
        assert!(!snapshot.enthält(ZielId(Uuid::from_u128(11))));
    }

    #[test_log::test]
    fn leerer_name_wird_abgelehnt() {
        let ergebnis = ParcoursSnapshot::neu(ParcoursId(Uuid::nil()), "  ", Vec::new(), Vec::new());
        assert!(matches!(ergebnis, Err(SnapshotFehler::ParcoursNameLeer)));
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let original = quelle().in_snapshot().unwrap();
        let geladen = ParcoursSnapshot::from_parts(
            original.parcours_id().0,
            original.parcours_name().to_owned(),
            original
                .pflöcke()
                .iter()
                .map(|pflock| {
                    (
                        pflock.bezeichnung().to_owned(),
                        pflock.farbe().to_owned(),
                        pflock.position(),
                    )
                })
                .collect(),
            original
                .ziele()
                .iter()
                .map(|ziel| {
                    (
                        ziel.ziel_id().0,
                        ziel.tiername().to_owned(),
                        ziel.zielgruppe(),
                        ziel.abstände().to_vec(),
                    )
                })
                .collect(),
        )
        .unwrap();
        assert_eq!(original, geladen);
    }
}
