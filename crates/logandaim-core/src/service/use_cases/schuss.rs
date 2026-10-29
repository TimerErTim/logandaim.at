//! Registering one Schuss inside the caller's own running Runde.
//!
//! Authorisation is decided here, from the aggregates the use case loads: the Runde must
//! belong to the caller and still be open, and the Ziel must have been part of the
//! captured route (SPEC-stammdaten AC-9, SPEC-wertung rule 1).

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use tracing::info;

use crate::{
    application::ExecutionContext,
    domain::{NeuerSchuss, RundeId, Schuss, SchussFehler, Wertungszone, ZielId, Zuordnung},
    ports::RepositoryError,
    service::UseCase,
};

/// Which Runde, which Ziel, and what the arrow hit.
#[derive(Debug, Clone)]
pub struct SchussRegistrieren {
    pub runde_id: RundeId,
    pub ziel_id: ZielId,
    pub pfeil: usize,
    pub wertungszone: Wertungszone,
}

/// Every way `SchussRegistrieren` may refuse. One HTTP meaning each: 401, 403, 400, 404,
/// 409.
#[derive(Debug, thiserror::Error)]
pub enum SchussRegistrierenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("diese Runde gehört nicht zu diesem Benutzer")]
    NichtErlaubt,
    #[error("die Anfrage ist ungültig")]
    UngültigeAnfrage,
    #[error("die Runde wurde nicht gefunden")]
    NichtGefunden,
    #[error("die Runde ist nicht mehr in Bearbeitung")]
    RundeNichtInBearbeitung,
    #[error("das Ziel ist nicht Teil des Runden-Snapshots")]
    ZielNichtImSnapshot,
    #[error("der Schuss konnte nicht gespeichert werden")]
    Speichern,
    #[error("die Runde konnte nicht geladen werden")]
    RundeLaden,
}

#[async_trait]
impl UseCase<Schuss> for SchussRegistrieren {
    type Error = Report<SchussRegistrierenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Schuss, Self::Error> {
        let (benutzer_id, _) = ctx
            .auth()
            .require_authenticated()
            .change_context(SchussRegistrierenFehler::NichtAngemeldet)?;

        let uow = ctx
            .enter()
            .await
            .change_context(SchussRegistrierenFehler::RundeLaden)?;

        let ergebnis = async {
            let runde = match uow.runden().find_by_id(self.runde_id).await {
                Ok(runde) => runde,
                Err(fehler)
                    if fehler
                        .downcast_ref::<RepositoryError>()
                        .is_some_and(|error| matches!(error, RepositoryError::NotFound)) =>
                {
                    return Err(Report::new(SchussRegistrierenFehler::NichtGefunden));
                }
                Err(fehler) => {
                    return Err(fehler).change_context(SchussRegistrierenFehler::RundeLaden);
                }
            };

            // Ownership, never the body: a caller cannot register for someone else.
            if !runde.gehört_zu(benutzer_id) {
                return Err(Report::new(SchussRegistrierenFehler::NichtErlaubt));
            }
            if !runde.ist_in_bearbeitung() {
                return Err(Report::new(
                    SchussRegistrierenFehler::RundeNichtInBearbeitung,
                ));
            }
            // AC-9: a Schuss may only name a Ziel the Runde captured.
            if !runde.enthält(self.ziel_id) {
                return Err(Report::new(SchussRegistrierenFehler::ZielNichtImSnapshot));
            }

            let neu = NeuerSchuss::neu(
                self.runde_id,
                Zuordnung::Benutzer(benutzer_id),
                self.ziel_id,
                self.pfeil,
                self.wertungszone,
            )
            .change_context(SchussRegistrierenFehler::UngültigeAnfrage)?;

            uow.schüsse()
                .create(neu)
                .await
                .map_err(IntoReport::into_report)
                .change_context(SchussRegistrierenFehler::Speichern)
        }
        .await;

        let schuss = uow
            .finish(ergebnis, SchussRegistrierenFehler::RundeLaden)
            .await?;
        info!(
            id = ?schuss.id(),
            runde_id = ?schuss.runde_id(),
            ziel_id = ?schuss.ziel_id(),
            pfeil = schuss.pfeil().get(),
            "schuss registriert"
        );
        Ok(schuss.into_data())
    }
}

impl From<SchussFehler> for SchussRegistrierenFehler {
    fn from(_: SchussFehler) -> Self {
        Self::UngültigeAnfrage
    }
}
