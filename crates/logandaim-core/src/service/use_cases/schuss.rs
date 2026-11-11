//! Registering one Schuss inside the caller's own running Runde.
//!
//! Authorisation is decided here, from the aggregates the use case loads: the Runde must
//! belong to the caller and still be open, and the Ziel must have been part of the
//! captured route (SPEC-stammdaten AC-9, SPEC-wertung rule 1).

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use tracing::{info, instrument};

use crate::{
    application::ExecutionContext,
    domain::{
        Erfasser, NeuerSchuss, RundeId, Schuss, SchussFehler, SchussId, SchussZielId,
        Schussposition, Wertungszone, ZielSchüsse,
    },
    ports::RepositoryError,
    service::UseCase,
};

/// Which Runde, which Ziel, and what the arrow hit.
#[derive(Debug, Clone)]
pub struct SchussRegistrieren {
    pub runde_id: RundeId,
    pub ziel_id: uuid::Uuid,
    pub wertungszone: Wertungszone,
    pub position: Option<Schussposition>,
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
    #[error("der Pflock ist nicht Teil des Runden-Snapshots")]
    PflockNichtImSnapshot,
    #[error("die Pfeilnummer wurde parallel vergeben; bitte erneut versuchen")]
    PfeilKonflikt,
    #[error("der Schuss konnte nicht gespeichert werden")]
    Speichern,
    #[error("die Runde konnte nicht geladen werden")]
    RundeLaden,
}

#[async_trait]
impl UseCase<Schuss> for SchussRegistrieren {
    type Error = Report<SchussRegistrierenFehler>;

    #[instrument(skip_all, fields(runde_id = ?self.runde_id, ziel_id = ?self.ziel_id), err(Debug))]
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
                Err(fehler) => {
                    return Err(match fehler.current_context() {
                        RepositoryError::NotFound => {
                            Report::new(SchussRegistrierenFehler::NichtGefunden)
                        }
                        _ => fehler.change_context(SchussRegistrierenFehler::RundeLaden),
                    });
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
            let Some(ziel_id) = runde
                .aufbau()
                .ziel_id_für_schuss(SchussZielId(self.ziel_id))
            else {
                return Err(Report::new(SchussRegistrierenFehler::ZielNichtImSnapshot));
            };

            // AC-9: a Schuss may only name a Pflock the Runde captured at this Ziel, or —
            // on a Freie Runde — any colour at all. The setup owns the question, so this use
            // case never asks which setup it is holding (SPEC-runde rule 19).
            if runde
                .aufbau()
                .pruefe_position(ziel_id, self.position)
                .is_err()
            {
                return Err(Report::new(SchussRegistrierenFehler::PflockNichtImSnapshot));
            }
            let vorhandene: Vec<Schuss> = uow
                .schüsse()
                .find_by_runde(self.runde_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(SchussRegistrierenFehler::Speichern)?
                .into_iter()
                .map(|schuss| schuss.into_data())
                .filter(|schuss| {
                    runde.aufbau().ziel_id_für_schuss(schuss.ziel_id()) == Some(ziel_id)
                })
                .collect();
            let ziel_schüsse = ZielSchüsse::für_ziel(ziel_id, vorhandene)
                .expect("all filtered Schüsse belong to this Ziel");
            let pfeil = ziel_schüsse.nächster_pfeil().get();
            let neu = NeuerSchuss::neu(
                self.runde_id,
                Erfasser::Benutzer(benutzer_id),
                ziel_id.as_schuss_ziel_id(),
                pfeil,
                self.wertungszone,
                self.position,
            )
            .change_context(SchussRegistrierenFehler::UngültigeAnfrage)?;

            uow.schüsse()
                .create(neu)
                .await
                .map_err(IntoReport::into_report)
                .map_err(|error| match error.current_context() {
                    RepositoryError::Conflict => {
                        Report::new(SchussRegistrierenFehler::PfeilKonflikt)
                    }
                    _ => error.change_context(SchussRegistrierenFehler::Speichern),
                })
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

#[derive(Debug, thiserror::Error)]
pub enum SchussÄndernFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("diese Runde gehört nicht zu diesem Benutzer")]
    NichtErlaubt,
    #[error("der Schuss wurde nicht gefunden")]
    NichtGefunden,
    #[error("die Anfrage ist ungültig")]
    UngültigeAnfrage,
    #[error("der Pflock ist nicht Teil des Runden-Snapshots")]
    PflockNichtImSnapshot,
    #[error("die Runde konnte nicht geladen werden")]
    Laden,
    #[error("der Schuss konnte nicht gespeichert werden")]
    Speichern,
}

#[derive(Debug, Clone)]
pub struct SchussÄndern {
    pub runde_id: RundeId,
    pub schuss_id: SchussId,
    pub wertungszone: Wertungszone,
    pub position: Option<Schussposition>,
}

#[async_trait]
impl UseCase<Schuss> for SchussÄndern {
    type Error = Report<SchussÄndernFehler>;

