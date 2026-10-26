use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    application::ExecutionContext,
    domain::{
        EmailAdresse,
        konto::{
            KlartextPasswort, NeueWiederherstellung, generate_secret_token, hash_password,
            hash_token,
        },
    },
    ports::{DrosselAktion, KundeSchlüssel, Mail},
    service::UseCase,
};

use super::{KontoDienstFehler, drossle, sende_mail, speicherfehler};

/// Request a Passwort-Wiederherstellung. Always the same `Ok(())` (rule 7, AC-10).
///
/// Only a real Konto receives mail, and requesting again spends the previous token, so at
/// most one link per Konto is usable.
#[derive(Debug, Clone)]
pub struct PasswortVergessen {
    pub email: String,
    pub kunde: KundeSchlüssel,
}

#[async_trait]
impl UseCase<()> for PasswortVergessen {
    type Error = Report<KontoDienstFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let email = EmailAdresse::normalisiert(&self.email)
            .change_context(KontoDienstFehler::UngültigeAnfrage)?;
        let jetzt = ctx.clock().now();

        drossle(&ctx, &self.kunde, DrosselAktion::Wiederherstellung, jetzt).await?;

        let token = generate_secret_token().change_context(KontoDienstFehler::Speicher)?;

        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;

        let ergebnis = async {
            let konto = uow
                .konten()
                .find_by_email(&email)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            let Some(konto) = konto else {
                return Ok::<_, Report<KontoDienstFehler>>(false);
            };

            uow.wiederherstellungen()
                .invalidate_open_for_konto(konto.id(), jetzt)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            uow.wiederherstellungen()
                .create(NeueWiederherstellung::neu(
                    konto.id(),
                    hash_token(&token),
                    jetzt,
                ))
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            Ok(true)
        }
        .await;

        let verschickt = uow.finish_with(ergebnis, speicherfehler).await?;
        if !verschickt {
            tracing::info!("wiederherstellung angefordert für eine unbekannte adresse");
            return Ok(());
        }

        sende_mail(
            &ctx,
            Mail::passwort_wiederherstellung(email, token.as_str()),
        )
        .await;
        Ok(())
    }
}

/// Set a new Passwort with a recovery token. Single use, ends every session (AC-11).
#[derive(Debug, Clone)]
pub struct PasswortWiederherstellen {
    pub token: String,
    pub neues_passwort: String,
}

#[async_trait]
impl UseCase<()> for PasswortWiederherstellen {
    type Error = Report<KontoDienstFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let jetzt = ctx.clock().now();

        // Validate and hash before spending the token, so a too-short Passwort does not
        // burn the recipient's only link.
        let passwort = KlartextPasswort::neu(self.neues_passwort)
            .change_context(KontoDienstFehler::UngültigeAnfrage)?;
        let token = crate::domain::konto::SecretToken::parse(self.token)
            .change_context(KontoDienstFehler::UngültigeAnfrage)?;
        let password_hash = hash_password(&passwort).change_context(KontoDienstFehler::Speicher)?;

        let uow = ctx
            .enter()
            .await
            .change_context(KontoDienstFehler::Speicher)?;

        let ergebnis = async {
            let wiederherstellung = uow
                .wiederherstellungen()
                .find_by_hash(&hash_token(&token))
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            let Some(wiederherstellung) = wiederherstellung else {
                return Err(Report::new(KontoDienstFehler::UngültigeAnfrage));
            };

            if !wiederherstellung.ist_gültig(jetzt) {
                return Err(Report::new(KontoDienstFehler::UngültigeAnfrage));
            }

            uow.wiederherstellungen()
                .consume(wiederherstellung.id(), jetzt)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::UngültigeAnfrage)?;

            let mut konto = uow
                .konten()
                .find_by_id(wiederherstellung.konto_id())
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::UngültigeAnfrage)?;

            konto.setze_password_hash(password_hash);
            uow.konten()
                .speichere(&konto)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            // Every Anmelde-Sitzung of the owner ends, including their access tokens
            // (rule 6, AC-11): whoever held a stolen token is out.
            uow.sessions()
                .widerrufe_alle_fuer_benutzer(konto.benutzer_id(), jetzt)
                .await
                .map_err(IntoReport::into_report)
                .change_context(KontoDienstFehler::Speicher)?;

            Ok::<_, Report<KontoDienstFehler>>(())
        }
        .await;

        uow.finish_with(ergebnis, speicherfehler).await
    }
}
