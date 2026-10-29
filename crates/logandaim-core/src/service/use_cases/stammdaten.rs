//! Master data: the `Anlage` and everything it owns, plus the authority to maintain it.
//!
//! Every use case decides authorisation itself, from the `Benutzer` and the `Anlage` it
//! loads (NFR-2, SPEC-konto rule 17). The predicates in [`crate::service::access`] are the
//! whole permission model: no table, no middleware, no role string handed in by an endpoint.
//! They live there, not here, because a read the API facade runs inline must answer the same
//! question the same way a write does.
//!
//! Removal follows AC-11/AC-12: master data still referenced by a logged or in-progress Runde
//! is archived, never destroyed; master data with no reference is deleted outright. Archiving
//! is a domain mutation, so the decision is a pure index lookup followed by `update` on the
//! root aggregate.

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};
use tracing::info;
use uuid::Uuid;

use crate::{
    application::{ExecutionContext, uow::Versioned},
    domain::{
        Anlage, AnlageAdresse, AnlageAktiv, AnlageFehler, AnlageId, AnlageKontakt, AnlageName,
        AnlageVerwaltungRolle, NeueAnlage, NeuerParcours, NeuesZiel, Parcours, ParcoursFehler,
        ParcoursId, ParcoursName, Pflock, Tiername, ZielFehler, ZielId, Zielgruppe,
        Zielplatzierung, Zielverwendung, konto::BenutzerId,
    },
    service::{
        UseCase, anlage_laden, anmeldung, benutzer_laden, darf_anlage_verwalten,
        darf_masterdaten_pflegen,
    },
};

// ---------------------------------------------------------------------------------------
// Shared errors and plumbing
// ---------------------------------------------------------------------------------------

/// The refusals of every master-data write.
///
/// Each variant has exactly one HTTP meaning (401, 403, 400, 404, 409), mapped in
/// `logandaim-api/src/errors/http_status.rs`.
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

impl From<ParcoursFehler> for StammdatenFehler {
    fn from(_: ParcoursFehler) -> Self {
        Self::UngültigeAnfrage
    }
}

impl From<ZielFehler> for StammdatenFehler {
    fn from(_: ZielFehler) -> Self {
        Self::UngültigeAnfrage
    }
}

/// Load an Anlage that must still be active, or report 404 / 409.
async fn aktive_anlage(
    uow: &crate::application::uow::UnitOfWork<'_>,
    anlage_id: AnlageId,
) -> Result<Versioned<Anlage>, Report<StammdatenFehler>> {
    let anlage = anlage_laden(uow, anlage_id).await?;
    if anlage.ist_archiviert() {
        return Err(Report::new(StammdatenFehler::Archiviert));
    }
    Ok(anlage)
}

/// Run one mutation against the active typestate, then write the whole aggregate back.
///
/// Every child write goes through the root, guarded by the root `_version` (AC-11: child rows
/// are replaced inside the root's transaction).
async fn aktive_anlage_ändern<F>(
    uow: &crate::application::uow::UnitOfWork<'_>,
    anlage_id: AnlageId,
    f: F,
) -> Result<Anlage, Report<StammdatenFehler>>
where
    F: FnOnce(&mut AnlageAktiv) -> Result<(), Report<StammdatenFehler>>,
{
    let mut anlage = aktive_anlage(uow, anlage_id).await?;
    let Anlage::Aktiv(aktiv) = &mut *anlage else {
        return Err(Report::new(StammdatenFehler::Archiviert));
    };
    f(aktiv)?;
    uow.anlagen()
        .update(&mut anlage)
        .await
        .map_err(IntoReport::into_report)
        .change_context(StammdatenFehler::AnlageSpeichern)?;
    Ok(anlage.cloned_data())
}

/// The caller, authorised for master data at this active Anlage.
async fn berechtigt_für_masterdaten(
    ctx: &ExecutionContext<'_>,
    uow: &crate::application::uow::UnitOfWork<'_>,
    anlage_id: AnlageId,
) -> Result<(BenutzerId, Versioned<Anlage>), Report<StammdatenFehler>> {
    let benutzer_id = anmeldung(ctx)?;
    let benutzer = benutzer_laden(uow, benutzer_id).await?;
    let anlage = aktive_anlage(uow, anlage_id).await?;
    if !darf_masterdaten_pflegen(&benutzer, &anlage, benutzer_id) {
        return Err(Report::new(StammdatenFehler::NichtErlaubt));
    }
    Ok((benutzer_id, anlage))
}

