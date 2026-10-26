//! Use cases of the `konto` subdomain.
//!
//! One error enum for the whole subdomain, on purpose: every variant is a distinct
//! *refusal* with exactly one HTTP meaning, and authorisation is decided by each use case
//! from the aggregates it loads (SPEC-konto rule 17). Splitting it per struct would
//! duplicate the same five refusals ten times without making any of them more precise.
//!
//! Mail is sent **after** the unit of work commits, and a failing send is logged, never
//! propagated (NFR-5): a slow SMTP server must not fail a registration.

mod anmeldung;
mod benutzer;
mod passwort;
mod registrierung;

pub use anmeldung::*;
pub use benutzer::*;
pub use passwort::*;
pub use registrierung::*;

use chrono::{DateTime, Utc};
use error_stack::ResultExt;
use uuid::Uuid;

use crate::{
    application::{ExecutionContext, ResultReport},
    domain::konto::{
        ACCESS_TOKEN_LIFETIME, AccessTokenClaims, Benutzer, BenutzerId, KontoFehler, RefreshToken,
        SessionId, SigningKeyId,
    },
    ports::Mail,
};

/// Every way a `konto` use case may refuse.
#[derive(Debug, thiserror::Error)]
pub enum KontoDienstFehler {
    #[error("die Anfrage ist ungültig")]
    UngültigeAnfrage,
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("E-Mail-Adresse oder Passwort ist falsch")]
    ZugangsdatenFalsch,
    #[error("dieser Benutzer darf diese Aktion nicht ausführen")]
    NichtErlaubt,
    #[error("die Sitzung ist nicht mehr gültig")]
    SitzungUngültig,
    #[error("zu viele Versuche; bitte später erneut versuchen")]
    Gedrosselt,
    #[error("der Datensatz wurde zwischenzeitlich geändert")]
    Konflikt,
    #[error("der Vorgang konnte nicht abgeschlossen werden")]
    Speicher,
}

impl From<KontoFehler> for KontoDienstFehler {
    fn from(fehler: KontoFehler) -> Self {
        match fehler {
            KontoFehler::TokenUngültig
            | KontoFehler::TokenVerbraucht
            | KontoFehler::TokenAbgelaufen
            | KontoFehler::TokenNochNichtGültig
            | KontoFehler::TokenHerkunftUngültig
            | KontoFehler::SignaturUngültig
            | KontoFehler::PasswortLängeUngültig { .. } => Self::UngültigeAnfrage,
            KontoFehler::SigningKeyUngültig
            | KontoFehler::RolleUnbekannt
            | KontoFehler::PasswortHashFehler
            | KontoFehler::PasswortHashUngültig
            | KontoFehler::ZufallFehlgeschlagen => Self::Speicher,
        }
    }
}

/// What a session-issuing use case hands back to the edge.
///
/// The claims are plain Rust: the API serialises them into the JOSE envelope, core never
/// writes JSON or base64url (see `domain/konto/signing.rs`).
#[derive(Debug, Clone)]
pub struct SitzungAusgestellt {
    pub benutzer: Benutzer,
    pub session_id: SessionId,
    pub refresh_token: RefreshToken,
    pub claims: AccessTokenClaims,
}

/// Build the claim set of one freshly issued access token.
pub(crate) fn claims_fuer(
    ctx: &ExecutionContext<'_>,
    benutzer_id: BenutzerId,
    session_id: SessionId,
    jetzt: DateTime<Utc>,
) -> AccessTokenClaims {
    let konfiguration = ctx.konto_konfiguration();
    AccessTokenClaims {
        sub: benutzer_id,
        sid: session_id,
        kid: ctx.signing_key().kid().clone(),
        iss: konfiguration.issuer.clone(),
        aud: konfiguration.audience.clone(),
        iat: jetzt,
        exp: jetzt + ACCESS_TOKEN_LIFETIME,
        jti: Uuid::new_v4().to_string(),
    }
}

/// Active signing key id of this deployment, for a new session row.
pub(crate) fn signing_key_id(ctx: &ExecutionContext<'_>) -> SigningKeyId {
    ctx.signing_key().kid().clone()
}

/// Send one mail after the transaction committed. A failure is a warning, never an error:
/// the account state is already durable and retrying is the operator's job (NFR-5).
pub(crate) async fn sende_mail(ctx: &ExecutionContext<'_>, mail: Mail) {
    if let Err(fehler) = ctx.mail_sender().senden(mail).await {
        tracing::warn!(
            fehler = %fehler,
            "Transaktionsmail konnte nicht zugestellt werden"
        );
    }
}

/// Count one throttled attempt, or refuse.
pub(crate) async fn drossle(
    ctx: &ExecutionContext<'_>,
    kunde: &crate::ports::KundeSchlüssel,
    aktion: crate::ports::DrosselAktion,
    jetzt: DateTime<Utc>,
) -> ResultReport<(), KontoDienstFehler> {
    ctx.drossel()
        .prüfe_und_zähle(kunde, aktion, jetzt)
        .await
        .change_context(KontoDienstFehler::Gedrosselt)
}

/// Repository failures are internal; only a lost race becomes a `Konflikt`.
pub(crate) fn speicherfehler(
    fehler: &error_stack::Report<crate::ports::RepositoryError>,
) -> KontoDienstFehler {
    match fehler.current_context() {
        crate::ports::RepositoryError::Conflict => KontoDienstFehler::Konflikt,
        _ => KontoDienstFehler::Speicher,
    }
}
