//! Starting, reading, and closing a Runde.
//!
//! `RundeStarten` captures the Parcours setup as a value copy inside the same transaction that
//! creates the Runde, so no later master-data edit can reach it (SPEC-stammdaten AC-7/AC-9).
//! `RundeAnzeigen` is owner-or-Systemadministrator and returns the snapshot together with both
//! Wertungen, because the Wertung is a read of a Runde and has no endpoint of its own
//! (SPEC-wertung rule 6).

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use tracing::info;

use crate::{
    application::ExecutionContext,
    domain::{
        Anlage, NeueRunde, ParcoursId, Runde, RundeId, Schuss, Schussmenge, ZielId, Zielergebnis,
        Zuordnung,
    },
    ports::RepositoryError,
    service::UseCase,
};

fn ist_nicht_gefunden<C>(fehler: &Report<C>) -> bool
where
    C: std::error::Error + Send + Sync + 'static,
{
    fehler
        .downcast_ref::<RepositoryError>()
        .is_some_and(|error| matches!(error, RepositoryError::NotFound))
}

// ---------------------------------------------------------------------------------------
// RundeStarten
// ---------------------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum RundeStartenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("der Parcours wurde nicht gefunden")]
    NichtGefunden,
    #[error("der Parcours oder seine Anlage ist nicht mehr aktiv")]
    NichtAktiv,
    #[error("die Anlage konnte nicht geladen werden")]
    AnlageLaden,
    #[error("die Runde konnte nicht gestartet werden")]
    Starten,
}

/// Start a Runde at one active Parcours of one active Anlage.
#[derive(Debug, Clone)]
pub struct RundeStarten {
    pub parcours_id: ParcoursId,
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
            // A Parcours id alone does not say which aggregate holds it, so active Anlagen are
            // searched. Fine for a handful of pilot sites; a read-model port is the escape hatch.
            let anlagen = uow
                .anlagen()
                .find_aktiv()
                .await
                .map_err(IntoReport::into_report)
                .change_context(RundeStartenFehler::AnlageLaden)?;

            let mut gefunden = None;
            let mut existiert = false;
            for anlage in &anlagen {
                let Anlage::Aktiv(aktiv) = &**anlage else {
                    continue;
                };
                let Some(parcours) = aktiv.parcours_mit_id(self.parcours_id) else {
                    continue;
                };
                existiert = true;
                if parcours.ist_archiviert() {
                    continue;
                }
                gefunden = Some((anlage, parcours));
                break;
            }

            let Some((anlage, parcours)) = gefunden else {
                // "No such Parcours" is 404; "exists but archived" is 409.
                return Err(Report::new(if existiert {
                    RundeStartenFehler::NichtAktiv
                } else {
                    RundeStartenFehler::NichtGefunden
                }));
            };

            // AC-7/AC-9: one value copy, taken in this transaction.
            let Some(snapshot) = anlage.snapshot_für(parcours.id()) else {
                return Err(Report::new(RundeStartenFehler::NichtGefunden));
            };
            let anlage_id = anlage.id();

            uow.runden()
                .create(NeueRunde::neu(
                    anlage_id,
                    Zuordnung::Benutzer(benutzer_id),
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
            parcours_id = ?runde.parcours_id(),
            "runde gestartet"
        );
        Ok(runde)
    }
}

// ---------------------------------------------------------------------------------------
// RundeAnzeigen
// ---------------------------------------------------------------------------------------

/// One captured Ziel's setup plus its folded result, if any Schuss was logged.
#[derive(Debug, Clone)]
pub struct Zielansicht {
    pub ziel_id: ZielId,
    pub tiername: String,
    pub zielgruppe: u8,
    /// Pflock label to metres, for the distances captured for this Ziel.
    pub abstände: Vec<(String, u32)>,
    /// `None` for a Ziel with no Schuss yet.
    pub ergebnis: Option<Zielergebnis>,
}

/// The whole read model of one Runde: who, where, the frozen setup, and both Wertungen.
#[derive(Debug, Clone)]
pub struct RundeAnsicht {
    pub runde: Runde,
    pub ziele: Vec<Zielansicht>,
    pub gesamt_erster_treffer: u32,
    pub gesamt_mehrschuss: u32,
    /// How many captured Ziele contributed a result.
    pub ziele_mit_ergebnis: usize,
    pub schüsse: Vec<Schuss>,
}

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

/// Read one Runde: its owner, or a `Systemadministrator` (AC-13).
#[derive(Debug, Clone)]
pub struct RundeAnzeigen {
    pub runde_id: RundeId,
}

