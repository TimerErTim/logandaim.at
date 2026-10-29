//! One logged Runde at one Parcours, holding its captured snapshot.
//!
//! A `Runde` is a standalone aggregate root. It never holds a collection of `Schuss`:
//! each `Schuss` carries `runde_id`, so two aggregates stay two aggregates and no use case
//! writes both. The Runde-wide Wertung folds the per-Ziel results once, in the order of the
//! captured route (SPEC-wertung rule 6, AC-8).

use chrono::{DateTime, Utc};

use super::anlage::{AnlageId, ParcoursId, ParcoursSnapshot, ZielId};
use super::konto::BenutzerId;
use super::schuss::{Punkte, Schussmenge, Zielergebnis, Zuordnung};

/// Identity of one Runde. Re-exported where a `Schuss` needs it.
pub use super::schuss::RundeId;

/// Lifecycle of a Runde. A plain status enum: closing one changes no behaviour beyond
/// refusing further Schüsse.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RundeZustand {
    InBearbeitung,
    Abgeschlossen,
}

impl RundeZustand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InBearbeitung => "InBearbeitung",
            Self::Abgeschlossen => "Abgeschlossen",
        }
    }

    pub fn ist_in_bearbeitung(self) -> bool {
        matches!(self, Self::InBearbeitung)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("der Zustand der Runde ist unbekannt")]
pub struct RundeZustandUnbekannt;

impl std::str::FromStr for RundeZustand {
    type Err = RundeZustandUnbekannt;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "InBearbeitung" => Ok(Self::InBearbeitung),
            "Abgeschlossen" => Ok(Self::Abgeschlossen),
            _ => Err(RundeZustandUnbekannt),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RundeFehler {
    #[error("die Runde ist bereits abgeschlossen")]
    BereitsAbgeschlossen,
}

/// Creation input of a Runde: everything except its identity and lifecycle state.
///
/// The snapshot is a value copy taken from the `Anlage` in the same transaction, so a
/// later master-data edit cannot reach it (AC-7, AC-9).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeueRunde {
    anlage_id: AnlageId,
    parcours_id: ParcoursId,
    zuordnung: Zuordnung,
    snapshot: ParcoursSnapshot,
    begonnen_am: DateTime<Utc>,
}

impl NeueRunde {
    pub fn neu(
        anlage_id: AnlageId,
        zuordnung: Zuordnung,
        snapshot: ParcoursSnapshot,
        begonnen_am: DateTime<Utc>,
    ) -> Self {
        Self {
            anlage_id,
            parcours_id: snapshot.parcours_id(),
            zuordnung,
            snapshot,
            begonnen_am,
        }
    }

    pub fn anlage_id(&self) -> AnlageId {
        self.anlage_id
    }

    pub fn parcours_id(&self) -> ParcoursId {
        self.parcours_id
    }

    pub fn zuordnung(&self) -> Zuordnung {
        self.zuordnung
    }

    pub fn snapshot(&self) -> &ParcoursSnapshot {
        &self.snapshot
    }

    pub fn begonnen_am(&self) -> DateTime<Utc> {
        self.begonnen_am
    }
}

/// One logged Runde: who and where, the frozen setup, and whether it is still running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runde {
    id: RundeId,
    anlage_id: AnlageId,
    parcours_id: ParcoursId,
    zuordnung: Zuordnung,
    snapshot: ParcoursSnapshot,
    zustand: RundeZustand,
    begonnen_am: DateTime<Utc>,
    abgeschlossen_am: Option<DateTime<Utc>>,
}

impl Runde {
    pub fn neu(id: RundeId, neu: NeueRunde) -> Self {
        Self {
            id,
            anlage_id: neu.anlage_id,
            parcours_id: neu.parcours_id,
            zuordnung: neu.zuordnung,
            snapshot: neu.snapshot,
            zustand: RundeZustand::InBearbeitung,
            begonnen_am: neu.begonnen_am,
            abgeschlossen_am: None,
        }
    }

