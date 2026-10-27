//! Use cases, grouped by journey in English file names.
//!
//! The type names stay German because they name domain nouns (`Anmeldung`,
//! `RegistrierungStarten`, `PasswortVergessen`); the modules are application structure and
//! are English.
//!
//! Each use case owns its own error enum. A variant names what the use case could not do,
//! keeps the underlying repository or crypto error in the `error_stack` chain, and has at
//! most one HTTP meaning — `logandaim-api` maps them in `errors/http_status.rs`.

mod login;
mod password;
mod registration;
mod schuss;
mod session;
mod user;

pub use login::*;
pub use password::*;
pub use registration::*;
pub use schuss::*;
pub use session::*;
pub use user::*;

use chrono::{DateTime, Utc};
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    application::{
        ExecutionContext, ThreadSafeError,
        uow::{UnitOfWork, Versioned},
    },
    domain::konto::{
        AccessTokenClaims, BenutzerId, NeueSession, RefreshToken, Session, SessionId,
        claims_for as domain_claims,
    },
    ports::Mail,
};

/// The creation input of one session, carrying the issuing key's public half so the row
/// itself is enough to verify its access tokens (AC-17).
pub(crate) fn new_session(
    ctx: &ExecutionContext<'_>,
    benutzer_id: BenutzerId,
    refresh_token: &RefreshToken,
    jetzt: DateTime<Utc>,
) -> NeueSession {
    NeueSession::neu(
        benutzer_id,
        refresh_token.hash(),
        ctx.signing().kid().clone(),
        ctx.signing().public_key_hex(),
        jetzt,
    )
}

/// The claim set of one freshly issued access token.
pub(crate) fn claims_for_session(
    ctx: &ExecutionContext<'_>,
    benutzer_id: BenutzerId,
    session_id: SessionId,
    jetzt: DateTime<Utc>,
) -> AccessTokenClaims {
    let konfiguration = ctx.konto_konfiguration();
    domain_claims(
        benutzer_id,
        session_id,
        ctx.signing().kid().clone(),
        &konfiguration.issuer,
        &konfiguration.audience,
        jetzt,
    )
}

/// Apply a domain transition to a loaded session and write it back.
///
/// The row goes in as it was loaded, is mutated in place, and goes out through
/// [`Versioned`]: the adapter writes it only while its version is still current, so a
/// concurrent writer surfaces as `VersionMismatch` instead of being silently overwritten.
pub(crate) async fn update_session<E, F, M>(
    uow: &UnitOfWork<'_>,
    mut geladen: Versioned<Session>,
    mutiere: M,
    fehler: F,
) -> Result<(), Report<E>>
where
    E: ThreadSafeError,
    F: Fn() -> E,
    M: FnOnce(Session) -> Session,
{
    *geladen = mutiere(geladen.cloned_data());
    uow.sessions()
        .update(&mut geladen)
        .await
        .map_err(IntoReport::into_report)
        .change_context(fehler())?;
    Ok(())
}

/// End every session of one Benutzer: a spent refresh token (replay, rule 12) and a
/// password recovery (rule 6) both need this.
///
/// Load-then-update per aggregate, one by one — the repository has no bulk mutator, because
/// "revoke every session" is a decision of the use case, not a `UPDATE` to hide in a port.
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

    for session in sessions {
        if session.ist_widerrufen() {
            continue;
        }
        update_session(uow, session, |geladen| geladen.widerrufen(jetzt), &fehler).await?;
    }
    Ok(())
}

/// Send one mail after the transaction committed. A failure is a warning, never an error:
/// the account state is already durable and retrying is the operator's job (NFR-5).
pub(crate) async fn send_mail(ctx: &ExecutionContext<'_>, mail: Mail) {
    if let Err(fehler) = ctx.mail_sender().send(mail).await {
        tracing::warn!(
            fehler = %fehler,
            "Transaktionsmail konnte nicht zugestellt werden"
        );
    }
}
