//! Reading, deleting, and bootstrapping a Benutzer.
//!
//! One error enum per use case. Authorisation is decided in the use case body from the
//! aggregates it loads, never by the endpoint (SPEC-konto rule 17).

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    application::ExecutionContext,
    domain::{
        EmailAdresse,
        konto::{Benutzer, BenutzerId, KlartextPasswort, Rolle, verify_password},
    },
    ports::not_found_as_none,
    service::UseCase,
};

/// One Benutzer together with the address of its `Konto`.
///
/// The two are separate aggregates — `Benutzer` is the identity, `Konto` owns the address —
/// but every reader of a Benutzer wants the address, so the use case loads both rather than
/// letting the edge reach for a repository.
#[derive(Debug, Clone)]
pub struct BenutzerAnsicht {
    pub benutzer: Benutzer,
    pub email: EmailAdresse,
}

impl BenutzerAnsicht {
    pub fn id(&self) -> BenutzerId {
        self.benutzer.id()
    }
}

/// Every way `BenutzerAnzeigen` may refuse.
#[derive(Debug, thiserror::Error)]
pub enum BenutzerAnzeigenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("dieser Benutzer darf diese Aktion nicht ausführen")]
    NichtErlaubt,
    #[error("der Benutzer konnte nicht geladen werden")]
    BenutzerLaden,
    #[error("das Konto konnte nicht geladen werden")]
    KontoLaden,
}

/// Load the signed-in Benutzer, or another one when the caller is a `Systemadministrator`.
///
/// Another Benutzer is readable only for a `Systemadministrator`; the caller's own record
/// always is (rule 17, FR-9).
#[derive(Debug, Clone)]
pub struct BenutzerAnzeigen {
    /// `None` means "the caller". `Some(other)` needs `Systemadministrator`.
    pub benutzer_id: Option<BenutzerId>,
}

#[async_trait]
impl UseCase<BenutzerAnsicht> for BenutzerAnzeigen {
    type Error = Report<BenutzerAnzeigenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<BenutzerAnsicht, Self::Error> {
        let (eigene_id, _) = ctx
            .auth()
            .require_authenticated()
            .change_context(BenutzerAnzeigenFehler::NichtAngemeldet)?;

        let ziel_id = self.benutzer_id.unwrap_or(eigene_id);

        let uow = ctx
            .enter()
            .await
            .change_context(BenutzerAnzeigenFehler::BenutzerLaden)?;

        let ergebnis = async {
            let eigenes = uow
                .benutzer()
                .find_by_id(eigene_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(BenutzerAnzeigenFehler::NichtAngemeldet)?;

            let ziel = if ziel_id == eigene_id {
                eigenes
            } else {
                if !eigenes.ist_systemadministrator() {
                    return Err(Report::new(BenutzerAnzeigenFehler::NichtErlaubt));
                }
                uow.benutzer()
                    .find_by_id(ziel_id)
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(BenutzerAnzeigenFehler::NichtErlaubt)?
            };

            let konten = uow
                .konten()
                .find_by_benutzer(ziel.id())
                .await
                .map_err(IntoReport::into_report)
                .change_context(BenutzerAnzeigenFehler::KontoLaden)?;

            // A Benutzer is created together with exactly one Konto (rule 3), so a missing
            // one means the record was torn down: there is no identity left to answer.
            let Some(konto) = konten.into_iter().next() else {
                return Err(Report::new(BenutzerAnzeigenFehler::NichtAngemeldet));
            };

            Ok(BenutzerAnsicht {
                benutzer: ziel.cloned_data(),
                email: konto.email().clone(),
            })
        }
        .await;

        uow.finish(ergebnis, BenutzerAnzeigenFehler::BenutzerLaden)
            .await
    }
}

/// Every way `BenutzerLöschen` may refuse.
#[derive(Debug, thiserror::Error)]
pub enum BenutzerLöschenFehler {
    #[error("die Anfrage ist ungültig")]
    UngültigeAnfrage,
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("das Passwort ist falsch")]
    PasswortFalsch,
    #[error("das Passwort konnte nicht geprüft werden")]
    PasswortPrüfen,
    #[error("das Konto konnte nicht geladen werden")]
    KontoLaden,
    #[error("die Wiederherstellung konnte nicht gelöscht werden")]
    WiederherstellungLöschen,
    #[error("die Bestätigung konnte nicht gelöscht werden")]
    BestätigungLöschen,
    #[error("das Konto konnte nicht gelöscht werden")]
    KontoLöschen,
    #[error("der Benutzer konnte nicht geladen werden")]
    BenutzerLaden,
    #[error("die Sitzungen konnten nicht gelöscht werden")]
    SessionLöschen,
    #[error("der Benutzer konnte nicht gelöscht werden")]
    BenutzerLöschen,
}

/// Delete the caller's own Benutzer, its Konto, sessions, and outstanding tokens.
///
/// Irreversible and gated by re-entering the Passwort plus an explicit confirmation
/// (rule 8, AC-12).
#[derive(Debug)]
pub struct BenutzerLöschen {
    pub passwort: KlartextPasswort,
    /// Explicit confirmation, so a stray request cannot destroy an account.
    pub bestätigt: bool,
}

