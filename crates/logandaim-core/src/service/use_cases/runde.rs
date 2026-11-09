//! Starting and closing a Runde.
//!
//! `RundeStarten` captures the Parkour setup as a value copy inside the same transaction that
//! creates the Runde, so no later master-data edit can reach it (SPEC-stammdaten AC-7/AC-9).
//! Reading a Runde is the API facade's: it opens its own transaction, folds the Wertung with
//! the domain's `ZielSchüsse`/`Runde::wertung`, and defines the shape it answers.

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use tracing::{info, instrument};

use crate::{
    application::ExecutionContext,
    domain::{Erfasser, NeueRunde, Runde, RundeId},
    ports::RepositoryError,
    service::UseCase,
};

// ---------------------------------------------------------------------------------------
// RundeStarten
// ---------------------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum RundeStartenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("der Parkour wurde nicht gefunden")]
    NichtGefunden,
    #[error("der Parkour oder seine Anlage ist nicht mehr aktiv")]
    NichtAktiv,
    #[error("für diesen Benutzer läuft bereits eine Runde")]
    RundeLaeuft,
    #[error("die Anlage konnte nicht geladen werden")]
    AnlageLaden,
    #[error("die Runde konnte nicht gestartet werden")]
    Starten,
}

/// Start a Runde at one active Parkour of one active Anlage.
#[derive(Debug, Clone)]
pub struct RundeStarten {
    pub parkour_id: crate::domain::ParkourId,
}

#[async_trait]
impl UseCase<Runde> for RundeStarten {
    type Error = Report<RundeStartenFehler>;

    #[instrument(skip_all, fields(parkour_id = ?self.parkour_id), err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Runde, Self::Error> {
        let benutzer_id = ctx
            .auth()
            .require_benutzer_id()
            .change_context(RundeStartenFehler::NichtAngemeldet)?;
        let jetzt = ctx.clock().now();

        let uow = ctx
            .enter()
            .await
            .change_context(RundeStartenFehler::AnlageLaden)?;

        let ergebnis = async {
            // The owning Anlage is a repository question: the adapter knows which row holds
            // this Parkour, so no caller walks a list of aggregates to find it.
            let anlage = match uow.anlagen().find_by_parkour_id(self.parkour_id).await {
                Ok(anlage) => anlage,
                Err(fehler) => {
                    return Err(match fehler.current_context() {
                        RepositoryError::NotFound => Report::new(RundeStartenFehler::NichtGefunden),
                        _ => fehler.change_context(RundeStartenFehler::AnlageLaden),
                    });
                }
            };

            // "No such Parkour" is 404; "exists but archived" is 409 — and so is a Parkour
            // whose Anlage was deactivated, because no Runde may start there (AC-10).
            let Some(parkour) = anlage
                .aktiver_parkour(self.parkour_id)
                .filter(|_| !anlage.ist_archiviert())
            else {
                return Err(Report::new(RundeStartenFehler::NichtAktiv));
            };

            // AC-7/AC-9: one value copy, taken in this transaction.
            let Ok(snapshot) = anlage.snapshot_für(parkour.id()) else {
                return Err(Report::new(RundeStartenFehler::NichtGefunden));
            };
            let anlage_id = anlage.id();
            match uow.runden().find_offen_by_benutzer(benutzer_id).await {
                Ok(_) => {
                    return Err(Report::new(RundeStartenFehler::RundeLaeuft));
                }
                Err(fehler) if matches!(fehler.current_context(), RepositoryError::NotFound) => {}
                Err(fehler) => {
                    return Err(fehler.change_context(RundeStartenFehler::AnlageLaden));
                }
            }

            uow.runden()
                .create(NeueRunde::neu(
                    anlage_id,
                    Erfasser::Benutzer(benutzer_id),
                    snapshot,
                    jetzt,
                ))
                .await
                .map_err(IntoReport::into_report)
                .map_err(|fehler| match fehler.current_context() {
                    RepositoryError::Conflict => {
                        fehler.change_context(RundeStartenFehler::RundeLaeuft)
                    }
                    _ => fehler.change_context(RundeStartenFehler::Starten),
                })
                .map(|runde| runde.into_data())
        }
        .await;

        let runde = uow.finish(ergebnis, RundeStartenFehler::Starten).await?;
        info!(
            id = ?runde.id(),
            parkour_id = ?runde.parkour_id(),
            "runde gestartet"
        );
        Ok(runde)
    }
}

