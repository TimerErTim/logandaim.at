use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use tracing::info;

use crate::{
    domain::{
        NeuerSchuss, Schuss, SchussId, Wertungsergebnis, Wertungszone, ZielId, wertung_aus_schüssen,
    },
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
        let neu = NeuerSchuss::neu(self.ziel_id, self.pfeil, self.wertungszone)
            .change_context(SchussRegistrierenFehler::Erstellung)?;

        let uow = ctx
            .enter()
            .await
            .change_context(SchussRegistrierenFehler::Erstellung)?;

        let result = uow
            .schüsse()
            .create(neu)
            .await
            .map_err(IntoReport::into_report)
            .change_context(SchussRegistrierenFehler::Erstellung);

        let schuss = uow
            .finish(result, SchussRegistrierenFehler::Erstellung)
            .await?;
        info!(
            id = ?schuss.id(),
            ziel_id = ?schuss.ziel_id(),
            pfeil = schuss.pfeil().get(),
            "schuss registriert"
        );
        Ok(schuss.into_data())
    }
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
/// Loading only; the aggregation itself is the domain service
/// [`crate::domain::wertung_aus_schüssen`].
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

        let schüsse: Vec<Schuss> = schüsse
            .into_iter()
            .map(|schuss| schuss.into_data())
            .collect();

        if schüsse.len() != self.schuss_ids.len() {
            return Err(Report::new(WertungFehler::NichtGefunden));
        }

        let ergebnis = wertung_aus_schüssen(&schüsse);

        info!(
            anzahl = ergebnis.anzahl_schüsse,
            erster_treffer = ergebnis.erster_treffer.value(),
            mehrschuss = ergebnis.mehrschuss.value(),
            "wertung berechnet"
        );
        Ok(ergebnis)
    }
}
