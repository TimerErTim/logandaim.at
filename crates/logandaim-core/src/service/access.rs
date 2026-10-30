//! The permission model, shared by every use case and by the API facade's own reads.
//!
//! Authorisation is always decided from the `Benutzer` and the `Anlage` a caller loaded
//! (NFR-2, SPEC-konto rule 17): no table, no middleware, no role string handed in by an
//! endpoint. The predicates below are the whole model, so a read the facade runs inline
//! and a write a use case runs answer the same question the same way.

use error_stack::{Report, ResultExt};

use crate::{
    application::{
        ExecutionContext,
        uow::{UnitOfWork, Versioned},
    },
    domain::{
        Anlage, AnlageId, AnlageVerwaltungRolle,
        konto::{Benutzer, BenutzerId},
    },
    ports::RepositoryError,
    service::StammdatenFehler,
};

/// Whether a Benutzer may maintain this Anlage's Parkour, Ziele, and Pflöcke (rule 10).
pub fn darf_masterdaten_pflegen(benutzer: &Benutzer, anlage: &Anlage) -> bool {
    benutzer.ist_systemadministrator()
        || anlage
            .rolle_von(benutzer.id())
            .is_some_and(AnlageVerwaltungRolle::darf_masterdaten_pflegen)
}

/// Whether a Benutzer may edit the Anlage's own fields, deactivate it, or manage its
/// `AnlageVerwaltungen` (rule 2, rule 10).
pub fn darf_anlage_verwalten(benutzer: &Benutzer, anlage: &Anlage) -> bool {
    benutzer.ist_systemadministrator()
        || anlage
            .rolle_von(benutzer.id())
            .is_some_and(AnlageVerwaltungRolle::darf_anlage_verwalten)
}

/// Whether a Benutzer may read this Anlage at all.
///
/// An archived Anlage is closed to everyone but a `Systemadministrator`: its history stays
/// reachable through the Runden, not through an admin surface (rule 11, AC-11).
pub fn darf_anlage_lesen(benutzer: &Benutzer, anlage: &Anlage) -> bool {
    !anlage.ist_archiviert() || benutzer.ist_systemadministrator()
}

/// Whether a repository report is a missing row.
pub fn ist_nicht_gefunden<C>(fehler: &Report<C>) -> bool
where
    C: std::error::Error + Send + Sync + 'static,
{
    fehler
        .downcast_ref::<RepositoryError>()
        .is_some_and(|error| matches!(error, RepositoryError::NotFound))
}

/// The signed-in identity, or `NichtAngemeldet`.
pub fn anmeldung(ctx: &ExecutionContext<'_>) -> Result<BenutzerId, Report<StammdatenFehler>> {
    ctx.auth()
        .require_benutzer_id()
        .change_context(StammdatenFehler::NichtAngemeldet)
}

/// Load the caller's own Benutzer row, or `BenutzerNichtGefunden`.
pub async fn benutzer_laden(
    uow: &UnitOfWork<'_>,
    benutzer_id: BenutzerId,
) -> Result<Versioned<Benutzer>, Report<StammdatenFehler>> {
    match uow.benutzer().find_by_id(benutzer_id).await {
        Ok(benutzer) => Ok(benutzer),
        Err(fehler) if ist_nicht_gefunden(&fehler) => {
            Err(Report::new(StammdatenFehler::BenutzerNichtGefunden))
        }
        Err(fehler) => Err(fehler).change_context(StammdatenFehler::BenutzerLaden),
    }
}

/// Load one Anlage. Archived Anlagen load fine: who may read them is
/// [`darf_anlage_lesen`]'s decision, not the loader's.
pub async fn anlage_laden(
    uow: &UnitOfWork<'_>,
    anlage_id: AnlageId,
) -> Result<Versioned<Anlage>, Report<StammdatenFehler>> {
    match uow.anlagen().find_by_id(anlage_id).await {
        Ok(anlage) => Ok(anlage),
        Err(fehler) if ist_nicht_gefunden(&fehler) => {
            Err(Report::new(StammdatenFehler::NichtGefunden))
        }
        Err(fehler) => Err(fehler).change_context(StammdatenFehler::AnlageLaden),
    }
}
