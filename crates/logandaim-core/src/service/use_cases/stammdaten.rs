//! Master data: the `Anlage` and everything it owns.
//!
//! Writes authorise themselves from the loaded `Benutzer` and `Anlage` (SPEC-konto rule 17);
//! reads run inline in the API facade. Removal archives what history references and deletes
//! the rest (AC-11/AC-12).
//!
//! Each write loads what it needs and keeps it: there is no loader helper to call, because a
//! caller that already holds a `Benutzer` and an `Anlage` must not pay for a second read.
//! Read models are the API's: the shape a page renders is defined where it is served.

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use std::num::NonZeroU32;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    application::ExecutionContext,
    domain::{
        Anlage, AnlageAdresse, AnlageFehler, AnlageId, AnlageKontakt, AnlageVerwaltungRolle,
        AnlageZielId, NeueAnlage, NeuerParkour, NonEmptyString, ParkourFehler, ParkourId,
        ParkourPflock, ZielFehler, Zielplatzierung, konto::BenutzerId,
    },
    ports::RepositoryError,
    service::{UseCase, darf_anlage_verwalten, darf_masterdaten_pflegen},
};

// ---------------------------------------------------------------------------------------
// Shared errors and plumbing
// ---------------------------------------------------------------------------------------

/// The refusals of every master-data write.
#[derive(Debug, thiserror::Error)]
pub enum StammdatenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("dieser Benutzer darf diese Aktion nicht ausführen")]
    NichtErlaubt,
    #[error("die Anfrage ist ungültig")]
    UngültigeAnfrage,
    #[error("die Anlage wurde nicht gefunden")]
    NichtGefunden,
    #[error("die Anlage ist deaktiviert und kann nicht mehr geändert werden")]
    Archiviert,
    #[error("der Benutzer wurde nicht gefunden")]
    BenutzerNichtGefunden,
    #[error("die Anlage konnte nicht geladen werden")]
    AnlageLaden,
    #[error("die Anlage konnte nicht gespeichert werden")]
    AnlageSpeichern,
    #[error("die Anlage konnte nicht angelegt werden")]
    AnlageAnlegen,
    #[error("der Benutzer konnte nicht geladen werden")]
    BenutzerLaden,
}

impl From<AnlageFehler> for StammdatenFehler {
    fn from(_: AnlageFehler) -> Self {
        Self::UngültigeAnfrage
    }
}

impl From<ParkourFehler> for StammdatenFehler {
    fn from(_: ParkourFehler) -> Self {
        Self::UngültigeAnfrage
    }
}

impl From<ZielFehler> for StammdatenFehler {
    fn from(_: ZielFehler) -> Self {
        Self::UngültigeAnfrage
    }
}

// ---------------------------------------------------------------------------------------
// Anlage
// ---------------------------------------------------------------------------------------

/// Create an Anlage. Only a `Systemadministrator` may (rule 2, AC-1).
#[derive(Debug, Clone)]
pub struct AnlageErstellen {
    pub name: NonEmptyString,
    pub adresse: AnlageAdresse,
    pub kontakt: AnlageKontakt,
}

#[async_trait]
impl UseCase<Anlage> for AnlageErstellen {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageAnlegen)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            if !benutzer.ist_systemadministrator() {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }

            uow.anlagen()
                .create(NeueAnlage::neu(self.name, self.adresse, self.kontakt))
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageAnlegen)
                .map(|anlage| anlage.into_data())
        }
        .await;

        let anlage = uow
            .finish(ergebnis, StammdatenFehler::AnlageAnlegen)
            .await?;
        info!(id = ?anlage.id(), "anlage angelegt");
        Ok(anlage)
    }
}

/// Edit the Anlage's name, address, and contact details (FR-1, rule 2).
#[derive(Debug, Clone)]
pub struct AnlageBeschreibendeDatenÄndern {
    pub anlage_id: AnlageId,
    pub name: NonEmptyString,
    pub adresse: AnlageAdresse,
    pub kontakt: AnlageKontakt,
}

#[async_trait]
impl UseCase<Anlage> for AnlageBeschreibendeDatenÄndern {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_anlage_verwalten(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }
            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            aktiv.beschreibende_daten_ändern(self.name, self.adresse, self.kontakt);

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Deactivate an Anlage. Irreversible; no Runde may start there afterwards (AC-10).
#[derive(Debug, Clone)]
pub struct AnlageDeaktivieren {
    pub anlage_id: AnlageId,
}

#[async_trait]
impl UseCase<Anlage> for AnlageDeaktivieren {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let jetzt = ctx.clock().now();
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_anlage_verwalten(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }
            let Anlage::Aktiv(aktiv) = anlage.cloned_data() else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            anlage.set_data(Anlage::Archiviert(aktiv.deaktiviere(jetzt)));

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        let anlage = uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await?;
        info!(id = ?anlage.id(), "anlage deaktiviert");
        Ok(anlage)
    }
}

