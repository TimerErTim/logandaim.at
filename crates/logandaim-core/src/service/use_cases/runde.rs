//! Starting, closing, deleting a Runde, and editing a free round's Ziel list.
//!
//! `RundeStarten` captures the Parkour setup as a value copy inside the same transaction that
//! creates the Runde, so no later master-data edit can reach it (SPEC-stammdaten AC-7/AC-9).
//! A `Freie Runde` skips master data entirely: it names itself and owns its Ziele
//! (SPEC-runde rules 17–18). Reading a Runde is the API facade's: it opens its own
//! transaction, folds the Wertung with the domain's `ZielSchüsse`/`Runde::wertung`, and
//! defines the shape it answers.

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use tracing::{info, instrument};

use crate::{
    application::ExecutionContext,
    domain::{
        Erfasser, NeueRunde, NonEmptyString, ParkourId, Runde, RundeId, RundenZielId, Rundenaufbau,
    },
    ports::RepositoryError,
    service::UseCase,
    uow::{UnitOfWork, Versioned},
};

// ---------------------------------------------------------------------------------------
// RundeStarten
// ---------------------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum RundeStartenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("die Anfrage ist ungültig")]
    UngültigeAnfrage,
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

/// Where a Runde's setup comes from: a master-data Parkour, or the archer's own name.
///
/// A variant, so "a free round that also names a Parkour" is unrepresentable.
#[derive(Debug, Clone)]
pub enum Rundenquelle {
    Parkour(ParkourId),
    Frei { name: NonEmptyString },
}

/// Start a Runde: at one active Parkour, or as a `Freie Runde` with no Parkour at all.
#[derive(Debug, Clone)]
pub struct RundeStarten {
    pub quelle: Rundenquelle,
}

impl RundeStarten {
    pub fn mit_parkour(parkour_id: ParkourId) -> Self {
        Self {
            quelle: Rundenquelle::Parkour(parkour_id),
        }
    }

    pub fn ohne_parkour(name: NonEmptyString) -> Self {
        Self {
            quelle: Rundenquelle::Frei { name },
        }
    }
}

#[async_trait]
impl UseCase<Runde> for RundeStarten {
    type Error = Report<RundeStartenFehler>;

    #[instrument(skip_all, fields(quelle = ?self.quelle), err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Runde, Self::Error> {
        let benutzer_id = ctx
            .auth()
            .require_benutzer_id()
            .change_context(RundeStartenFehler::NichtAngemeldet)?;
        let jetzt = ctx.clock().now();
        let erfasser = Erfasser::Benutzer(benutzer_id);

        let uow = ctx
            .enter()
            .await
            .change_context(RundeStartenFehler::AnlageLaden)?;

        let ergebnis = async {
            let neue = match &self.quelle {
                Rundenquelle::Parkour(parkour_id) => {
                    // The owning Anlage is a repository question: the adapter knows which row
                    // holds this Parkour, so no caller walks a list of aggregates to find it.
                    let anlage = match uow.anlagen().find_by_parkour_id(*parkour_id).await {
                        Ok(anlage) => anlage,
                        Err(fehler) => {
                            return Err(match fehler.current_context() {
                                RepositoryError::NotFound => {
                                    Report::new(RundeStartenFehler::NichtGefunden)
                                }
                                _ => fehler.change_context(RundeStartenFehler::AnlageLaden),
                            });
                        }
                    };

                    // "No such Parkour" is 404; "exists but archived" is 409 — and so is a
                    // Parkour whose Anlage was deactivated (AC-10).
                    let Some(parkour) = anlage
                        .aktiver_parkour(*parkour_id)
                        .filter(|_| !anlage.ist_archiviert())
                    else {
                        return Err(Report::new(RundeStartenFehler::NichtAktiv));
                    };

                    // AC-7/AC-9: one value copy, taken in this transaction.
                    let Ok(snapshot) = anlage.snapshot_für(parkour.id()) else {
                        return Err(Report::new(RundeStartenFehler::NichtGefunden));
                    };
                    NeueRunde::mit_parkour(anlage.id(), erfasser, snapshot, jetzt)
                }
                Rundenquelle::Frei { name } => {
                    NeueRunde::ohne_parkour(name.clone(), erfasser, jetzt)
                }
            };

            // The one-open-Runde rule covers both setups: a free round and a Parkour round
            // fill the same slot (SPEC-runde rule 20). Checked after the setup, so a missing
            // or archived Parkour still answers its own 404/409 rather than "a round runs".
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
                .create(neue)
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
            parkour_id = ?match runde.aufbau() {
                Rundenaufbau::Parkour { parkour_id, .. } => Some(parkour_id),
                Rundenaufbau::Freie { .. } => None,
            },
            "runde gestartet"
        );
        Ok(runde)
    }
}