// ---------------------------------------------------------------------------------------
// RundeLöschen
// ---------------------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum RundeLöschenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("diese Runde gehört nicht zu diesem Benutzer")]
    NichtErlaubt,
    #[error("die Runde wurde nicht gefunden")]
    NichtGefunden,
    #[error("die Runde konnte nicht geladen werden")]
    Laden,
    #[error("die Runde konnte nicht gelöscht werden")]
    Löschen,
}

#[derive(Debug, Clone)]
pub struct RundeLöschen {
    pub runde_id: RundeId,
}

#[async_trait]
impl UseCase<()> for RundeLöschen {
    type Error = Report<RundeLöschenFehler>;

    #[instrument(skip_all, fields(runde_id = ?self.runde_id), err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let benutzer_id = ctx
            .auth()
            .require_benutzer_id()
            .change_context(RundeLöschenFehler::NichtAngemeldet)?;
        let uow = ctx
            .enter()
            .await
            .change_context(RundeLöschenFehler::Laden)?;
        let ergebnis = async {
            let runde = match uow.runden().find_by_id(self.runde_id).await {
                Ok(runde) => runde,
                Err(error) => {
                    return Err(match error.current_context() {
                        RepositoryError::NotFound => Report::new(RundeLöschenFehler::NichtGefunden),
                        _ => error.change_context(RundeLöschenFehler::Laden),
                    });
                }
            };
            if !runde.gehört_zu(benutzer_id) {
                return Err(Report::new(RundeLöschenFehler::NichtErlaubt));
            }
            for schuss in uow
                .schüsse()
                .find_by_runde(self.runde_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RundeLöschenFehler::Löschen)?
            {
                uow.schüsse()
                    .delete(schuss)
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(RundeLöschenFehler::Löschen)?;
            }
            uow.runden()
                .delete(runde)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RundeLöschenFehler::Löschen)
        }
        .await;
        uow.finish(ergebnis, RundeLöschenFehler::Laden).await
    }
}

// ---------------------------------------------------------------------------------------
// RundeAbschliessen
// ---------------------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum RundeAbschliessenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("diese Runde gehört nicht zu diesem Benutzer")]
    NichtErlaubt,
    #[error("die Runde wurde nicht gefunden")]
    NichtGefunden,
    #[error("die Runde ist bereits abgeschlossen")]
    BereitsAbgeschlossen,
    #[error("die Runde konnte nicht geladen werden")]
    RundeLaden,
    #[error("die Runde konnte nicht gespeichert werden")]
    Speichern,
}

/// Close the caller's own Runde. A closed Runde refuses further Schüsse.
#[derive(Debug, Clone)]
pub struct RundeAbschliessen {
    pub runde_id: RundeId,
}

#[async_trait]
impl UseCase<Runde> for RundeAbschliessen {
    type Error = Report<RundeAbschliessenFehler>;

    #[instrument(skip_all, fields(runde_id = ?self.runde_id), err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Runde, Self::Error> {
        let benutzer_id = ctx
            .auth()
            .require_benutzer_id()
            .change_context(RundeAbschliessenFehler::NichtAngemeldet)?;
        let jetzt = ctx.clock().now();

        let uow = ctx
            .enter()
            .await
            .change_context(RundeAbschliessenFehler::RundeLaden)?;

        let ergebnis = async {
            let mut runde = match uow.runden().find_by_id(self.runde_id).await {
                Ok(runde) => runde,
                Err(fehler) => {
                    return Err(match fehler.current_context() {
                        RepositoryError::NotFound => {
                            Report::new(RundeAbschliessenFehler::NichtGefunden)
                        }
                        _ => fehler.change_context(RundeAbschliessenFehler::RundeLaden),
                    });
                }
            };

            if !runde.gehört_zu(benutzer_id) {
                return Err(Report::new(RundeAbschliessenFehler::NichtErlaubt));
            }
            // The typestate makes closing once-only: a closed Runde has no `abschliessen`.
            let Runde::InBearbeitung(offen) = runde.cloned_data() else {
                return Err(Report::new(RundeAbschliessenFehler::BereitsAbgeschlossen));
            };
            runde.set_data(offen.abschliessen(jetzt).into());

            uow.runden()
                .update(&mut runde)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RundeAbschliessenFehler::Speichern)?;

            Ok(runde.into_data())
        }
        .await;

        let runde = uow
            .finish(ergebnis, RundeAbschliessenFehler::RundeLaden)
            .await?;
        info!(id = ?runde.id(), "runde abgeschlossen");
        Ok(runde)
    }
}