/// The caller, authorised to manage this active Anlage (fields, lifecycle, Verwaltungen).
async fn berechtigt_für_anlage(
    ctx: &ExecutionContext<'_>,
    uow: &crate::application::uow::UnitOfWork<'_>,
    anlage_id: AnlageId,
) -> Result<BenutzerId, Report<StammdatenFehler>> {
    let benutzer_id = anmeldung(ctx)?;
    let benutzer = benutzer_laden(uow, benutzer_id).await?;
    let anlage = aktive_anlage(uow, anlage_id).await?;
    if !darf_anlage_verwalten(&benutzer, &anlage, benutzer_id) {
        return Err(Report::new(StammdatenFehler::NichtErlaubt));
    }
    Ok(benutzer_id)
}

// ---------------------------------------------------------------------------------------
// Views
// ---------------------------------------------------------------------------------------

/// One Anlage with the placement aggregation its admin page renders.
#[derive(Debug, Clone)]
pub struct AnlageAnsicht {
    pub anlage: Anlage,
    /// One entry per Ziel, listing every Parcours that currently places it.
    pub zielverwendungen: Vec<(ZielId, Vec<Zielverwendung>)>,
}

impl AnlageAnsicht {
    /// Build the admin view: the Anlage plus its per-Ziel placement aggregation.
    pub fn neu(anlage: Anlage) -> Self {
        let zielverwendungen = anlage
            .ziele()
            .iter()
            .map(|ziel| (ziel.id(), anlage.ziel_verwendungen(ziel.id())))
            .collect();
        Self {
            anlage,
            zielverwendungen,
        }
    }
}

/// One row of the admin list, with the caller's role so the UI can gate its controls.
#[derive(Debug, Clone)]
pub struct AnlagenZeile {
    pub anlage: Anlage,
    pub rolle: Option<AnlageVerwaltungRolle>,
    pub ist_systemadministrator: bool,
}

// ---------------------------------------------------------------------------------------
// Anlage
// ---------------------------------------------------------------------------------------

/// Create an Anlage. Only a `Systemadministrator` may (rule 2, AC-1).
#[derive(Debug, Clone)]
pub struct AnlageErstellen {
    pub name: AnlageName,
    pub adresse: AnlageAdresse,
    pub kontakt: AnlageKontakt,
}

#[async_trait]
impl UseCase<Anlage> for AnlageErstellen {
    type Error = Report<StammdatenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageAnlegen)?;

        let ergebnis = async {
            let benutzer_id = anmeldung(&ctx)?;
            let benutzer = benutzer_laden(&uow, benutzer_id).await?;
            if !benutzer.ist_systemadministrator() {
                return Err(Report::new(StammdatenFehler::NichtErlaubt));
            }

            uow.anlagen()
                .create(NeueAnlage::neu(self.name, self.adresse, self.kontakt))
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageAnlegen)
                .map(Versioned::into_data)
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
    pub name: AnlageName,
    pub adresse: AnlageAdresse,
    pub kontakt: AnlageKontakt,
}

