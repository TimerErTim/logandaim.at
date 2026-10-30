//! Starting and closing a Runde.
//!
//! `RundeStarten` captures the Parkour setup as a value copy inside the same transaction that
//! creates the Runde, so no later master-data edit can reach it (SPEC-stammdaten AC-7/AC-9).
//! Reading a Runde lives in the API facade: its read model (`RundeAnsicht`, `Zielansicht`)
//! and the fold over `ZielSchüsse` stay here, but the transaction is opened by the caller,
//! because a read authorises from the owner-or-Systemadministrator rule and aggregates
//! nothing a use case would own.

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use std::num::NonZeroU32;
use tracing::info;

use crate::{
    application::ExecutionContext,
    domain::{
        Abstand, Anlage, Erfasser, Farbe, NeueRunde, NonEmptyString, ParkourId, Runde, RundeId,
        Schuss, ZielId, ZielSchüsse, Zielwertung,
    },
    service::{UseCase, ist_nicht_gefunden},
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
    pub parkour_id: ParkourId,
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
            // A Parkour id alone does not say which aggregate holds it, so active Anlagen are
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
                let Some(parkour) = aktiv.parkour_mit_id(self.parkour_id) else {
                    continue;
                };
                existiert = true;
                if parkour.ist_archiviert() {
                    continue;
                }
                gefunden = Some((anlage, parkour));
                break;
            }

            let Some((anlage, parkour)) = gefunden else {
                // "No such Parkour" is 404; "exists but archived" is 409.
                return Err(Report::new(if existiert {
                    RundeStartenFehler::NichtAktiv
                } else {
                    RundeStartenFehler::NichtGefunden
                }));
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
// RundeAnzeigen
// ---------------------------------------------------------------------------------------

/// One captured Ziel's setup plus its folded Wertung, if any Schuss was logged.
#[derive(Debug, Clone)]
pub struct Zielansicht {
    pub ziel_id: ZielId,
    pub tiername: NonEmptyString,
    pub zielnummer: NonZeroU32,
    pub abstände: Vec<(Farbe, Abstand)>,
    /// `None` for a Ziel with no Schuss yet.
    pub wertung: Option<Zielwertung>,
}

/// The whole read model of one Runde: who, where, the frozen setup, and both Wertungen.
#[derive(Debug, Clone)]
pub struct RundeAnsicht {
    pub runde: Runde,
    pub ziele: Vec<Zielansicht>,
    pub gesamt_erster_treffer: u32,
    pub gesamt_mehrschuss: u32,
    /// How many captured Ziele contributed a result.
    pub ziele_mit_wertung: usize,
    pub schüsse: Vec<Schuss>,
}

impl RundeAnsicht {
    /// Fold the loaded Schüsse over the captured route.
    ///
    /// The aggregation is domain code (`ZielSchüsse`, `Runde::wertung`), so a caller that
    /// loaded a Runde and its Schüsse never re-derives a Wertung at the API edge.
    pub fn neu(runde: Runde, schüsse: Vec<Schuss>) -> Self {
        let je_ziel: Vec<(ZielId, Option<Zielwertung>)> = runde
            .snapshot()
            .ziele()
            .iter()
            .map(|ziel| {
                let menge = ZielSchüsse::für_ziel(
                    ziel.ziel_id(),
                    schüsse
                        .iter()
                        .filter(|schuss| {
                            schuss.runde_id() == runde.id() && schuss.ziel_id() == ziel.ziel_id()
                        })
                        .cloned(),
                )
                .expect("jeder Schuss ist auf dieses Ziel gefiltert");
                (ziel.ziel_id(), menge.wertung())
            })
            .collect();

        let ziele = runde
            .snapshot()
            .ziele()
            .iter()
            .map(|ziel| Zielansicht {
                ziel_id: ziel.ziel_id(),
                tiername: ziel.tiername().clone(),
                zielnummer: ziel.zielnummer(),
                abstände: ziel.abstände().to_vec(),
                wertung: je_ziel
                    .iter()
                    .find(|(id, _)| *id == ziel.ziel_id())
                    .and_then(|(_, wertung)| *wertung),
            })
            .collect();

        let gesamt = runde.wertung(schüsse.iter().cloned());
        Self {
            runde,
            ziele,
            gesamt_erster_treffer: gesamt.erster_treffer(),
            gesamt_mehrschuss: gesamt.mehrschuss(),
            ziele_mit_wertung: gesamt.ziele_mit_wertung(),
            schüsse,
        }
    }
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
// RundenAnzeigen read model
// ---------------------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum RundenAnzeigenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("die Runden konnten nicht geladen werden")]
    Laden,
}

/// Sort the caller's own Runden newest first.
///
/// The order is part of the read's contract, so it is spelled here rather than left to a
/// repository's `ORDER BY` that only one adapter happens to have.
pub fn runden_neueste_zuerst(mut runden: Vec<Runde>) -> Vec<Runde> {
    runden.sort_by_key(|runde| std::cmp::Reverse(runde.begonnen_am()));
    runden
}