// ---------------------------------------------------------------------------------------
// ZielHinzufügen / ZielEntfernen (the free round's JIT Ziel list)
// ---------------------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum RundeZielHinzufügenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("diese Runde gehört nicht zu diesem Benutzer")]
    NichtErlaubt,
    #[error("die Runde wurde nicht gefunden")]
    NichtGefunden,
    #[error("diese Runde ist keine freie Runde")]
    RundeIstKeineFreieRunde,
    #[error("die Runde ist nicht mehr in Bearbeitung")]
    RundeNichtInBearbeitung,
    #[error("die Runde konnte nicht geladen werden")]
    RundeLaden,
    #[error("die Runde konnte nicht gespeichert werden")]
    Speichern,
}

/// Append one `Freies Ziel` to the caller's own open `Freie Runde`.
#[derive(Debug, Clone)]
pub struct RundeZielHinzufügen {
    pub runde_id: RundeId,
    pub name: Option<NonEmptyString>,
}

#[async_trait]
impl UseCase<Runde> for RundeZielHinzufügen {
    type Error = Report<RundeZielHinzufügenFehler>;

    #[instrument(skip_all, fields(runde_id = ?self.runde_id), err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Runde, Self::Error> {
        let benutzer_id = ctx
            .auth()
            .require_benutzer_id()
            .change_context(RundeZielHinzufügenFehler::NichtAngemeldet)?;
        let uow = ctx
            .enter()
            .await
            .change_context(RundeZielHinzufügenFehler::RundeLaden)?;

        let ergebnis = async {
            let mut runde = laden(&uow, self.runde_id).await?;
            if !runde.gehört_zu(benutzer_id) {
                return Err(Report::new(RundeZielHinzufügenFehler::NichtErlaubt));
            }
            if !runde.ist_in_bearbeitung() {
                return Err(Report::new(
                    RundeZielHinzufügenFehler::RundeNichtInBearbeitung,
                ));
            }
            if !runde.ist_freie_runde() {
                return Err(Report::new(
                    RundeZielHinzufügenFehler::RundeIstKeineFreieRunde,
                ));
            }
            if !runde.ziel_hinzufügen(RundenZielId(uuid::Uuid::new_v4()), self.name) {
                return Err(Report::new(
                    RundeZielHinzufügenFehler::RundeIstKeineFreieRunde,
                ));
            }

            uow.runden()
                .update(&mut runde)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RundeZielHinzufügenFehler::Speichern)?;
            Ok(runde.into_data())
        }
        .await;

        let runde = uow
            .finish(ergebnis, RundeZielHinzufügenFehler::RundeLaden)
            .await?;
        info!(id = ?runde.id(), "freies ziel hinzugefügt");
        Ok(runde)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RundeZielEntfernenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("diese Runde gehört nicht zu diesem Benutzer")]
    NichtErlaubt,
    #[error("die Runde wurde nicht gefunden")]
    NichtGefunden,
    #[error("das Ziel wurde in dieser Runde nicht gefunden")]
    ZielNichtGefunden,
    #[error("diese Runde ist keine freie Runde")]
    RundeIstKeineFreieRunde,
    #[error("die Runde konnte nicht geladen werden")]
    RundeLaden,
    #[error("die Runde ist nicht mehr in Bearbeitung")]
    RundeNichtInBearbeitung,
    #[error("das Ziel konnte nicht entfernt werden")]
    Entfernen,
    #[error("die Runde konnte nicht gespeichert werden")]
    Speichern,
}

/// Remove one `Freies Ziel` and, in the same transaction, every Schuss logged at it.
#[derive(Debug, Clone)]
pub struct RundeZielEntfernen {
    pub runde_id: RundeId,
    pub ziel_id: RundenZielId,
}

#[async_trait]
impl UseCase<Runde> for RundeZielEntfernen {
    type Error = Report<RundeZielEntfernenFehler>;

    #[instrument(skip_all, fields(runde_id = ?self.runde_id, ziel_id = ?self.ziel_id), err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Runde, Self::Error> {
        let benutzer_id = ctx
            .auth()
            .require_benutzer_id()
            .change_context(RundeZielEntfernenFehler::NichtAngemeldet)?;
        let uow = ctx
            .enter()
            .await
            .change_context(RundeZielEntfernenFehler::RundeLaden)?;

        let ergebnis = async {
            let mut runde = laden_entfernen(&uow, self.runde_id).await?;
            if !runde.gehört_zu(benutzer_id) {
                return Err(Report::new(RundeZielEntfernenFehler::NichtErlaubt));
            }
            if !runde.ist_freie_runde() {
                return Err(Report::new(
                    RundeZielEntfernenFehler::RundeIstKeineFreieRunde,
                ));
            }
            if !runde.ist_in_bearbeitung() {
                return Err(Report::new(
                    RundeZielEntfernenFehler::RundeNichtInBearbeitung,
                ));
            }
            if runde.ziel_entfernen(self.ziel_id).is_none() {
                return Err(Report::new(RundeZielEntfernenFehler::ZielNichtGefunden));
            }

            // The cascade is the point: a removed Ziel takes its Schüsse with it, in this
            // transaction, so a half-removed Ziel cannot survive (SPEC-runde rule 18).
            for schuss in uow
                .schüsse()
                .find_by_runde(self.runde_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RundeZielEntfernenFehler::Entfernen)?
            {
                if schuss.ziel_id() == self.ziel_id.into() {
                    uow.schüsse()
                        .delete(schuss)
                        .await
                        .map_err(IntoReport::into_report)
                        .change_context(RundeZielEntfernenFehler::Entfernen)?;
                }
            }

            uow.runden()
                .update(&mut runde)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RundeZielEntfernenFehler::Speichern)?;
            Ok(runde.into_data())
        }
        .await;

        let runde = uow
            .finish(ergebnis, RundeZielEntfernenFehler::RundeLaden)
            .await?;
        info!(id = ?runde.id(), ziel_id = ?self.ziel_id, "freies ziel entfernt");
        Ok(runde)
    }
}