#[async_trait]
impl UseCase<RundeAnsicht> for RundeAnzeigen {
    type Error = Report<RundeAnzeigenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<RundeAnsicht, Self::Error> {
        let benutzer_id = ctx
            .auth()
            .require_benutzer_id()
            .change_context(RundeAnzeigenFehler::NichtAngemeldet)?;

        let uow = ctx
            .enter()
            .await
            .change_context(RundeAnzeigenFehler::RundeLaden)?;

        let ergebnis = async {
            let runde = match uow.runden().find_by_id(self.runde_id).await {
                Ok(runde) => runde,
                Err(fehler) if ist_nicht_gefunden(&fehler) => {
                    return Err(Report::new(RundeAnzeigenFehler::NichtGefunden));
                }
                Err(fehler) => {
                    return Err(fehler).change_context(RundeAnzeigenFehler::RundeLaden);
                }
            };

            // Owner or Systemadministrator. An Anlage's staff never reads another Benutzer's
            // Runde (AC-13), which is why the AnlageVerwaltung grants nothing here.
            if !runde.gehört_zu(benutzer_id) {
                let benutzer = uow
                    .benutzer()
                    .find_by_id(benutzer_id)
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(RundeAnzeigenFehler::BenutzerLaden)?;
                if !benutzer.ist_systemadministrator() {
                    return Err(Report::new(RundeAnzeigenFehler::NichtErlaubt));
                }
            }

            let schüsse: Vec<Schuss> = uow
                .schüsse()
                .find_by_runde(self.runde_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RundeAnzeigenFehler::SchüsseLaden)?
                .into_iter()
                .map(|schuss| schuss.into_data())
                .collect();

            // One Schussmenge per captured Ziel; the fold over the route is the Runde's.
            let mengen: Vec<Schussmenge> = runde
                .snapshot()
                .ziele()
                .iter()
                .map(|ziel| {
                    Schussmenge::für_ziel(
                        ziel.ziel_id(),
                        schüsse
                            .iter()
                            .filter(|schuss| schuss.ziel_id() == ziel.ziel_id())
                            .cloned(),
                    )
                })
                .collect();

            let ziele = runde
                .snapshot()
                .ziele()
                .iter()
                .map(|ziel| Zielansicht {
                    ziel_id: ziel.ziel_id(),
                    tiername: ziel.tiername().to_owned(),
                    zielgruppe: ziel.zielgruppe(),
                    abstände: ziel.abstände().to_vec(),
                    ergebnis: mengen
                        .iter()
                        .find(|menge| menge.ziel_id() == ziel.ziel_id())
                        .and_then(Schussmenge::ergebnis),
                })
                .collect();

            let gesamt = runde.ergebnis(&mengen);

            Ok(RundeAnsicht {
                runde: runde.cloned_data(),
                ziele,
                gesamt_erster_treffer: gesamt.erster_treffer().value(),
                gesamt_mehrschuss: gesamt.mehrschuss().value(),
                ziele_mit_ergebnis: gesamt.ziele_mit_ergebnis(),
                schüsse,
            })
        }
        .await;

        uow.finish(ergebnis, RundeAnzeigenFehler::RundeLaden).await
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
                Err(fehler) if ist_nicht_gefunden(&fehler) => {
                    return Err(Report::new(RundeAbschliessenFehler::NichtGefunden));
                }
                Err(fehler) => {
                    return Err(fehler).change_context(RundeAbschliessenFehler::RundeLaden);
                }
            };

            if !runde.gehört_zu(benutzer_id) {
                return Err(Report::new(RundeAbschliessenFehler::NichtErlaubt));
            }
            runde
                .abschliessen(jetzt)
                .change_context(RundeAbschliessenFehler::BereitsAbgeschlossen)?;

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
// RundenAnzeigen (own list)
// ---------------------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum RundenAnzeigenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("die Runden konnten nicht geladen werden")]
    Laden,
}

/// The caller's own Runden, newest first.
#[derive(Debug, Clone)]
pub struct RundenAnzeigen;

#[async_trait]
impl UseCase<Vec<Runde>> for RundenAnzeigen {
    type Error = Report<RundenAnzeigenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Vec<Runde>, Self::Error> {
        let benutzer_id = ctx
            .auth()
            .require_benutzer_id()
            .change_context(RundenAnzeigenFehler::NichtAngemeldet)?;

        let uow = ctx
            .enter()
            .await
            .change_context(RundenAnzeigenFehler::Laden)?;

        let ergebnis = async {
            let mut runden: Vec<Runde> = uow
                .runden()
                .find_by_benutzer(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RundenAnzeigenFehler::Laden)?
                .into_iter()
                .map(|runde| runde.into_data())
                .collect();
            runden.sort_by_key(|a| std::cmp::Reverse(a.begonnen_am()));
            Ok(runden)
        }
        .await;

        uow.finish(ergebnis, RundenAnzeigenFehler::Laden).await
    }
}