    #[instrument(skip_all, fields(runde_id = ?self.runde_id, schuss_id = ?self.schuss_id), err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Schuss, Self::Error> {
        let benutzer_id = ctx
            .auth()
            .require_benutzer_id()
            .change_context(SchussÄndernFehler::NichtAngemeldet)?;
        let uow = ctx
            .enter()
            .await
            .change_context(SchussÄndernFehler::Laden)?;
        let ergebnis = async {
            let runde = match uow.runden().find_by_id(self.runde_id).await {
                Ok(runde) => runde,
                Err(error) => {
                    return Err(match error.current_context() {
                        RepositoryError::NotFound => Report::new(SchussÄndernFehler::NichtGefunden),
                        _ => error.change_context(SchussÄndernFehler::Laden),
                    });
                }
            };
            if !runde.gehört_zu(benutzer_id) {
                return Err(Report::new(SchussÄndernFehler::NichtErlaubt));
            }

            let mut schuss = match uow.schüsse().find_by_id(self.schuss_id).await {
                Ok(schuss) if schuss.runde_id() == self.runde_id => schuss,
                Ok(_) => return Err(Report::new(SchussÄndernFehler::NichtGefunden)),
                Err(error) => {
                    return Err(match error.current_context() {
                        RepositoryError::NotFound => Report::new(SchussÄndernFehler::NichtGefunden),
                        _ => error.change_context(SchussÄndernFehler::Laden),
                    });
                }
            };
            // The Ziel's effective set decides a Parkour round's Pflock; a Freie Runde
            // accepts any colour. Both go through the setup's one validation seam.
            if runde
                .aufbau()
                .pruefe_position(
                    runde
                        .aufbau()
                        .ziel_id_für_schuss(schuss.ziel_id())
                        .ok_or_else(|| Report::new(SchussÄndernFehler::NichtGefunden))?,
                    self.position,
                )
                .is_err()
            {
                return Err(Report::new(SchussÄndernFehler::PflockNichtImSnapshot));
            }
            schuss.set_data(
                schuss
                    .cloned_data()
                    .geändert(self.wertungszone, self.position),
            );
            uow.schüsse()
                .update(&mut schuss)
                .await
                .map_err(IntoReport::into_report)
                .change_context(SchussÄndernFehler::Speichern)?;
            Ok(schuss.into_data())
        }
        .await;
        uow.finish(ergebnis, SchussÄndernFehler::Laden).await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SchussLöschenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("diese Runde gehört nicht zu diesem Benutzer")]
    NichtErlaubt,
    #[error("der Schuss wurde nicht gefunden")]
    NichtGefunden,
    #[error("die Runde konnte nicht geladen werden")]
    Laden,
    #[error("der Schuss konnte nicht gelöscht werden")]
    Löschen,
    #[error("die verbleibenden Schüsse konnten nicht neu nummeriert werden")]
    Speichern,
}

#[derive(Debug, Clone)]
pub struct SchussLöschen {
    pub runde_id: RundeId,
    pub schuss_id: SchussId,
}

#[async_trait]
impl UseCase<()> for SchussLöschen {
    type Error = Report<SchussLöschenFehler>;

    #[instrument(skip_all, fields(runde_id = ?self.runde_id, schuss_id = ?self.schuss_id), err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let benutzer_id = ctx
            .auth()
            .require_benutzer_id()
            .change_context(SchussLöschenFehler::NichtAngemeldet)?;
        let uow = ctx
            .enter()
            .await
            .change_context(SchussLöschenFehler::Laden)?;
        let ergebnis = async {
            let runde = match uow.runden().find_by_id(self.runde_id).await {
                Ok(runde) => runde,
                Err(error) => {
                    return Err(match error.current_context() {
                        RepositoryError::NotFound => {
                            Report::new(SchussLöschenFehler::NichtGefunden)
                        }
                        _ => error.change_context(SchussLöschenFehler::Laden),
                    });
                }
            };
            if !runde.gehört_zu(benutzer_id) {
                return Err(Report::new(SchussLöschenFehler::NichtErlaubt));
            }

            let schuss = match uow.schüsse().find_by_id(self.schuss_id).await {
                Ok(schuss) if schuss.runde_id() == self.runde_id => schuss,
                Ok(_) => return Err(Report::new(SchussLöschenFehler::NichtGefunden)),
                Err(error) => {
                    return Err(match error.current_context() {
                        RepositoryError::NotFound => {
                            Report::new(SchussLöschenFehler::NichtGefunden)
                        }
                        _ => error.change_context(SchussLöschenFehler::Laden),
                    });
                }
            };
            let ziel_id = runde
                .aufbau()
                .ziel_id_für_schuss(schuss.ziel_id())
                .expect("persisted shot target belongs to its round");
            let ziel_schüsse = uow
                .schüsse()
                .find_by_runde(self.runde_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(SchussLöschenFehler::Laden)?
                .into_iter()
                .map(|row| row.into_data())
                .filter(|row| runde.aufbau().ziel_id_für_schuss(row.ziel_id()) == Some(ziel_id))
                .collect::<Vec<_>>();
            let verbleibend = ZielSchüsse::für_ziel(ziel_id, ziel_schüsse)
                .expect("the target's shot list contains only that target")
                .ohne_schuss(self.schuss_id);

            uow.schüsse()
                .delete(schuss)
                .await
                .map_err(IntoReport::into_report)
                .change_context(SchussLöschenFehler::Löschen)?;
            let offset = verbleibend.len() + 1;
            let mut rows = Vec::with_capacity(verbleibend.len());
            for aktualisiert in verbleibend {
                let mut gespeichert = uow
                    .schüsse()
                    .find_by_id(aktualisiert.id())
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(SchussLöschenFehler::Speichern)?;
                let temporaer = gespeichert.pfeil().get() + offset;
                gespeichert.set_data(
                    gespeichert
                        .cloned_data()
                        .mit_pfeil(temporaer)
                        .expect("temporary Pfeil remains positive"),
                );
                uow.schüsse()
                    .update(&mut gespeichert)
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(SchussLöschenFehler::Speichern)?;
                rows.push((gespeichert, aktualisiert));
            }
            for (mut gespeichert, aktualisiert) in rows {
                gespeichert.set_data(aktualisiert);
                uow.schüsse()
                    .update(&mut gespeichert)
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(SchussLöschenFehler::Speichern)?;
            }
            Ok(())
        }
        .await;
        uow.finish(ergebnis, SchussLöschenFehler::Laden).await
    }
}