#[async_trait]
impl UseCase<()> for BenutzerLöschen {
    type Error = Report<BenutzerLöschenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let (benutzer_id, _) = ctx
            .auth()
            .require_authenticated()
            .change_context(BenutzerLöschenFehler::NichtAngemeldet)?;

        if !self.bestätigt {
            return Err(Report::new(BenutzerLöschenFehler::UngültigeAnfrage));
        }

        let uow = ctx
            .enter()
            .await
            .change_context(BenutzerLöschenFehler::KontoLaden)?;

        let ergebnis = async {
            // The row is loaded first: `delete` takes the aggregate as it was read, so a
            // Benutzer changed since then is a conflict rather than a vanished record.
            let benutzer = uow
                .benutzer()
                .find_by_id(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(BenutzerLöschenFehler::BenutzerLaden)?;

            let konten = uow
                .konten()
                .find_by_benutzer(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(BenutzerLöschenFehler::NichtAngemeldet)?;

            // Re-entering the Passwort must match *a* Konto of this Benutzer; a wrong
            // value refuses before anything is removed.
            let mut stimmt = false;
            for konto in &konten {
                if verify_password(&self.passwort, konto.password_hash())
                    .change_context(BenutzerLöschenFehler::PasswortPrüfen)?
                {
                    stimmt = true;
                    break;
                }
            }
            if !stimmt {
                return Err(Report::new(BenutzerLöschenFehler::PasswortFalsch));
            }

            for konto in konten {
                uow.wiederherstellungen()
                    .delete_by_konto(konto.id())
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(BenutzerLöschenFehler::WiederherstellungLöschen)?;
                uow.bestätigungen()
                    .delete_by_email(konto.email())
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(BenutzerLöschenFehler::BestätigungLöschen)?;
                // Delete the Konto explicitly rather than relying on a foreign-key
                // cascade: every adapter must free the address the same way, and a fake
                // has no `ON DELETE CASCADE`. The row goes in as loaded, so a Konto that
                // somebody changed meanwhile is a conflict, not a deletion.
                uow.konten()
                    .delete(konto)
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(BenutzerLöschenFehler::KontoLöschen)?;
            }

            uow.sessions()
                .delete_by_benutzer(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(BenutzerLöschenFehler::SessionLöschen)?;

            uow.benutzer()
                .delete(benutzer)
                .await
                .map_err(IntoReport::into_report)
                .change_context(BenutzerLöschenFehler::NichtAngemeldet)?;

            Ok::<_, Report<BenutzerLöschenFehler>>(())
        }
        .await;

        uow.finish(ergebnis, BenutzerLöschenFehler::KontoLaden)
            .await?;
        tracing::info!(benutzer_id = ?benutzer_id, "benutzer gelöscht");
        Ok(())
    }
}

/// Every way `SystemadministratorBefördern` may refuse.
#[derive(Debug, thiserror::Error)]
pub enum SystemadministratorBefördernFehler {
    #[error("das Konto konnte nicht geladen werden")]
    KontoLaden,
    #[error("der Benutzer konnte nicht geladen werden")]
    BenutzerLaden,
    #[error("der Benutzer konnte nicht gespeichert werden")]
    BenutzerSpeichern,
}

/// Promote the configured address to `Systemadministrator`, or report that there is nothing
/// to promote.
///
/// The only path that grants the role (rule 9): no API route reaches this, it runs at
/// startup from deployment configuration. Idempotent — promoting twice changes nothing —
/// and it never creates an account, because an address with no `Benutzer` has no identity
/// and no password. A missing account is `Ok` so a fresh deployment can start before the
/// superuser has registered.
#[derive(Debug, Clone)]
pub struct SystemadministratorBefördern {
    pub email: EmailAdresse,
}

/// What the bootstrap found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Beförderung {
    /// Promoted now.
    Befördert,
    /// Already a `Systemadministrator`; nothing written.
    BereitsBefördert,
    /// No `Konto` holds the address, so there was nothing to promote.
    KeinKonto,
}

#[async_trait]
impl UseCase<Beförderung> for SystemadministratorBefördern {
    type Error = Report<SystemadministratorBefördernFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Beförderung, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(SystemadministratorBefördernFehler::KontoLaden)?;

        let ergebnis = async {
            let Some(konto) = not_found_as_none(uow.konten().find_by_email(&self.email).await)
                .change_context(SystemadministratorBefördernFehler::KontoLaden)?
            else {
                return Ok(Beförderung::KeinKonto);
            };

            let benutzer = uow
                .benutzer()
                .find_by_id(konto.benutzer_id())
                .await
                .map_err(IntoReport::into_report)
                .change_context(SystemadministratorBefördernFehler::BenutzerLaden)?;

            if benutzer.rolle() == Rolle::Systemadministrator {
                return Ok(Beförderung::BereitsBefördert);
            }

            // A plain domain mutation written back with `update`, so optimistic concurrency
            // applies like any other change.
            let mut benutzer = benutzer;
            benutzer.befördere_zu_systemadministrator();
            uow.benutzer()
                .update(&mut benutzer)
                .await
                .map_err(IntoReport::into_report)
                .change_context(SystemadministratorBefördernFehler::BenutzerSpeichern)?;

            Ok(Beförderung::Befördert)
        }
        .await;

        uow.finish(ergebnis, SystemadministratorBefördernFehler::KontoLaden)
            .await
    }
}
