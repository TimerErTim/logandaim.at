//! Starting and closing a Runde.
//!
//! `RundeStarten` captures the Parkour setup as a value copy inside the same transaction that
//! creates the Runde, so no later master-data edit can reach it (SPEC-stammdaten AC-7/AC-9).
//! Reading a Runde is the API facade's: it opens its own transaction, folds the Wertung with
//! the domain's `ZielSchüsse`/`Runde::wertung`, and defines the shape it answers.

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use tracing::info;

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

            uow.runden()
                .create(NeueRunde::neu(
                    anlage_id,
                    Erfasser::Benutzer(benutzer_id),
                    snapshot,
                    jetzt,
                ))
                .await
                .map_err(IntoReport::into_report)
                .change_context(RundeStartenFehler::Starten)
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

// ---------------------------------------------------------------------------------------
// RundenAnzeigen error
// ---------------------------------------------------------------------------------------

/// The refusals of the caller's own Runden list, which the API facade reads inline.
#[derive(Debug, thiserror::Error)]
pub enum RundenAnzeigenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("die Runden konnten nicht geladen werden")]
    Laden,
}

// ---------------------------------------------------------------------------------------
// RundeAnzeigen error
// ---------------------------------------------------------------------------------------

/// The refusals of one Runde's detail read, which the API facade reads inline.
#[derive(Debug, thiserror::Error)]
pub enum RundeAnzeigenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("diese Runde darf nicht eingesehen werden")]
    NichtErlaubt,
    #[error("die Runde wurde nicht gefunden")]
    NichtGefunden,
    #[error("die Runde konnte nicht geladen werden")]
    RundeLaden,
    #[error("die Schüsse konnten nicht geladen werden")]
    SchüsseLaden,
    #[error("der Benutzer konnte nicht geladen werden")]
    BenutzerLaden,
}
