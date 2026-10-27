//! Registration: two steps, one Benutzer at the end.
//!
//! One error enum per use case, each variant naming what the use case could not do.

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    application::ExecutionContext,
    domain::{
        EmailAdresse,
        konto::{
            KlartextPasswort, NeueBestätigung, NeuerBenutzer, NeuesKonto, RefreshToken,
            SecretToken, hash_password,
        },
    },
    ports::{ClientKey, Mail, RepositoryError, ThrottleAction},
    service::{SessionAusgestellt, UseCase},
};

use super::{claims_for_session, new_session};

/// Every way `RegistrierungStarten` may refuse.
#[derive(Debug, thiserror::Error)]
pub enum RegistrierungStartenFehler {
    #[error("die Anfrage ist ungültig")]
    UngültigeAnfrage,
    #[error("zu viele Versuche; bitte später erneut versuchen")]
    Gedrosselt,
    #[error("die E-Mail-Adresse konnte nicht geprüft werden")]
    KontoPrüfen,
    #[error("das Bestätigungs-Token konnte nicht erzeugt werden")]
    TokenErzeugen,
    #[error("die Bestätigung konnte nicht gespeichert werden")]
    BestätigungSpeichern,
}

/// Registration step 1: name an address, receive a confirmation mail.
///
/// Answers identically whether the address is free or already holds a `Konto` (rule 4,
/// AC-1, AC-2). The only observable difference is that a free address gets mail. Nothing is
/// written except an `EmailBestätigung`, so no `Benutzer` exists yet (AC-20).
#[derive(Debug, Clone)]
pub struct RegistrierungStarten {
    pub email: EmailAdresse,
    /// Who is asking, for the client-scoped throttle. Never the address itself.
    pub client_key: ClientKey,
}

#[async_trait]
impl UseCase<()> for RegistrierungStarten {
    type Error = Report<RegistrierungStartenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let jetzt = ctx.clock().now();

        ctx.throttle()
            .check_and_count(&self.client_key, ThrottleAction::RegistrierungStart, jetzt)
            .await
            .change_context(RegistrierungStartenFehler::Gedrosselt)?;

        let uow = ctx
            .enter()
            .await
            .change_context(RegistrierungStartenFehler::KontoPrüfen)?;

        let konto = uow.konten().find_by_email(&self.email).await;
        uow.finish(Ok(()), RegistrierungStartenFehler::KontoPrüfen)
            .await?;
        match konto {
            Ok(_) => {
                // Taken: same answer as free, and the owner is not mailed either (AC-2).
                tracing::info!("registrierung gestartet für eine bereits vergebene adresse");
                return Ok(());
            }
            Err(fehler) if !matches!(fehler.current_context(), RepositoryError::NotFound) => {
                return Err(fehler).change_context(RegistrierungStartenFehler::KontoPrüfen);
            }
            _ => {}
        };

        let token =
            SecretToken::random().change_context(RegistrierungStartenFehler::TokenErzeugen)?;
        let email = self.email;
        let uow = ctx
            .enter()
            .await
            .change_context(RegistrierungStartenFehler::BestätigungSpeichern)?;

        let ergebnis = async {
            // One link per address: an outstanding confirmation is spent before the new
            // one is written (rule 13).
            spend_open_confirmations(&uow, &email, jetzt).await?;

            let bestätigung = NeueBestätigung::neu(email.clone(), (&token).into(), jetzt);
            uow.bestätigungen()
                .create(bestätigung)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RegistrierungStartenFehler::BestätigungSpeichern)?;
            Ok::<_, Report<RegistrierungStartenFehler>>(())
        }
        .await;

        uow.finish(ergebnis, RegistrierungStartenFehler::BestätigungSpeichern)
            .await?;

        if let Err(fehler) = ctx
            .mail_sender()
            .send(Mail::email_bestätigung(email, token.base64url().as_str()))
            .await
        {
            tracing::warn!(fehler = %fehler, "Transaktionsmail konnte nicht zugestellt werden");
        }
        Ok(())
    }
}

async fn spend_open_confirmations(
    uow: &crate::application::uow::UnitOfWork<'_>,
    email: &EmailAdresse,
    jetzt: chrono::DateTime<chrono::Utc>,
) -> Result<(), Report<RegistrierungStartenFehler>> {
    let offene = uow
        .bestätigungen()
        .find_by_email(email)
        .await
        .map_err(IntoReport::into_report)
        .change_context(RegistrierungStartenFehler::BestätigungSpeichern)?;
    for bestätigung in offene {
        if bestätigung.ist_verbraucht() {
            continue;
        }
        let mut geladen = bestätigung;
        geladen.verbrauchen(jetzt);
        uow.bestätigungen()
            .update(&mut geladen)
            .await
            .map_err(IntoReport::into_report)
            .change_context(RegistrierungStartenFehler::BestätigungSpeichern)?;
    }
    Ok(())
}

