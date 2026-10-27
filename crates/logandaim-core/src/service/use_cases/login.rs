//! Login.
//!
//! One error enum per use case. Every variant names what the use case could not do and
//! carries exactly one HTTP meaning; the underlying repository or crypto error stays in
//! the `error_stack` chain instead of being flattened into a generic storage failure.

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    application::ExecutionContext,
    domain::{
        EmailAdresse,
        konto::{
            KlartextPasswort, RefreshToken, SitzungAusgestellt, dummy_verify, verify_password,
        },
    },
    ports::{ClientKey, ThrottleAction},
    service::UseCase,
};

use super::{claims_for_session, new_session};

/// Every way `Anmeldung` may refuse.
#[derive(Debug, thiserror::Error)]
pub enum AnmeldungFehler {
    #[error("E-Mail-Adresse oder Passwort ist falsch")]
    ZugangsdatenFalsch,
    #[error("zu viele Versuche; bitte später erneut versuchen")]
    Gedrosselt,
    #[error("das Konto konnte nicht geladen werden")]
    KontoLaden,
    #[error("der Benutzer konnte nicht geladen werden")]
    BenutzerLaden,
    #[error("das Passwort konnte nicht geprüft werden")]
    PasswortPrüfen,
    #[error("das Refresh-Token konnte nicht erzeugt werden")]
    TokenErzeugen,
    #[error("die Sitzung konnte nicht gespeichert werden")]
    SitzungSpeichern,
}

/// Log in with address and Passwort.
///
/// Unknown address and wrong Passwort answer identically, and the unknown-address path
/// still pays for one argon2 verification (`dummy_verify`), so the response time does not
/// disclose existence (rule 4, AC-6).
#[derive(Debug)]
pub struct Anmeldung {
    pub email: EmailAdresse,
    pub passwort: KlartextPasswort,
    /// Throttle key. Never the address: that would let anybody lock an owner out.
    pub client_key: ClientKey,
}

#[async_trait]
impl UseCase<SitzungAusgestellt> for Anmeldung {
    type Error = Report<AnmeldungFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<SitzungAusgestellt, Self::Error> {
        let jetzt = ctx.clock().now();

        ctx.throttle()
            .check_and_count(&self.client_key, ThrottleAction::Anmeldung, jetzt)
            .await
            .change_context(AnmeldungFehler::Gedrosselt)?;

        let uow = ctx
            .enter()
            .await
            .change_context(AnmeldungFehler::KontoLaden)?;

        let ergebnis = async {
            let konto = uow
                .konten()
                .find_by_email(&self.email)
                .await
                .map_err(IntoReport::into_report)
                .change_context(AnmeldungFehler::KontoLaden)?;

            let Some(konto) = konto else {
                // Same work as the wrong-Passwort path (rule 4).
                dummy_verify(&self.passwort).change_context(AnmeldungFehler::PasswortPrüfen)?;
                return Err(Report::new(AnmeldungFehler::ZugangsdatenFalsch));
            };

            let stimmt = verify_password(&self.passwort, konto.password_hash())
                .change_context(AnmeldungFehler::PasswortPrüfen)?;
            if !stimmt {
                return Err(Report::new(AnmeldungFehler::ZugangsdatenFalsch));
            }

            let benutzer = uow
                .benutzer()
                .find_by_id(konto.benutzer_id())
                .await
                .map_err(IntoReport::into_report)
                .change_context(AnmeldungFehler::BenutzerLaden)?;

            let refresh_token =
                RefreshToken::random().change_context(AnmeldungFehler::TokenErzeugen)?;
            let session = uow
                .sessions()
                .create(new_session(&ctx, benutzer.id(), &refresh_token, jetzt))
                .await
                .map_err(IntoReport::into_report)
                .change_context(AnmeldungFehler::SitzungSpeichern)?;

            Ok::<_, Report<AnmeldungFehler>>(SitzungAusgestellt {
                benutzer: benutzer.cloned_data(),
                session_id: session.id(),
                refresh_token,
                claims: claims_for_session(&ctx, benutzer.id(), session.id(), jetzt),
            })
        }
        .await;

        let ausgestellt = uow.finish(ergebnis, AnmeldungFehler::KontoLaden).await?;

        // A successful login clears the client's failure budget, so one mistyped
        // Passwort is not held against it for the rest of the window (rule 5).
        if let Err(fehler) = ctx
            .throttle()
            .reset(&self.client_key, ThrottleAction::Anmeldung)
            .await
        {
            tracing::warn!(fehler = %fehler, "die Drossel konnte nicht zurückgesetzt werden");
        }

        tracing::info!(benutzer_id = ?ausgestellt.benutzer.id(), "angemeldet");
        Ok(ausgestellt)
    }
}