    /// Rebuild a persisted Runde.
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: RundeId,
        anlage_id: AnlageId,
        zuordnung: Zuordnung,
        snapshot: ParcoursSnapshot,
        zustand: RundeZustand,
        begonnen_am: DateTime<Utc>,
        abgeschlossen_am: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            anlage_id,
            parcours_id: snapshot.parcours_id(),
            zuordnung,
            snapshot,
            zustand,
            begonnen_am,
            abgeschlossen_am,
        }
    }

    pub fn id(&self) -> RundeId {
        self.id
    }

    pub fn anlage_id(&self) -> AnlageId {
        self.anlage_id
    }

    pub fn parcours_id(&self) -> ParcoursId {
        self.parcours_id
    }

    pub fn zuordnung(&self) -> Zuordnung {
        self.zuordnung
    }

    pub fn snapshot(&self) -> &ParcoursSnapshot {
        &self.snapshot
    }

    pub fn zustand(&self) -> RundeZustand {
        self.zustand
    }

    pub fn begonnen_am(&self) -> DateTime<Utc> {
        self.begonnen_am
    }

    pub fn abgeschlossen_am(&self) -> Option<DateTime<Utc>> {
        self.abgeschlossen_am
    }

    pub fn gehört_zu(&self, benutzer_id: BenutzerId) -> bool {
        self.zuordnung.benutzer() == Some(benutzer_id)
    }

    pub fn ist_in_bearbeitung(&self) -> bool {
        self.zustand.ist_in_bearbeitung()
    }

    /// Whether the captured route contains this Ziel (the AC-9 contract a Schuss obeys).
    pub fn enthält(&self, ziel_id: ZielId) -> bool {
        self.snapshot.enthält(ziel_id)
    }

    /// Close the Runde: no further Schuss may be registered.
    pub fn abschliessen(&mut self, am: DateTime<Utc>) -> Result<(), RundeFehler> {
        if !self.ist_in_bearbeitung() {
            return Err(RundeFehler::BereitsAbgeschlossen);
        }
        self.zustand = RundeZustand::Abgeschlossen;
        self.abgeschlossen_am = Some(am);
        Ok(())
    }

    /// Replace the owner with [`Zuordnung::Anonymisiert`] (SPEC-datenschutz retention).
    /// The snapshot and every Schuss stay untouched (NFR-3).
    pub fn anonymisiere(&mut self, am: DateTime<Utc>) {
        self.zuordnung = Zuordnung::Anonymisiert { am };
    }

    /// Fold the per-Ziel results of this Runde into its total per Wertung.
    ///
    /// `schussmengen` is keyed by identity and may arrive in any order: the fold follows
    /// the snapshot's route order, so a Ziel is never counted twice and a Ziel with no
    /// Schuss simply does not appear (AC-8).
    pub fn ergebnis(&self, schussmengen: &[Schussmenge]) -> Rundenergebnis {
        let je_ziel = self.snapshot.ziele().iter().map(|ziel| {
            schussmengen
                .iter()
                .find(|menge| menge.ziel_id() == ziel.ziel_id())
                .and_then(Schussmenge::ergebnis)
        });
        Rundenergebnis::aus_zielen(je_ziel)
    }
}

/// The total per Wertung over one Runde's ordered Ziele.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Rundenergebnis {
    erster_treffer: Punkte,
    mehrschuss: Punkte,
    ziele_mit_ergebnis: usize,
}

impl Rundenergebnis {
    /// Sum per Wertung over all Ziel results. A Ziel without a result is skipped, not
    /// counted as zero (AC-8).
    pub fn aus_zielen(ziele: impl IntoIterator<Item = Option<Zielergebnis>>) -> Self {
        ziele
            .into_iter()
            .flatten()
            .fold(Self::default(), |ergebnis, ziel| Self {
                erster_treffer: ergebnis.erster_treffer + ziel.erster_treffer(),
                mehrschuss: ergebnis.mehrschuss + ziel.mehrschuss(),
                ziele_mit_ergebnis: ergebnis.ziele_mit_ergebnis + 1,
            })
    }

    pub fn erster_treffer(&self) -> Punkte {
        self.erster_treffer
    }

    pub fn mehrschuss(&self) -> Punkte {
        self.mehrschuss
    }