// ---------------------------------------------------------------------------------------
// AnlageVerwaltung
// ---------------------------------------------------------------------------------------

/// Grant a role to an existing Benutzer (FR-2, AC-2).
#[derive(Debug, Clone)]
pub struct AnlageVerwaltungGewähren {
    pub anlage_id: AnlageId,
    pub benutzer_id: BenutzerId,
    pub rolle: AnlageVerwaltungRolle,
}

#[async_trait]
impl UseCase<Anlage> for AnlageVerwaltungGewähren {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_anlage_verwalten(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }
            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            // A Verwaltung for an identity that does not exist would be a dangling grant.
            if let Err(fehler) = uow.benutzer().find_by_id(self.benutzer_id).await {
                return Err(match fehler.current_context() {
                    RepositoryError::NotFound => {
                        Report::new(StammdatenFehler::BenutzerNichtGefunden)
                    }
                    _ => fehler.change_context(StammdatenFehler::BenutzerLaden),
                });
            }
            aktiv
                .verwaltung_gewähren(self.benutzer_id, self.rolle)
                .change_context(StammdatenFehler::UngültigeAnfrage)?;

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Change an existing Verwaltung's role (FR-2, AC-2).
#[derive(Debug, Clone)]
pub struct AnlageVerwaltungÄndern {
    pub anlage_id: AnlageId,
    pub benutzer_id: BenutzerId,
    pub rolle: AnlageVerwaltungRolle,
}

#[async_trait]
impl UseCase<Anlage> for AnlageVerwaltungÄndern {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_anlage_verwalten(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }
            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            aktiv
                .verwaltung_ändern(self.benutzer_id, self.rolle)
                .change_context(StammdatenFehler::UngültigeAnfrage)?;

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Revoke a Verwaltung. The last Administrator cannot be revoked (AC-2).
#[derive(Debug, Clone)]
pub struct AnlageVerwaltungEntziehen {
    pub anlage_id: AnlageId,
    pub benutzer_id: BenutzerId,
}

#[async_trait]
impl UseCase<Anlage> for AnlageVerwaltungEntziehen {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_anlage_verwalten(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }
            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            aktiv
                .verwaltung_entziehen(self.benutzer_id)
                .change_context(StammdatenFehler::UngültigeAnfrage)?;

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

// ---------------------------------------------------------------------------------------
// Ziel
// ---------------------------------------------------------------------------------------

/// Add a physical Ziel (FR-3, AC-5).
#[derive(Debug, Clone)]
pub struct ZielHinzufügen {
    pub anlage_id: AnlageId,
    pub tiername: NonEmptyString,
}

#[async_trait]
impl UseCase<Anlage> for ZielHinzufügen {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_masterdaten_pflegen(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }
            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            aktiv.ziel_hinzufügen(AnlageZielId(Uuid::new_v4()), self.tiername);

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Edit a Ziel's master data: its animal name and its ordinal. The identity does not change
/// (AC-5).
#[derive(Debug, Clone)]
pub struct ZielÄndern {
    pub anlage_id: AnlageId,
    pub ziel_id: AnlageZielId,
    pub tiername: NonEmptyString,
    /// 1-based ordinal; `NonZero` because "Ziel 0" cannot be printed or referred to.
    pub zielnummer: NonZeroU32,
}

#[async_trait]
impl UseCase<Anlage> for ZielÄndern {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_masterdaten_pflegen(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }
            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            aktiv
                .ziel_ändern(self.ziel_id, self.tiername, self.zielnummer)
                .change_context(StammdatenFehler::UngültigeAnfrage)?;

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Replace the physical target behind one Ziel identity (AC-5).
///
/// "Replacing the physical target creates a new Ziel identity; it does not inherit the old
/// target's history." So this archives the old identity and places a new one in the same route
/// positions with the same distances — the new target takes over the slot, not the history.
#[derive(Debug, Clone)]
pub struct ZielErsetzen {
    pub anlage_id: AnlageId,
    pub ziel_id: AnlageZielId,
    pub tiername: NonEmptyString,
}

#[async_trait]
impl UseCase<Anlage> for ZielErsetzen {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_masterdaten_pflegen(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }
            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            aktiv
                .ziel_ersetzen(self.ziel_id, AnlageZielId(Uuid::new_v4()), self.tiername)
                .change_context(StammdatenFehler::UngültigeAnfrage)?;

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Remove a Ziel: archive it when history references it, delete it when nothing does
/// (AC-11/AC-12).
#[derive(Debug, Clone)]
pub struct ZielEntfernen {
    pub anlage_id: AnlageId,
    pub ziel_id: AnlageZielId,
}

#[async_trait]
impl UseCase<Anlage> for ZielEntfernen {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_masterdaten_pflegen(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }

