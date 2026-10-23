use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use rustc_hash::FxHashMap;
use tracing::info;

use crate::{
    domain::{Punkte, Rundenergebnis, Schuss, SchussId, Wertungszone, ZielId, Zielwertung},
    ports::NeuerSchuss,
    service::{ExecutionContext, UseCase},
};

/// Register one Schuss at a Ziel.
#[derive(Debug, Clone)]
pub struct SchussRegistrieren {
    pub ziel_id: ZielId,
    pub pfeil: usize,
    pub wertungszone: Wertungszone,
}

#[derive(thiserror::Error, Debug)]
pub enum SchussRegistrierenFehler {
    #[error("fehler beim registrieren des schusses")]
    Erstellung,
}

#[async_trait]
impl UseCase<Schuss> for SchussRegistrieren {
    type Error = Report<SchussRegistrierenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Schuss, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(SchussRegistrierenFehler::Erstellung)?;

        let result = uow
            .schüsse()
            .create(NeuerSchuss {
                ziel_id: self.ziel_id,
                pfeil: self.pfeil,
                wertungszone: self.wertungszone,
            })
            .await
            .map_err(IntoReport::into_report)
            .change_context(SchussRegistrierenFehler::Erstellung);

        let schuss = uow
            .finish(result, SchussRegistrierenFehler::Erstellung)
            .await?;
        info!(
            id = ?schuss.id(),
            ziel_id = ?schuss.ziel_id(),
            pfeil = schuss.pfeil(),
            "schuss registriert"
        );
        Ok(schuss)
    }
}

/// Summary over a selection of Schüsse, computed by the domain rules.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wertungsergebnis {
    pub erster_treffer: Punkte,
    pub mehrschuss: Punkte,
    pub erster_pfeil_getroffen: bool,
    pub anzahl_schüsse: usize,
}

#[derive(thiserror::Error, Debug)]
pub enum WertungFehler {
    #[error("mindestens ein schuss ist erforderlich")]
    KeineSchüsse,
    #[error("schuss nicht gefunden")]
    NichtGefunden,
    #[error("fehler beim laden der schüsse")]
    Laden,
}

/// Summarize Erster Treffer and Mehrschuss for a list of Schuss IDs.
///
/// Schüsse are grouped per Ziel and ordered by `pfeil`; each Ziel is summarized with
/// [`Zielwertung`], then folded with [`Rundenergebnis`] (Ziele without result skipped).
#[derive(Debug, Clone)]
pub struct WertungBerechnen {
    pub schuss_ids: Vec<SchussId>,
}

#[async_trait]
impl UseCase<Wertungsergebnis> for WertungBerechnen {
    type Error = Report<WertungFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Wertungsergebnis, Self::Error> {
        if self.schuss_ids.is_empty() {
            return Err(Report::new(WertungFehler::KeineSchüsse));
        }

        let uow = ctx.enter().await.change_context(WertungFehler::Laden)?;

        let result = uow
            .schüsse()
            .find_by_ids(&self.schuss_ids)
            .await
            .map_err(IntoReport::into_report)
            .change_context(WertungFehler::Laden);

        let schüsse = uow.finish(result, WertungFehler::Laden).await?;

        if schüsse.len() != self.schuss_ids.len() {
            return Err(Report::new(WertungFehler::NichtGefunden));
        }

        let ergebnis = wertungsergebnis_aus_schüssen(&schüsse);

        info!(
            anzahl = ergebnis.anzahl_schüsse,
            erster_treffer = ergebnis.erster_treffer.value(),
            mehrschuss = ergebnis.mehrschuss.value(),
            "wertung berechnet"
        );
        Ok(ergebnis)
    }
}

/// Pure domain-side aggregation: group by Ziel, order by Pfeil, summarize.
fn wertungsergebnis_aus_schüssen(schüsse: &[Schuss]) -> Wertungsergebnis {
    let mut je_ziel: FxHashMap<ZielId, Vec<(usize, Wertungszone)>> = FxHashMap::default();
    for schuss in schüsse {
        je_ziel
            .entry(schuss.ziel_id())
            .or_default()
            .push((schuss.pfeil(), schuss.wertungszone()));
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
    use uuid::Uuid;

    fn schuss(ziel: u8, pfeil: usize, zone: Wertungszone) -> Schuss {
        Schuss::neu(
            SchussId(Uuid::from_u128(pfeil as u128)),
            ZielId(Uuid::from_u128(ziel as u128)),
            pfeil,
            zone,
        )
        .unwrap()
    }

    #[test_log::test]
    fn summiert_je_ziel_und_ordnung_nach_pfeil() {
        let schüsse = vec![
            schuss(1, 1, Wertungszone::Kill),
            schuss(1, 2, Wertungszone::Kill),
            schuss(1, 3, Wertungszone::Spot),
        ];
        let ergebnis = wertungsergebnis_aus_schüssen(&schüsse);
        assert_eq!(ergebnis.erster_treffer.value(), 18);
        assert_eq!(ergebnis.mehrschuss.value(), 38);
        assert!(ergebnis.erster_pfeil_getroffen);
        assert_eq!(ergebnis.anzahl_schüsse, 3);
    }

    #[test_log::test]
    fn ordnet_auch_bei_unsortierter_eingabe() {
        let schüsse = vec![
            schuss(1, 3, Wertungszone::Body),
            schuss(1, 1, Wertungszone::Miss),
            schuss(1, 2, Wertungszone::Spot),
        ];
        let ergebnis = wertungsergebnis_aus_schüssen(&schüsse);
        // Erster Treffer is Pfeil 2 Spot = 14; Mehrschuss = 0 (Miss) + 14 + 4 (Body).
        assert_eq!(ergebnis.erster_treffer.value(), 14);
        assert_eq!(ergebnis.mehrschuss.value(), 18);
        assert!(!ergebnis.erster_pfeil_getroffen);
    }

    #[test_log::test]
    fn ziele_werden_getrennt_summiert() {
        let schüsse = vec![
            schuss(1, 1, Wertungszone::Spot),
            schuss(2, 1, Wertungszone::Body),
            schuss(1, 2, Wertungszone::Miss),
        ];
        let ergebnis = wertungsergebnis_aus_schüssen(&schüsse);
        assert_eq!(ergebnis.erster_treffer.value(), 20 + 16);
        // 20 (Spot) + 0 (Miss at Pfeil 2) + 16 (Body at Pfeil 3)
        assert_eq!(ergebnis.mehrschuss.value(), 36);
    }

    #[test_log::test]
    fn ab_pfeil_vier_zaehlt_null() {
        let schüsse = vec![
            schuss(1, 1, Wertungszone::Miss),
            schuss(1, 2, Wertungszone::Miss),
            schuss(1, 3, Wertungszone::Miss),
            schuss(1, 4, Wertungszone::Kill),
        ];
        let ergebnis = wertungsergebnis_aus_schüssen(&schüsse);
        assert_eq!(ergebnis.erster_treffer.value(), 0);
        assert_eq!(ergebnis.mehrschuss.value(), 0);
    }
}