#[async_trait]
impl UseCase<Anlage> for AnlageBeschreibendeDatenÄndern {
    type Error = Report<StammdatenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_anlage(&ctx, &uow, self.anlage_id).await?;
            let (name, adresse, kontakt) = (self.name, self.adresse, self.kontakt);
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                aktiv.beschreibende_daten_ändern(name, adresse, kontakt);
                Ok(())
            })
            .await
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

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let jetzt = ctx.clock().now();
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_anlage(&ctx, &uow, self.anlage_id).await?;

            let mut anlage = aktive_anlage(&uow, self.anlage_id).await?;
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

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_anlage(&ctx, &uow, self.anlage_id).await?;
            // A Verwaltung for an identity that does not exist would be a dangling grant.
            if benutzer_laden(&uow, self.benutzer_id).await.is_err() {
                return Err(Report::new(StammdatenFehler::BenutzerNichtGefunden));
            }
            let (benutzer_id, rolle) = (self.benutzer_id, self.rolle);
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                aktiv
                    .verwaltung_gewähren(benutzer_id, rolle)
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
                Ok(())
            })
            .await
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

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_anlage(&ctx, &uow, self.anlage_id).await?;
            let (benutzer_id, rolle) = (self.benutzer_id, self.rolle);
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                aktiv
                    .verwaltung_ändern(benutzer_id, rolle)
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
                Ok(())
            })
            .await
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

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_anlage(&ctx, &uow, self.anlage_id).await?;
            let benutzer_id = self.benutzer_id;
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                aktiv
                    .verwaltung_entziehen(benutzer_id)
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
                Ok(())
            })
            .await
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
    pub tiername: Tiername,
    pub zielgruppe: Zielgruppe,
}

#[async_trait]
impl UseCase<Anlage> for ZielHinzufügen {
    type Error = Report<StammdatenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_masterdaten(&ctx, &uow, self.anlage_id).await?;
            let neu = NeuesZiel::neu(self.tiername, self.zielgruppe);
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                aktiv.ziel_hinzufügen(ZielId(Uuid::new_v4()), neu);
                Ok(())
            })
            .await
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Edit a Ziel's animal name and target group. Its identity does not change (AC-5).
#[derive(Debug, Clone)]
pub struct ZielÄndern {
    pub anlage_id: AnlageId,
    pub ziel_id: ZielId,
    pub tiername: Tiername,
    pub zielgruppe: Zielgruppe,
}

#[async_trait]
impl UseCase<Anlage> for ZielÄndern {
    type Error = Report<StammdatenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_masterdaten(&ctx, &uow, self.anlage_id).await?;
            let (ziel_id, tiername, zielgruppe) = (self.ziel_id, self.tiername, self.zielgruppe);
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                aktiv
                    .ziel_ändern(ziel_id, tiername, zielgruppe)
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
                Ok(())
            })
            .await
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
    pub ziel_id: ZielId,
    pub tiername: Tiername,
    pub zielgruppe: Zielgruppe,
}

#[async_trait]
impl UseCase<Anlage> for ZielErsetzen {
    type Error = Report<StammdatenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_masterdaten(&ctx, &uow, self.anlage_id).await?;
            let (ziel_id, tiername, zielgruppe) = (self.ziel_id, self.tiername, self.zielgruppe);
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                let neu = ZielId(Uuid::new_v4());
                aktiv
                    .ziel_ersetzen(ziel_id, neu, NeuesZiel::neu(tiername, zielgruppe))
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
                Ok(())
            })
            .await
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
    pub ziel_id: ZielId,
}

#[async_trait]
impl UseCase<Anlage> for ZielEntfernen {
    type Error = Report<StammdatenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_masterdaten(&ctx, &uow, self.anlage_id).await?;
            let referenziert = uow
                .runden()
                .exists_referencing_ziel(self.ziel_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            let ziel_id = self.ziel_id;
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                if referenziert {
                    aktiv
                        .ziel_archivieren(ziel_id)
                        .change_context(StammdatenFehler::UngültigeAnfrage)
                } else {
                    aktiv
                        .ziel_entfernen(ziel_id)
                        .change_context(StammdatenFehler::UngültigeAnfrage)
                }
            })
            .await
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

// ---------------------------------------------------------------------------------------
// Parcours
// ---------------------------------------------------------------------------------------

/// Add a Parcours with its Pflock setup and route (FR-3, AC-4, AC-6).
#[derive(Debug, Clone)]
pub struct ParcoursHinzufügen {
    pub anlage_id: AnlageId,
    pub name: ParcoursName,
    pub pflöcke: Vec<Pflock>,
    pub platzierungen: Vec<Zielplatzierung>,
}

