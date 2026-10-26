use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    application::ExecutionContext,
    domain::{
        EmailAdresse,
        konto::{Benutzer, BenutzerId, KlartextPasswort, Rolle, verify_password},
    },
    service::UseCase,
};

use super::{KontoDienstFehler, speicherfehler};

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

/// Load the signed-in Benutzer, or another one when the caller is a `Systemadministrator`.
///
/// Authorisation is decided here, from the loaded aggregate, not by the endpoint
/// (SPEC-konto rule 17, FR-9): another Benutzer is readable only for a
/// `Systemadministrator`, and the caller's own record always is.
#[derive(Debug, Clone)]
pub struct BenutzerAnzeigen {
    /// `None` means "the caller". `Some(other)` needs `Systemadministrator`.
    pub benutzer_id: Option<BenutzerId>,
}

#[async_trait]
impl UseCase<BenutzerAnsicht> for BenutzerAnzeigen {
    type Error = Report<KontoDienstFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<BenutzerAnsicht, Self::Error> {
        let (eigene_id, _) = ctx
            .auth()
            .require_authenticated()
            .change_context(KontoDienstFehler::NichtAngemeldet)?;

        let ziel_id = self.benutzer_id.unwrap_or(eigene_id);

        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;

        let ergebnis = async {
            let eigenes = uow
                .benutzer()
                .find_by_id(eigene_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::NichtAngemeldet)?;

            let ziel = if ziel_id == eigene_id {
                eigenes
            } else {
                if !eigenes.ist_systemadministrator() {
                    return Err(Report::new(KontoDienstFehler::NichtErlaubt));
                }
                uow.benutzer()
                    .find_by_id(ziel_id)
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(KontoDienstFehler::NichtErlaubt)?
            };

            let konten = uow
                .konten()
                .find_by_benutzer(ziel.id())
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            // A Benutzer is created together with exactly one Konto (rule 3), so a missing
            // one means the record was torn down: there is no identity left to answer.
            let Some(konto) = konten.into_iter().next() else {
                return Err(Report::new(KontoDienstFehler::NichtAngemeldet));
            };

            Ok(BenutzerAnsicht {
                benutzer: ziel,
                email: konto.email().clone(),
            })
        }
        .await;

        uow.finish_with(ergebnis, speicherfehler).await
    }
}

/// Delete the caller's own Benutzer, its Konto, sessions, and outstanding tokens.
///
/// Irreversible and gated by re-entering the Passwort plus an explicit confirmation
/// (rule 8, AC-12). The `Runden`-anonymisation half of AC-12 is deferred until `Runde`
/// exists (tracked on issue #11).
#[derive(Debug, Clone)]
pub struct BenutzerLöschen {
    pub passwort: String,
    /// Explicit confirmation, so a stray request cannot destroy an account.
    pub bestätigt: bool,
}

#[async_trait]
impl UseCase<()> for BenutzerLöschen {
    type Error = Report<KontoDienstFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let (benutzer_id, _) = ctx
            .auth()
            .require_authenticated()
            .change_context(KontoDienstFehler::NichtAngemeldet)?;

        if !self.bestätigt {
            return Err(Report::new(KontoDienstFehler::UngültigeAnfrage));
        }

        let passwort = KlartextPasswort::neu(self.passwort)
            .change_context(KontoDienstFehler::UngültigeAnfrage)?;

        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;

        let ergebnis = async {
            let konten = uow
                .konten()
                .find_by_benutzer(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::NichtAngemeldet)?;

            // Re-entering the Passwort must match *a* Konto of this Benutzer; a wrong
            // value refuses before anything is removed.
            let mut stimmt = false;
            for konto in &konten {
                if verify_password(&passwort, konto.password_hash())
                    .change_context(KontoDienstFehler::Speicher)?
                {
                    stimmt = true;
                    break;
                }
            }
            if !stimmt {
                return Err(Report::new(KontoDienstFehler::ZugangsdatenFalsch));
            }

            for konto in &konten {
                uow.wiederherstellungen()
                    .delete_alle_fuer_konto(konto.id())
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(KontoDienstFehler::Speicher)?;
                uow.bestätigungen()
                    .delete_alle_fuer_email(konto.email())
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(KontoDienstFehler::Speicher)?;
                // Delete the Konto explicitly rather than relying on a foreign-key
                // cascade: every adapter must free the address the same way, and a fake
                // has no `ON DELETE CASCADE`.
                uow.konten()
                    .delete(konto.id())
                    .await
                    .map_err(IntoReport::into_report)
                    .change_context(KontoDienstFehler::Speicher)?;
            }

            uow.sessions()
                .delete_alle_fuer_benutzer(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            uow.benutzer()
                .delete(benutzer_id)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::NichtAngemeldet)?;

            Ok::<_, Report<KontoDienstFehler>>(())
        }
        .await;

        uow.finish_with(ergebnis, speicherfehler).await?;
        tracing::info!(benutzer_id = ?benutzer_id, "benutzer gelöscht");
        Ok(())
    }
}

/// Promote the configured address to `Systemadministrator`, or report that there is
/// nothing to promote.
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
    type Error = Report<KontoDienstFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Beförderung, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;

        let ergebnis = async {
            let Some(konto) = uow
                .konten()
                .find_by_email(&self.email)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?
            else {
                return Ok(Beförderung::KeinKonto);
            };

            let benutzer = uow
                .benutzer()
                .find_by_id(konto.benutzer_id())
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            if benutzer.rolle() == Rolle::Systemadministrator {
                return Ok(Beförderung::BereitsBefördert);
            }

            uow.benutzer()
                .setze_rolle(benutzer.id(), Rolle::Systemadministrator)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            Ok(Beförderung::Befördert)
        }
        .await;

        uow.finish_with(ergebnis, speicherfehler).await
    }
}