/// Every way `RegistrierungAbschließen` may refuse.
#[derive(Debug, thiserror::Error)]
pub enum RegistrierungAbschliessenFehler {
    #[error("die Anfrage ist ungültig")]
    UngültigeAnfrage,
    #[error("zu viele Versuche; bitte später erneut versuchen")]
    Gedrosselt,
    #[error("die Bestätigung konnte nicht geladen werden")]
    BestätigungLaden,
    #[error("die Bestätigung konnte nicht gespeichert werden")]
    BestätigungSpeichern,
    #[error("das Passwort konnte nicht gespeichert werden")]
    PasswortSetzen,
    #[error("der Benutzer konnte nicht angelegt werden")]
    BenutzerAnlegen,
    #[error("das Konto konnte nicht angelegt werden")]
    KontoAnlegen,
    #[error("das Refresh-Token konnte nicht erzeugt werden")]
    TokenErzeugen,
    #[error("die Sitzung konnte nicht gespeichert werden")]
    SessionSpeichern,
}

/// Registration step 2: spend the confirmation token, set the Passwort, be logged in.
///
/// The `Benutzer` and its one `Konto` are created here and nowhere else, so an unconfirmed
/// `Benutzer` cannot exist (rule 3, AC-20). A token whose address gained a `Konto` in the
/// meantime is spent and refused without creating anything (rule 4).
#[derive(Debug)]
pub struct RegistrierungAbschließen {
    pub token: SecretToken,
    pub passwort: KlartextPasswort,
    pub client_key: ClientKey,
}

#[async_trait]
impl UseCase<SessionAusgestellt> for RegistrierungAbschließen {
    type Error = Report<RegistrierungAbschliessenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<SessionAusgestellt, Self::Error> {
        let jetzt = ctx.clock().now();

        ctx.throttle()
            .check_and_count(
                &self.client_key,
                ThrottleAction::RegistrierungAbschluss,
                jetzt,
            )
            .await
            .change_context(RegistrierungAbschliessenFehler::Gedrosselt)?;

        // Hash before the token is looked at: a Passwort that fails the length rule must
        // not spend a usable token (AC-4). The length rule already ran at construction.
        let hashed_passwort = hash_password(&self.passwort)
            .change_context(RegistrierungAbschliessenFehler::PasswortSetzen)?;

        let uow = ctx
            .enter()
            .await
            .change_context(RegistrierungAbschliessenFehler::BestätigungLaden)?;

        let ergebnis = async {
            let bestätigung = match uow
                .bestätigungen()
                .find_by_hash(&(&self.token).into())
                .await
            {
                Ok(bestätigung) => bestätigung,
                Err(fehler) if matches!(fehler.current_context(), RepositoryError::NotFound) => {
                    return Err(Report::new(
                        RegistrierungAbschliessenFehler::UngültigeAnfrage,
                    ));
                }
                Err(fehler) => {
                    return Err(fehler)
                        .change_context(RegistrierungAbschliessenFehler::BestätigungLaden);
                }
            };

            if !bestätigung.ist_gültig(jetzt) {
                return Err(Report::new(
                    RegistrierungAbschliessenFehler::UngültigeAnfrage,
                ));
            }

            // Spend first, so a concurrent completion sees `VersionMismatch` rather than
            // creating a second Benutzer for one address.
            let mut verbraucht = bestätigung.clone();
            verbraucht.verbrauchen(jetzt);
            uow.bestätigungen()
                .update(&mut verbraucht)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RegistrierungAbschliessenFehler::BestätigungSpeichern)?;

            let email = bestätigung.email().clone();
            // The address gained a Konto while this token was outstanding (rule 4).
            match uow.konten().find_by_email(&email).await {
                Ok(_) => {
                    return Err(Report::new(
                        RegistrierungAbschliessenFehler::UngültigeAnfrage,
                    ));
                }
                Err(fehler) if !matches!(fehler.current_context(), RepositoryError::NotFound) => {
                    return Err(fehler)
                        .change_context(RegistrierungAbschliessenFehler::KontoAnlegen);
                }
                _ => {}
            };

            let benutzer = uow
                .benutzer()
                .create(NeuerBenutzer::registrierung(jetzt))
                .await
                .map_err(IntoReport::into_report)
                .change_context(RegistrierungAbschliessenFehler::BenutzerAnlegen)?;

            uow.konten()
                .create(NeuesKonto::neu(
                    benutzer.id(),
                    email.clone(),
                    hashed_passwort,
                    jetzt,
                ))
                .await
                .map_err(IntoReport::into_report)
                .change_context(RegistrierungAbschliessenFehler::KontoAnlegen)?;

            uow.bestätigungen()
                .delete_by_email(&email)
                .await
                .map_err(IntoReport::into_report)
                .change_context(RegistrierungAbschliessenFehler::BestätigungSpeichern)?;

            let refresh_token = RefreshToken::random()
                .change_context(RegistrierungAbschliessenFehler::TokenErzeugen)?;
            let session = uow
                .sessions()
                .create(new_session(&ctx, benutzer.id(), &refresh_token, jetzt))
                .await
                .map_err(IntoReport::into_report)
                .change_context(RegistrierungAbschliessenFehler::SessionSpeichern)?;

            Ok::<_, Report<RegistrierungAbschliessenFehler>>(SessionAusgestellt {
                benutzer: benutzer.cloned_data(),
                session_id: session.id(),
                refresh_token,
                claims: claims_for_session(&ctx, benutzer.id(), session.id(), jetzt),
            })
        }
        .await;

        let ausgestellt = uow
            .finish(ergebnis, RegistrierungAbschliessenFehler::BestätigungLaden)
            .await?;

        tracing::info!(
            benutzer_id = ?ausgestellt.benutzer.id(),
            "registrierung abgeschlossen"
        );
        Ok(ausgestellt)
    }
}
