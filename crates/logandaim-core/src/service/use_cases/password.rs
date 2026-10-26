//! Password recovery: request a link, set a new Passwort.
//!
//! One error enum per use case. The `sende_mail` wrapper is gone: the flow is linear here,
//! the mail is sent once after the unit of work commits, and a failure is a warning
//! (NFR-5).

use async_trait::async_trait;
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    application::ExecutionContext,
    domain::{
        EmailAdresse,
        konto::{KlartextPasswort, NeuePasswortWiederherstellung, SecretToken, hash_password},
    },
    ports::{ClientKey, Mail, ThrottleAction},
    service::UseCase,
};

use super::{revoke_all_sessions, send_mail};

/// Every way `PasswortVergessen` may refuse.
#[derive(Debug, thiserror::Error)]
pub enum PasswortVergessenFehler {
    #[error("die Anfrage ist ungültig")]
    UngültigeAnfrage,
    #[error("zu viele Versuche; bitte später erneut versuchen")]
    Gedrosselt,
    #[error("das Token konnte nicht erzeugt werden")]
    TokenErzeugen,
    #[error("das Konto konnte nicht geladen werden")]
    KontoLaden,
    #[error("die Wiederherstellung konnte nicht gespeichert werden")]
    WiederherstellungSpeichern,
}

/// Request a Passwort-Wiederherstellung. Always the same `Ok(())` (rule 7, AC-10).
///
/// Only a real Konto receives mail, and requesting again spends the previous token, so at
/// most one link per Konto is usable.
#[derive(Debug, Clone)]
pub struct PasswortVergessen {
    pub email: EmailAdresse,
    pub client_key: ClientKey,
}

#[async_trait]
impl UseCase<()> for PasswortVergessen {
    type Error = Report<PasswortVergessenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let jetzt = ctx.clock().now();

        ctx.throttle()
            .check_and_count(&self.client_key, ThrottleAction::Wiederherstellung, jetzt)
            .await
            .change_context(PasswortVergessenFehler::Gedrosselt)?;

        let token = SecretToken::random().change_context(PasswortVergessenFehler::TokenErzeugen)?;

        let uow = ctx
            .enter()
            .await
            .change_context(PasswortVergessenFehler::KontoLaden)?;

        let ergebnis = async {
            let konto = uow
                .konten()
                .find_by_email(&self.email)
                .await
                .map_err(IntoReport::into_report)
                .change_context(PasswortVergessenFehler::KontoLaden)?;

            let Some(konto) = konto else {
                return Ok::<_, Report<PasswortVergessenFehler>>(false);
            };

            // One usable link per Konto: spend the outstanding ones first (rule 7).
            spend_open_recoveries(&uow, konto.id(), jetzt).await?;

            uow.wiederherstellungen()
                .create(NeuePasswortWiederherstellung::neu(
                    konto.id(),
                    (&token).into(),
                    jetzt,
                ))
                .await
                .map_err(IntoReport::into_report)
                .change_context(PasswortVergessenFehler::WiederherstellungSpeichern)?;

            Ok(true)
        }
        .await;

        let verschickt = uow
            .finish(ergebnis, PasswortVergessenFehler::KontoLaden)
            .await?;
        if !verschickt {
            tracing::info!("wiederherstellung angefordert für eine unbekannte adresse");
            return Ok(());
        }

        send_mail(
            &ctx,
            Mail::passwort_wiederherstellung(self.email, token.base64url().as_str()),
        )
        .await;
        Ok(())
    }
}

async fn spend_open_recoveries(
    uow: &crate::application::uow::UnitOfWork<'_>,
    konto_id: crate::domain::konto::KontoId,
    jetzt: chrono::DateTime<chrono::Utc>,
) -> Result<(), Report<PasswortVergessenFehler>> {
    let offene = uow
        .wiederherstellungen()
        .find_by_konto(konto_id)
        .await
        .map_err(IntoReport::into_report)
        .change_context(PasswortVergessenFehler::WiederherstellungSpeichern)?;
    for wiederherstellung in offene {
        if wiederherstellung.ist_verbraucht() {
            continue;
        }
        let mut neu = wiederherstellung.cloned_data();
        neu.verbrauchen(jetzt);
        uow.wiederherstellungen()
            .update(&crate::application::uow::Versioned::new(
                wiederherstellung.v(),
                neu,
            ))
            .await
            .map_err(IntoReport::into_report)
            .change_context(PasswortVergessenFehler::WiederherstellungSpeichern)?;
    }
    Ok(())
}

