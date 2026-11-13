//! Registering, correcting, and deleting one Schuss inside the caller's own running Runde.
//!
//! Authorisation is decided here, from the aggregates the use case loads: the Runde must
//! belong to the caller and still be open, and the Ziel must have been part of the
//! captured route (SPEC-stammdaten AC-9, SPEC-wertung rule 1).
//!
//! Position validation is a typestate question, so each write asks the open state it holds:
//! a closed round has no `pruefe_position` at all, and this module refuses it up front
//! instead of pretending the question has an answer.

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use tracing::{info, instrument};

use crate::{
    application::ExecutionContext,
    domain::{
        Erfasser, NeuerSchuss, Runde, RundeId, Schuss, SchussFehler, SchussId, Schussposition,
        Wertungszone, ZielId, ZielSchüsse,
    },
    ports::RepositoryError,
    service::UseCase,
};

/// Which Runde, which Ziel, and what the arrow hit.
#[derive(Debug, Clone)]
pub struct SchussRegistrieren {
    pub runde_id: RundeId,
    pub ziel_id: ZielId,
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
    #[error("das Ziel ist nicht Teil des Aufbaus dieser Runde")]
    ZielNichtImAufbau,
    #[error("der Pflock ist nicht Teil des Aufbaus dieser Runde")]
    PflockNichtImAufbau,
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

            // AC-9: a Schuss may only name a Ziel the Runde captured, and only a Pflock that
            // Ziel's captured set carries — or, on a Freie Runde, any colour at all. The
            // open typestate owns the question, because a closed round has no answer.
            let Some(offen) = offene_runde(&runde)? else {
                return Err(Report::new(
                    SchussRegistrierenFehler::RundeNichtInBearbeitung,
                ));
            };
            if !offen.enthält_ziel(self.ziel_id) {
                return Err(Report::new(SchussRegistrierenFehler::ZielNichtImAufbau));
            }
            let position_ok = match &offen {
                OffeneRunde::MitParkour(runde) => {
                    runde.pruefe_position(self.ziel_id, self.position)
                }
                OffeneRunde::Freie(runde) => runde.pruefe_position(self.ziel_id, self.position),
            };
            if position_ok.is_err() {
                return Err(Report::new(SchussRegistrierenFehler::PflockNichtImAufbau));
            }

            let vorhandene: Vec<Schuss> = uow
                .schüsse()
                .find_by_runde(self.runde_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(SchussRegistrierenFehler::Speichern)?
                .into_iter()
                .map(|schuss| schuss.into_data())
                .filter(|schuss| schuss.ziel_id() == self.ziel_id)
                .collect();
            let ziel_schüsse = ZielSchüsse::für_ziel(self.ziel_id, vorhandene)
                .expect("all filtered Schüsse belong to this Ziel");
            let pfeil = ziel_schüsse.nächster_pfeil().get();
            let neu = NeuerSchuss::neu(
                self.runde_id,
                Erfasser::Benutzer(benutzer_id),
                self.ziel_id,
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

/// The two open typestates, borrowed from the loaded round.
///
/// A closed round has no position rule at all, which is why the guard is expressed as "give
/// me an open round" rather than as a boolean the caller could ignore.
enum OffeneRunde<'a> {
    MitParkour(&'a crate::domain::RundeMitParkour),
    Freie(&'a crate::domain::FreieRunde),
}

impl<'a> OffeneRunde<'a> {
    fn enthält_ziel(&self, ziel_id: ZielId) -> bool {
        match self {
            Self::MitParkour(runde) => runde.enthält_ziel(ziel_id),
            Self::Freie(runde) => runde.enthält_ziel(ziel_id),
        }
    }
}

/// The open state of `runde`, or `None` when it is closed.
fn offene_runde(
    runde: &Runde,
) -> Result<Option<OffeneRunde<'_>>, Report<SchussRegistrierenFehler>> {
    Ok(match runde {
        Runde::MitParkour(offen) => Some(OffeneRunde::MitParkour(offen)),
        Runde::FreieRunde(offen) => Some(OffeneRunde::Freie(offen)),
        Runde::AbgeschlosseneRunde(_) => None,
    })
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
    #[error("die Runde ist nicht mehr in Bearbeitung")]
    RundeNichtInBearbeitung,
    #[error("der Pflock ist nicht Teil des Aufbaus dieser Runde")]
    PflockNichtImAufbau,
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
            // A closed round freezes its Schüsse too: the typestate has no position rule to
            // ask, so the write is refused before anything is read.
            if !runde.ist_in_bearbeitung() {
                return Err(Report::new(SchussÄndernFehler::RundeNichtInBearbeitung));
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
            // The Ziel's captured set decides a Parkour round's Pflock; a Freie Runde
            // accepts any colour. Both ask their own open typestate.
            let position_ok = match &*runde {
                Runde::MitParkour(offen) => offen.pruefe_position(schuss.ziel_id(), self.position),
                Runde::FreieRunde(offen) => offen.pruefe_position(schuss.ziel_id(), self.position),
                // The guard above already refused this; the arm exists so the match is total.
                Runde::AbgeschlosseneRunde(_) => {
                    return Err(Report::new(SchussÄndernFehler::RundeNichtInBearbeitung));
                }
            };
            if position_ok.is_err() {
                return Err(Report::new(SchussÄndernFehler::PflockNichtImAufbau));
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
    #[error("die Runde ist nicht mehr in Bearbeitung")]
    RundeNichtInBearbeitung,
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
            // A closed round freezes its Schüsse: deleting one would renumber the rest.
            if !runde.ist_in_bearbeitung() {
                return Err(Report::new(SchussLöschenFehler::RundeNichtInBearbeitung));
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
            let ziel_id = schuss.ziel_id();
            let ziel_schüsse = uow
                .schüsse()
                .find_by_runde(self.runde_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(SchussLöschenFehler::Laden)?
                .into_iter()
                .map(|row| row.into_data())
                .filter(|row| row.ziel_id() == ziel_id)
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
