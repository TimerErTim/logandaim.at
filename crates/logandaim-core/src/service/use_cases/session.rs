//! Session lifecycle: renewal, logout, and access-token verification.
//!
//! One error enum per use case. Each variant names what the use case could not do and
//! keeps the underlying repository error in the `error_stack` chain.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    ThreadSafeError,
    application::{AuthContext, ExecutionContext},
    domain::konto::{
        AccessTokenClaims, BenutzerId, RefreshToken, RenewableSession, SessionNotRenewable,
        Signature, SigningKeypair,
    },
    ports::{ClientKey, RepositoryError, ThrottleAction},
    service::UseCase,
    uow::UnitOfWork,
};

use super::claims_for_session;

/// A renewed token pair: the API encodes `claims`, the client gets `refresh_token`.
#[derive(Debug, Clone)]
pub struct ErneuerungAusgestellt {
    pub refresh_token: RefreshToken,
    pub claims: AccessTokenClaims,
}

/// Every way `Erneuerung` may refuse.
#[derive(Debug, thiserror::Error)]
pub enum ErneuerungFehler {
    #[error("die Sitzung ist nicht mehr gültig")]
    SessionUngültig,
    #[error("zu viele Versuche; bitte später erneut versuchen")]
    Gedrosselt,
    #[error("die Sitzung konnte nicht geladen werden")]
    SessionLaden,
    #[error("die Sitzung konnte nicht gespeichert werden")]
    SessionSpeichern,
    #[error("das Refresh-Token konnte nicht erzeugt werden")]
    TokenErzeugen,
}

/// Rotate the refresh token of one Anmelde-Sitzung.
///
/// Every use spends the presented token and mints a successor session. A second use of an
/// already spent token is treated as theft and ends the session family (rule 12).
#[derive(Debug, Clone)]
pub struct Erneuerung {
    pub refresh_token: RefreshToken,
    pub client_key: ClientKey,
}

#[async_trait]
impl UseCase<ErneuerungAusgestellt> for Erneuerung {
    type Error = Report<ErneuerungFehler>;

    async fn perform(
        self,
        ctx: ExecutionContext<'_>,
    ) -> Result<ErneuerungAusgestellt, Self::Error> {
        let jetzt = ctx.clock().now();
        ctx.throttle()
            .check_and_count(&self.client_key, ThrottleAction::Erneuerung, jetzt)
            .await
            .change_context(ErneuerungFehler::Gedrosselt)?;

        let uow = ctx
            .enter()
            .await
            .change_context(ErneuerungFehler::SessionLaden)?;

        let ergebnis = async {
            let session = uow
                .sessions()
                .find_by_refresh_hash(&self.refresh_token.hash())
                .await
                .map_err(IntoReport::into_report)
                .change_context(ErneuerungFehler::SessionLaden)?;

            let Some(mut session) = session else {
                return Err(Report::new(ErneuerungFehler::SessionUngültig));
            };

            let benutzer_id = session.benutzer_id();
            let live = match RenewableSession::try_from(session.cloned_data()) {
                Ok(live) => live,
                Err(SessionNotRenewable::Spent) => {
                    // Replay of a spent token: end the family rather than warn (rule 12).
                    // The revocation belongs to this unit of work, so it commits and the
                    // use case answers with a refusal afterwards.
                    revoke_all_sessions(&uow, benutzer_id, jetzt, || {
                        ErneuerungFehler::SessionSpeichern
                    })
                    .await?;
                    return Ok::<_, Report<ErneuerungFehler>>(Erneuerungsausgang::Wiederholung);
                }
                Err(SessionNotRenewable::Revoked) => {
                    return Err(Report::new(ErneuerungFehler::SessionUngültig));
                }
            };

            if !live.ist_gültig(jetzt) {
                return Err(Report::new(ErneuerungFehler::SessionUngültig));
            }

            let folge_refresh =
                RefreshToken::random().change_context(ErneuerungFehler::TokenErzeugen)?;
            let (verbraucht, folge) = live.erneuern(folge_refresh.hash(), jetzt);

            session.map_data(|_| verbraucht.into());
            uow.sessions()
                .update(&mut session)
                .await
                .map_err(IntoReport::into_report)
                .change_context_lazy(|| ErneuerungFehler::SessionSpeichern)?;

            let folge = uow
                .sessions()
                .create(folge)
                .await
                .map_err(IntoReport::into_report)
                .change_context(ErneuerungFehler::SessionSpeichern)?;

            Ok(Erneuerungsausgang::Neu(ErneuerungAusgestellt {
                refresh_token: folge_refresh,
                claims: claims_for_session(&ctx, benutzer_id, folge.id(), jetzt),
            }))
        }
        .await;

        match uow
            .finish(ergebnis, ErneuerungFehler::SessionSpeichern)
            .await?
        {
            Erneuerungsausgang::Neu(ausgestellt) => Ok(ausgestellt),
            Erneuerungsausgang::Wiederholung => Err(Report::new(ErneuerungFehler::SessionUngültig)),
        }
    }
}

/// Whether a renewal produced a successor session or detected a replay.
enum Erneuerungsausgang {
    Neu(ErneuerungAusgestellt),
    Wiederholung,
}

/// Every way `Abmeldung` may refuse.
#[derive(Debug, thiserror::Error)]
pub enum AbmeldungFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("die Sitzung konnte nicht geladen werden")]
    SessionLaden,
    #[error("die Sitzung konnte nicht gespeichert werden")]
    SessionSpeichern,
}