#[async_trait]
impl UseCase<Anlage> for ParcoursHinzufügen {
    type Error = Report<StammdatenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_masterdaten(&ctx, &uow, self.anlage_id).await?;
            let neu = NeuerParcours::neu(self.name, self.pflöcke, self.platzierungen)
                .change_context(StammdatenFehler::UngültigeAnfrage)?;
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                aktiv
                    .parcours_hinzufügen(ParcoursId(Uuid::new_v4()), neu)
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
                Ok(())
            })
            .await
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Replace a Parcours' whole shooting setup (FR-3, AC-6/AC-7).
///
/// Only master data changes: an existing Runde keeps its captured snapshot (NFR-3).
#[derive(Debug, Clone)]
pub struct ParcoursSetupÄndern {
    pub anlage_id: AnlageId,
    pub parcours_id: ParcoursId,
    pub pflöcke: Vec<Pflock>,
    pub platzierungen: Vec<Zielplatzierung>,
}

#[async_trait]
impl UseCase<Anlage> for ParcoursSetupÄndern {
    type Error = Report<StammdatenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_masterdaten(&ctx, &uow, self.anlage_id).await?;
            let (parcours_id, pflöcke, platzierungen) =
                (self.parcours_id, self.pflöcke, self.platzierungen);
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                aktiv
                    .parcours_setup_ändern(parcours_id, pflöcke, platzierungen)
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
                Ok(())
            })
            .await
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Rename a Parcours. Its identity, and every snapshot naming it, stay unchanged.
#[derive(Debug, Clone)]
pub struct ParcoursUmbenennen {
    pub anlage_id: AnlageId,
    pub parcours_id: ParcoursId,
    pub name: ParcoursName,
}

#[async_trait]
impl UseCase<Anlage> for ParcoursUmbenennen {
    type Error = Report<StammdatenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_masterdaten(&ctx, &uow, self.anlage_id).await?;
            let (parcours_id, name) = (self.parcours_id, self.name);
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                aktiv
                    .parcours_umbenennen(parcours_id, name)
                    .change_context(StammdatenFehler::UngültigeAnfrage)?;
                Ok(())
            })
            .await
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Remove a Parcours: archive it when a Runde captured it, delete it when none did.
#[derive(Debug, Clone)]
pub struct ParcoursEntfernen {
    pub anlage_id: AnlageId,
    pub parcours_id: ParcoursId,
}

#[async_trait]
impl UseCase<Anlage> for ParcoursEntfernen {
    type Error = Report<StammdatenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Anlage, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(StammdatenFehler::AnlageLaden)?;

        let ergebnis = async {
            berechtigt_für_masterdaten(&ctx, &uow, self.anlage_id).await?;
            let referenziert = uow
                .runden()
                .exists_referencing_parcours(self.parcours_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(StammdatenFehler::AnlageLaden)?;
            let parcours_id = self.parcours_id;
            aktive_anlage_ändern(&uow, self.anlage_id, |aktiv| {
                if referenziert {
                    aktiv
                        .parcours_archivieren(parcours_id)
                        .change_context(StammdatenFehler::UngültigeAnfrage)
                } else {
                    aktiv
                        .parcours_entfernen(parcours_id)
                        .change_context(StammdatenFehler::UngültigeAnfrage)
                }
            })
            .await
        }
        .await;

        uow.finish(ergebnis, StammdatenFehler::AnlageLaden).await
    }
}

/// Public read model: one active Parcours with its Anlage, for selecting a Runde.
#[derive(Debug, Clone)]
pub struct ParcoursÖffentlich {
    pub anlage_id: AnlageId,
    pub anlage_name: AnlageName,
    pub parcours: Parcours,
}

/// One active Parcours as the public Anlagen directory names it.
#[derive(Debug, Clone)]
pub struct ParcoursKurz {
    pub id: ParcoursId,
    pub name: ParcoursName,
    pub anzahl_pflöcke: usize,
    pub anzahl_ziele: usize,
}

/// One active Anlage in the public directory: where it is, and which Parcours it offers.
///
/// Deliberately no contact data: email and phone are personal data, and no spec approved
/// publishing them.
#[derive(Debug, Clone)]
pub struct AnlageÖffentlich {
    pub anlage_id: AnlageId,
    pub name: AnlageName,
    pub ort: String,
    pub ländercode: crate::domain::Ländercode,
    pub parcours: Vec<ParcoursKurz>,
}
