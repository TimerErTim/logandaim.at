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
    }, domain::konto::{
        AccessTokenClaims, Benutzer, BenutzerId, NeueSession, RefreshToken, Session, SessionId
    }, ports::Mail,
};

#[derive(Debug, Clone)]
pub struct SessionAusgestellt {
    pub benutzer: Benutzer,
    pub session_id: SessionId,
    pub refresh_token: RefreshToken,
    pub claims: AccessTokenClaims,
}

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
    AccessTokenClaims::neu(
        benutzer_id,
        session_id,
        ctx.signing().kid().clone(),
        &konfiguration.issuer,
        &konfiguration.audience,
        jetzt,
    )
}