    /// How many Ziele contributed a result.
    pub fn ziele_mit_ergebnis(&self) -> usize {
        self.ziele_mit_ergebnis
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::anlage::{ParcoursSnapshot, SnapshotPflock, SnapshotZiel};
    use crate::domain::schuss::{NeuerSchuss, Schuss, SchussId, Schussmenge, Wertungszone};
    use uuid::Uuid;

    fn snapshot(ziele: &[(u128, &str)]) -> ParcoursSnapshot {
        ParcoursSnapshot::neu(
            ParcoursId(Uuid::from_u128(7)),
            "Waldrunde",
            vec![SnapshotPflock::neu("Weit".to_owned(), "Rot".to_owned(), 0).unwrap()],
            ziele
                .iter()
                .map(|&(id, tiername)| {
                    SnapshotZiel::neu(
                        ZielId(Uuid::from_u128(id)),
                        tiername.to_owned(),
                        2,
                        Vec::new(),
                    )
                    .unwrap()
                })
                .collect(),
        )
        .unwrap()
    }

    fn jetzt() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn runde(ziele: &[(u128, &str)]) -> Runde {
        Runde::neu(
            RundeId(Uuid::nil()),
            NeueRunde::neu(
                AnlageId(Uuid::nil()),
                Zuordnung::Benutzer(BenutzerId(Uuid::nil())),
                snapshot(ziele),
                jetzt(),
            ),
        )
    }

    fn schuss(ziel: u128, pfeil: usize, zone: Wertungszone) -> Schuss {
        Schuss::neu(
            SchussId(Uuid::from_u128(pfeil as u128)),
            NeuerSchuss::neu(
                RundeId(Uuid::nil()),
                Zuordnung::Benutzer(BenutzerId(Uuid::nil())),
                ZielId(Uuid::from_u128(ziel)),
                pfeil,
                zone,
            )
            .unwrap(),
        )
    }

    fn menge(ziel: u128, schüsse: &[(usize, Wertungszone)]) -> Schussmenge {
        Schussmenge::für_ziel(
            ZielId(Uuid::from_u128(ziel)),
            schüsse
                .iter()
                .map(|&(pfeil, zone)| schuss(ziel, pfeil, zone)),
        )
    }

    #[test_log::test]
    fn ac8_die_summe_ist_die_summe_der_zielergebnisse() {
        // Ziel 10: Kill, Kill, Spot = erster 18, mehr 38.
        // Ziel 11: Miss, Spot = erster 14, mehr 14.
        let runde = runde(&[(10, "Keiler"), (11, "Reh")]);
        let mengen = [
            menge(
                10,
                &[
                    (1, Wertungszone::Kill),
                    (2, Wertungszone::Kill),
                    (3, Wertungszone::Spot),
                ],
            ),
            menge(11, &[(1, Wertungszone::Miss), (2, Wertungszone::Spot)]),
        ];
        let ergebnis = runde.ergebnis(&mengen);
        assert_eq!(ergebnis.erster_treffer().value(), 18 + 14);
        assert_eq!(ergebnis.mehrschuss().value(), 38 + 14);
        assert_eq!(ergebnis.ziele_mit_ergebnis(), 2);
    }

    #[test_log::test]
    fn ac8_ziele_ohne_schuss_werden_uebersprungen() {
        let runde = runde(&[(10, "Keiler"), (11, "Reh")]);
        let mengen = [menge(11, &[(1, Wertungszone::Kill)])];
        let ergebnis = runde.ergebnis(&mengen);
        assert_eq!(ergebnis.erster_treffer().value(), 18);
        assert_eq!(ergebnis.mehrschuss().value(), 18);
        assert_eq!(ergebnis.ziele_mit_ergebnis(), 1);
    }

    #[test_log::test]
    fn ac8_ein_ziel_ausserhalb_des_snapshots_zaehlt_nicht() {
        let runde = runde(&[(10, "Keiler")]);
        // A Menge for a Ziel the snapshot does not contain is ignored by the fold.
        let mengen = [
            menge(10, &[(1, Wertungszone::Kill)]),
            menge(99, &[(1, Wertungszone::Spot)]),
        ];
        let ergebnis = runde.ergebnis(&mengen);
        assert_eq!(ergebnis.erster_treffer().value(), 18);
        assert_eq!(ergebnis.ziele_mit_ergebnis(), 1);
    }

    #[test_log::test]
    fn ac8_die_faltung_ist_unabhaengig_von_der_menge_reihenfolge() {
        let runde = runde(&[(10, "Keiler"), (11, "Reh")]);
        let a = menge(10, &[(1, Wertungszone::Body)]);
        let b = menge(11, &[(1, Wertungszone::Kill)]);
        assert_eq!(
            runde.ergebnis(&[a.clone(), b.clone()]),
            runde.ergebnis(&[b, a])
        );
    }

    #[test_log::test]
    fn eine_leere_runde_hat_null() {
        let runde = runde(&[(10, "Keiler")]);
        let ergebnis = runde.ergebnis(&[]);
        assert_eq!(ergebnis.erster_treffer().value(), 0);
        assert_eq!(ergebnis.ziele_mit_ergebnis(), 0);
    }

    #[test_log::test]
    fn abschliessen_ist_einmalig() {
        let mut runde = runde(&[(10, "Keiler")]);
        runde.abschliessen(jetzt()).unwrap();
        assert_eq!(runde.zustand(), RundeZustand::Abgeschlossen);
        assert_eq!(runde.abgeschlossen_am(), Some(jetzt()));
        assert!(matches!(
            runde.abschliessen(jetzt()),
            Err(RundeFehler::BereitsAbgeschlossen)
        ));
    }

    #[test_log::test]
    fn ac9_der_snapshot_kennt_nur_die_platzierte_route() {
        let runde = runde(&[(10, "Keiler")]);
        assert!(runde.enthält(ZielId(Uuid::from_u128(10))));
        assert!(!runde.enthält(ZielId(Uuid::from_u128(11))));
    }

    #[test_log::test]
    fn anonymisieren_laesst_den_snapshot_unberuehrt() {
        let mut runde = runde(&[(10, "Keiler")]);
        let vorher = runde.snapshot().clone();
        runde.anonymisiere(jetzt());
        assert_eq!(runde.zuordnung(), Zuordnung::Anonymisiert { am: jetzt() });
        assert_eq!(runde.snapshot(), &vorher);
    }

    #[test_log::test]
    fn zustand_roundtrip() {
        for zustand in [RundeZustand::InBearbeitung, RundeZustand::Abgeschlossen] {
            assert_eq!(zustand.as_str().parse::<RundeZustand>().unwrap(), zustand);
        }
    }
}