/// Every way `PasswortWiederherstellen` may refuse.
#[derive(Debug, thiserror::Error)]
pub enum PasswortWiederherstellenFehler {
    #[error("die Anfrage ist ungültig")]
    UngültigeAnfrage,
    #[error("das Passwort konnte nicht gespeichert werden")]
    PasswortSetzen,
    #[error("die Wiederherstellung konnte nicht geladen werden")]
    WiederherstellungLaden,
    #[error("die Wiederherstellung konnte nicht gespeichert werden")]
    WiederherstellungSpeichern,
    #[error("das Konto konnte nicht gespeichert werden")]
    KontoSpeichern,
    #[error("die Sitzung konnte nicht geladen werden")]
    SitzungLaden,
    #[error("die Sitzung konnte nicht gespeichert werden")]
    SitzungSpeichern,
}

/// Set a new Passwort with a recovery token. Single use, ends every session (AC-11).
#[derive(Debug)]
pub struct PasswortWiederherstellen {
    pub token: SecretToken,
    pub neues_passwort: KlartextPasswort,
}

#[async_trait]
impl UseCase<()> for PasswortWiederherstellen {
    type Error = Report<PasswortWiederherstellenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let jetzt = ctx.clock().now();

        // Hash before spending the token, so a too-short Passwort does not burn the
        // recipient's only link (AC-4). The length rule already ran at construction.
        let password_hash = hash_password(&self.neues_passwort)
            .change_context(PasswortWiederherstellenFehler::PasswortSetzen)?;

        let uow = ctx
            .enter()
            .await
            .change_context(PasswortWiederherstellenFehler::WiederherstellungLaden)?;

        let ergebnis = async {
            let wiederherstellung = uow
                .wiederherstellungen()
                .find_by_hash(&(&self.token).into())
                .await
                .map_err(IntoReport::into_report)
                .change_context(PasswortWiederherstellenFehler::WiederherstellungLaden)?;

            let Some(wiederherstellung) = wiederherstellung else {
                return Err(Report::new(
                    PasswortWiederherstellenFehler::UngültigeAnfrage,
                ));
            };

            if !wiederherstellung.ist_gültig(jetzt) {
                return Err(Report::new(
                    PasswortWiederherstellenFehler::UngültigeAnfrage,
                ));
            }

            let mut verbraucht = wiederherstellung.cloned_data();
            verbraucht.verbrauchen(jetzt);
            uow.wiederherstellungen()
                .update(&crate::application::uow::Versioned::new(
                    wiederherstellung.v(),
                    verbraucht,
                ))
                .await
                .map_err(IntoReport::into_report)
                .change_context(PasswortWiederherstellenFehler::WiederherstellungSpeichern)?;

            let konto = uow
                .konten()
                .find_by_id(wiederherstellung.konto_id())
                .await
                .map_err(IntoReport::into_report)
                .change_context(PasswortWiederherstellenFehler::UngültigeAnfrage)?;

            let mut neu = konto.cloned_data();
            neu.setze_password_hash(password_hash);
            uow.konten()
                .update(&crate::application::uow::Versioned::new(konto.v(), neu))
                .await
                .map_err(IntoReport::into_report)
                .change_context(PasswortWiederherstellenFehler::KontoSpeichern)?;

            // Every Anmelde-Sitzung of the owner ends, including their access tokens
            // (rule 6, AC-11): whoever held a stolen token is out.
            revoke_all_sessions(&uow, konto.benutzer_id(), jetzt, || {
                PasswortWiederherstellenFehler::SitzungSpeichern
            })
            .await?;

            Ok::<_, Report<PasswortWiederherstellenFehler>>(())
        }
        .await;

        uow.finish(
            ergebnis,
            PasswortWiederherstellenFehler::WiederherstellungLaden,
        )
        .await
    }
}