            let referenziert = uow
                .runden()
                .exists_referencing_ziel(self.ziel_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;

            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            if referenziert {
                aktiv
                    .ziel_archivieren(self.ziel_id)
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
            } else {
                aktiv
                    .ziel_entfernen(self.ziel_id)
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
            }

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

// ---------------------------------------------------------------------------------------
// Parkour
// ---------------------------------------------------------------------------------------

/// Add a Parkour with its Pflock setup and route (FR-3, AC-4, AC-6).
#[derive(Debug, Clone)]
pub struct ParkourHinzufügen {
    pub anlage_id: AnlageId,
    pub name: NonEmptyString,
    pub pflöcke: Vec<ParkourPflock>,
    pub platzierungen: Vec<Zielplatzierung>,
}

#[async_trait]
impl UseCase<Anlage> for ParkourHinzufügen {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_masterdaten_pflegen(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }
            let neu = NeuerParkour::neu(self.name, self.pflöcke, self.platzierungen)
                .change_context(StammdatenFehler::UngültigeAnfrage)?;
            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            aktiv
                .parkour_hinzufügen(ParkourId(Uuid::new_v4()), neu)
                .change_context(StammdatenFehler::UngültigeAnfrage)?;

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Replace a Parkour' whole shooting setup (FR-3, AC-6/AC-7).
///
/// Only master data changes: an existing Runde keeps its captured route (NFR-3).
#[derive(Debug, Clone)]
pub struct ParkourSetupÄndern {
    pub anlage_id: AnlageId,
    pub parkour_id: ParkourId,
    pub pflöcke: Vec<ParkourPflock>,
    pub platzierungen: Vec<Zielplatzierung>,
}

#[async_trait]
impl UseCase<Anlage> for ParkourSetupÄndern {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_masterdaten_pflegen(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }
            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            aktiv
                .parkour_setup_ändern(self.parkour_id, self.pflöcke, self.platzierungen)
                .change_context(StammdatenFehler::UngültigeAnfrage)?;

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Rename a Parkour. Its identity, and every round naming it, stay unchanged.
#[derive(Debug, Clone)]
pub struct ParkourUmbenennen {
    pub anlage_id: AnlageId,
    pub parkour_id: ParkourId,
    pub name: NonEmptyString,
}

#[async_trait]
impl UseCase<Anlage> for ParkourUmbenennen {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_masterdaten_pflegen(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }
            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            aktiv
                .parkour_umbenennen(self.parkour_id, self.name)
                .change_context(StammdatenFehler::UngültigeAnfrage)?;

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Remove a Parkour: archive it when a Runde captured it, delete it when none did.
#[derive(Debug, Clone)]
pub struct ParkourEntfernen {
    pub anlage_id: AnlageId,
    pub parkour_id: ParkourId,
}

#[async_trait]
impl UseCase<Anlage> for ParkourEntfernen {
    type Error = Report<StammdatenFehler>;

    #[instrument(skip_all, err(Debug))]
    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            let benutzer_id = ctx
                .auth()
                .require_benutzer_id()
                .change_context(StammdatenFehler::NichtAngemeldet)?;
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::BenutzerLaden)?;
            let mut anlage = uow
                .anlagen()
                .find_by_id(self.anlage_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            if !darf_masterdaten_pflegen(&benutzer, &anlage) {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }

            let referenziert = uow
                .runden()
                .exists_referencing_parkour(self.parkour_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;

            let Anlage::Aktiv(aktiv) = &mut *anlage else {
                return Err(Report::new(StammdatenFehler::Archiviert));
            };
            if referenziert {
                aktiv
                    .parkour_archivieren(self.parkour_id)
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
            } else {
                aktiv
                    .parkour_entfernen(self.parkour_id)
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
            }

            uow.anlagen()
                .update(&mut anlage)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageSpeichern)?;
            Ok(anlage.into_data())
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}
