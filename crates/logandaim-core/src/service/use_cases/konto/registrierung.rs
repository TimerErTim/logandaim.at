use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    application::ExecutionContext,
    domain::{
        EmailAdresse,
        konto::{
            KlartextPasswort, NeueBestaetigung, NeueSession, NeuerBenutzer, NeuesKonto,
            generate_secret_token, hash_password, hash_token,
        },
    },
    ports::{DrosselAktion, KundeSchluessel, Mail},
    service::UseCase,
};

use super::{
    KontoDienstFehler, SitzungAusgestellt, claims_fuer, drossle, sende_mail, signing_key_id,
    speicherfehler,
};

/// Registration step 1: name an address, receive a confirmation mail.
///
/// Answers identically whether the address is free or already holds a `Konto` (rule 4,
/// AC-1, AC-2). The only observable difference is that a free address gets mail. Nothing
/// is written except an `EmailBestaetigung`, so no `Benutzer` exists yet (AC-20).
#[derive(Debug, Clone)]
pub struct RegistrierungStarten {
    /// Address as typed. Normalization is the domain's job, not the edge's.
    pub email: String,
    /// Who is asking, for the client-scoped throttle. Never the address itself.
    pub kunde: KundeSchluessel,
}

#[async_trait]
impl UseCase<()> for RegistrierungStarten {
    type Error = Report<KontoDienstFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let email = EmailAdresse::normalisiert(&self.email)
            .change_context(KontoDienstFehler::UngueltigeAnfrage)?;
        let jetzt = ctx.clock().now();

        drossle(&ctx, &self.kunde, DrosselAktion::RegistrierungStart, jetzt).await?;

        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;

        let vorhandenes_konto = uow
            .konten()
            .find_by_email(&email)
            .await
            .map_err(IntoReport::into_report)
            .change_context(KontoDienstFehler::Speicher);

        let vorhandenes_konto = uow
            .finish(vorhandenes_konto, KontoDienstFehler::Speicher)
            .await?;

        if vorhandenes_konto.is_some() {
            // Taken: same answer as free, and the owner is not mailed either (AC-2).
            tracing::info!("registrierung gestartet für eine bereits vergebene adresse");
            return Ok(());
        }

        let token = generate_secret_token().change_context(KontoDienstFehler::Speicher)?;
        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;

        let ergebnis = async {
            uow.bestaetigungen()
                .invalidate_open_for_email(&email, jetzt)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            let bestaetigung = NeueBestaetigung::neu(email.clone(), hash_token(&token), jetzt);
            let bestaetigung = uow
                .bestaetigungen()
                .create(bestaetigung)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;
            Ok::<_, Report<KontoDienstFehler>>(bestaetigung)
        }
        .await;

        uow.finish_with(ergebnis, speicherfehler).await?;

        let link = ctx.konto_konfiguration().bestaetigung_url(token.as_str());
        sende_mail(
            &ctx,
            Mail::neu(
                email,
                "Bestätige deine E-Mail-Adresse",
                format!(
                    "Hallo,\n\n\
                     bitte bestätige deine E-Mail-Adresse für logandaim.at:\n\n\
                     {link}\n\n\
                     Der Link ist 24 Stunden gültig und kann nur einmal verwendet werden.\n\
                     Wenn du dich nicht registriert hast, ignoriere diese Mail.\n"
                ),
            ),
        )
        .await;

        Ok(())
    }
}

/// Registration step 2: spend the confirmation token, set the Passwort, be logged in.
///
/// The `Benutzer` and its one `Konto` are created here and nowhere else, so an
/// unconfirmed `Benutzer` cannot exist (rule 3, AC-20). A token whose address gained a
/// `Konto` in the meantime is spent and refused without creating anything (rule 4).
#[derive(Debug, Clone)]
pub struct RegistrierungAbschliessen {
    pub token: String,
    pub passwort: String,
    /// Trailing characters beyond the cap are rejected, not truncated.
    pub kunde: KundeSchluessel,
}

#[async_trait]
impl UseCase<SitzungAusgestellt> for RegistrierungAbschliessen {
    type Error = Report<KontoDienstFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<SitzungAusgestellt, Self::Error> {
        let jetzt = ctx.clock().now();

        drossle(
            &ctx,
            &self.kunde,
            DrosselAktion::RegistrierungAbschluss,
            jetzt,
        )
        .await?;

        // Validate and hash before the token is looked at: a Passwort that fails the
        // length rule must not spend a usable token (AC-4).
        let passwort = KlartextPasswort::neu(self.passwort)
            .change_context(KontoDienstFehler::UngueltigeAnfrage)?;
        let token = crate::domain::konto::SecretToken::parse(self.token)
            .change_context(KontoDienstFehler::UngueltigeAnfrage)?;
        let password_hash = hash_password(&passwort).change_context(KontoDienstFehler::Speicher)?;

        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;

        let ergebnis = async {
            let bestaetigung = uow
                .bestaetigungen()
                .find_by_hash(&hash_token(&token))
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            let Some(bestaetigung) = bestaetigung else {
                return Err(Report::new(KontoDienstFehler::UngueltigeAnfrage));
            };

            if !bestaetigung.ist_gueltig(jetzt) {
                return Err(Report::new(KontoDienstFehler::UngueltigeAnfrage));
            }

            // Spend first, so a concurrent completion sees `Conflict` rather than
            // creating a second Benutzer for one address.
            uow.bestaetigungen()
                .consume(bestaetigung.id(), jetzt)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::UngueltigeAnfrage)?;

            let email = bestaetigung.email().clone();
            let belegt = uow
                .konten()
                .find_by_email(&email)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;
            if belegt.is_some() {
                // The address gained a Konto while this token was outstanding (rule 4).
                return Err(Report::new(KontoDienstFehler::UngueltigeAnfrage));
            }

            let benutzer = uow
                .benutzer()
                .create(NeuerBenutzer::registrierung(jetzt))
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            uow.konten()
                .create(NeuesKonto::neu(benutzer.id(), email, password_hash, jetzt))
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            uow.bestaetigungen()
                .delete_alle_fuer_email(bestaetigung.email())
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            let refresh_token = crate::domain::konto::RefreshToken::neu()
                .change_context(KontoDienstFehler::Speicher)?;
            let session = uow
                .sessions()
                .create(NeueSession::neu(
                    benutzer.id(),
                    refresh_token.hash(),
                    signing_key_id(&ctx),
                    jetzt,
                ))
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            let claims = claims_fuer(&ctx, benutzer.id(), session.id(), jetzt);
            Ok::<_, Report<KontoDienstFehler>>(SitzungAusgestellt {
                benutzer,
                session_id: session.id(),
                refresh_token,
                claims,
            })
        }
        .await;

        let ausgestellt = uow.finish_with(ergebnis, speicherfehler).await?;

        tracing::info!(
            benutzer_id = ?ausgestellt.benutzer.id(),
            "registrierung abgeschlossen"
        );
        Ok(ausgestellt)
    }
}