/// End the current Anmelde-Sitzung. Idempotent: a missing session is already logged out.
#[derive(Debug, Clone, Copy)]
pub struct Abmeldung;

#[async_trait]
impl UseCase<()> for Abmeldung {
    type Error = Report<AbmeldungFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<(), Self::Error> {
        let (_, session_id) = ctx
            .auth()
            .require_authenticated()
            .change_context(AbmeldungFehler::NichtAngemeldet)?;
        let jetzt = ctx.clock().now();

        let uow = ctx
            .enter()
            .await
            .change_context(AbmeldungFehler::SessionLaden)?;

        let ergebnis = async {
            let mut session = match uow.sessions().find_by_id(session_id).await {
                Ok(session) => session,
                Err(fehler) if matches!(fehler.current_context(), RepositoryError::NotFound) => {
                    // Already gone: the caller's intent is satisfied.
                    tracing::debug!("abmeldung: session war bereits beendet");
                    return Ok(());
                }
                Err(fehler) => {
                    return Err(Report::new(AbmeldungFehler::SessionLaden)
                        .attach(fehler.current_context().to_string()));
                }
            };

            if session.ist_widerrufen() {
                return Ok(());
            }
            session.map_data(|session| session.clone().widerrufen(jetzt));
            uow.sessions()
                .update(&mut session)
                .await
                .map_err(IntoReport::into_report)
                .change_context_lazy(|| AbmeldungFehler::SessionSpeichern)?;
            Ok::<_, Report<AbmeldungFehler>>(())
        }
        .await;

        uow.finish(ergebnis, AbmeldungFehler::SessionSpeichern)
            .await
    }
}

/// Every way `SessionPrüfen` may refuse.
#[derive(Debug, thiserror::Error)]
pub enum SessionPrüfenFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("die Sitzung konnte nicht geladen werden")]
    SessionLaden,
    #[error("der Benutzer konnte nicht geladen werden")]
    BenutzerLaden,
}

/// Verify an access token that the API already parsed structurally.
///
/// The signature is checked against the public key stored **on the session row**, not
/// against a key registry: a row issued under another key rejects the token even when that
/// key is valid on its own (AC-17). The `sid` row must exist and be unrevoked, and the
/// `Benutzer` must still exist.
#[derive(Debug, Clone)]
pub struct SessionPrüfen {
    pub claims: AccessTokenClaims,
    /// The ASCII `header.payload` the signature was produced over.
    pub signed_message: String,
    pub signature: Signature,
}

#[async_trait]
impl UseCase<AuthContext> for SessionPrüfen {
    type Error = Report<SessionPrüfenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<AuthContext, Self::Error> {
        let jetzt = ctx.clock().now();
        let konfiguration = ctx.konto_konfiguration();

        self.claims
            .prüfe_herkunft(&konfiguration.issuer, &konfiguration.audience)
            .change_context(SessionPrüfenFehler::NichtAngemeldet)?;
        self.claims
            .prüfe_zeitfenster(jetzt)
            .change_context(SessionPrüfenFehler::NichtAngemeldet)?;

        let uow = ctx
            .enter()
            .await
            .change_context(SessionPrüfenFehler::SessionLaden)?;

        let ergebnis = async {
            let session = match uow.sessions().find_by_id(self.claims.sid).await {
                Ok(session) => session,
                Err(fehler) if matches!(fehler.current_context(), RepositoryError::NotFound) => {
                    // An unknown session is a 401, never a 500.
                    return Err(Report::new(SessionPrüfenFehler::NichtAngemeldet));
                }
                Err(fehler) => {
                    return Err(Report::new(SessionPrüfenFehler::SessionLaden)
                        .attach(fehler.current_context().to_string()));
                }
            };

            // AC-17: the row carries the public half of the key that issued it, so a
            // signature made by any other key is refused here.
            SigningKeypair::verify_with_public_key_hex(
                session.public_key_hex(),
                self.signed_message.as_bytes(),
                &self.signature,
            )
            .change_context(SessionPrüfenFehler::NichtAngemeldet)?;

            if session.benutzer_id() != self.claims.sub || !session.ist_annehmbar(jetzt) {
                return Err(Report::new(SessionPrüfenFehler::NichtAngemeldet));
            }

            // The Benutzer must still exist: a deleted account leaves no live identity.
            uow.benutzer()
                .find_by_id(self.claims.sub)
                .await
                .map_err(IntoReport::into_report)
                .change_context(SessionPrüfenFehler::BenutzerLaden)?;

            Ok::<_, Report<SessionPrüfenFehler>>(AuthContext::Authenticated {
                benutzer_id: self.claims.sub,
                session_id: self.claims.sid,
            })
        }
        .await;

        uow.finish(ergebnis, SessionPrüfenFehler::SessionLaden)
            .await
    }
}

pub(crate) async fn revoke_all_sessions<E, F>(
    uow: &UnitOfWork<'_>,
    benutzer_id: BenutzerId,
    jetzt: DateTime<Utc>,
    fehler: F,
) -> Result<(), Report<E>>
where
    E: ThreadSafeError,
    F: Fn() -> E,
{
    let sessions = uow
        .sessions()
        .find_by_benutzer(benutzer_id)
        .await
        .map_err(IntoReport::into_report)
        .change_context(fehler())?;

    for mut session in sessions {
        if session.ist_widerrufen() {
            continue;
        }
        session.map_data(|session| session.clone().widerrufen(jetzt));
        uow.sessions()
            .update(&mut session)
            .await
            .map_err(IntoReport::into_report)
            .change_context_lazy(&fehler)?;
    }
    Ok(())
}