async fn laden(
    uow: &UnitOfWork<'_>,
    runde_id: RundeId,
) -> Result<Versioned<Runde>, Report<RundeZielHinzufügenFehler>> {
    match uow.runden().find_by_id(runde_id).await {
        Ok(runde) => Ok(runde),
        Err(fehler) => Err(match fehler.current_context() {
            RepositoryError::NotFound => Report::new(RundeZielHinzufügenFehler::NichtGefunden),
            _ => fehler.change_context(RundeZielHinzufügenFehler::RundeLaden),
        }),
    }
}

async fn laden_entfernen(
    uow: &UnitOfWork<'_>,
    runde_id: RundeId,
) -> Result<Versioned<Runde>, Report<RundeZielEntfernenFehler>> {
    match uow.runden().find_by_id(runde_id).await {
        Ok(runde) => Ok(runde),
        Err(fehler) => Err(match fehler.current_context() {
            RepositoryError::NotFound => Report::new(RundeZielEntfernenFehler::NichtGefunden),
            _ => fehler.change_context(RundeZielEntfernenFehler::RundeLaden),
        }),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RundeZielUmbenennenFehler {
    #[error("die Anfrage ist ungültig")]
    UngültigeAnfrage,
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("diese Runde gehört nicht zu diesem Benutzer")]
    NichtErlaubt,
    #[error("die Runde wurde nicht gefunden")]
    NichtGefunden,
    #[error("das Ziel wurde in dieser Runde nicht gefunden")]
    ZielNichtGefunden,
    #[error("diese Runde ist keine freie Runde")]
    RundeIstKeineFreieRunde,
    #[error("die Runde ist nicht mehr in Bearbeitung")]
    RundeNichtInBearbeitung,
    #[error("die Runde konnte nicht geladen werden")]
    RundeLaden,
    #[error("die Runde konnte nicht gespeichert werden")]
    Speichern,
}

/// Rename a target in the caller's own running `Freie Runde`.
#[derive(Debug, Clone)]
pub struct RundeZielUmbenennen {
    pub runde_id: RundeId,
    pub ziel_id: RundenZielId,
    pub name: NonEmptyString,
}

#[async_trait]
impl UseCase<Runde> for RundeZielUmbenennen {
    type Error = Report<RundeZielUmbenennenFehler>;

    #[instrument(skip_all, fields(runde_id = ?self.runde_id, ziel_id = ?self.ziel_id), err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Runde, Self::Error> {
        let benutzer_id = ctx
            .auth()
            .require_benutzer_id()
            .change_context(RundeZielUmbenennenFehler::NichtAngemeldet)?;
        let uow = ctx
            .enter()
            .await
            .change_context(RundeZielUmbenennenFehler::RundeLaden)?;

        let ergebnis = async {
            let mut runde = laden_umbenennen(&uow, self.runde_id).await?;
            if !runde.gehört_zu(benutzer_id) {
                return Err(Report::new(RundeZielUmbenennenFehler::NichtErlaubt));
            }
            if !runde.ist_freie_runde() {
                return Err(Report::new(
                    RundeZielUmbenennenFehler::RundeIstKeineFreieRunde,
                ));
            }
            if !runde.ist_in_bearbeitung() {
                return Err(Report::new(
                    RundeZielUmbenennenFehler::RundeNichtInBearbeitung,
                ));
            }
            if !runde.ziel_umbenennen(self.ziel_id, self.name) {
                return Err(Report::new(RundeZielUmbenennenFehler::ZielNichtGefunden));
            }

            uow.runden()
                .update(&mut runde)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RundeZielUmbenennenFehler::Speichern)?;
            Ok(runde.into_data())
        }
        .await;

        let runde = uow
            .finish(ergebnis, RundeZielUmbenennenFehler::RundeLaden)
            .await?;
        info!(id = ?runde.id(), ziel_id = ?self.ziel_id, "freies ziel umbenannt");
        Ok(runde)
    }
}

async fn laden_umbenennen(
    uow: &UnitOfWork<'_>,
    runde_id: RundeId,
) -> Result<Versioned<Runde>, Report<RundeZielUmbenennenFehler>> {
    match uow.runden().find_by_id(runde_id).await {
        Ok(runde) => Ok(runde),
        Err(fehler) => Err(match fehler.current_context() {
            RepositoryError::NotFound => Report::new(RundeZielUmbenennenFehler::NichtGefunden),
            _ => fehler.change_context(RundeZielUmbenennenFehler::RundeLaden),
        }),
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
