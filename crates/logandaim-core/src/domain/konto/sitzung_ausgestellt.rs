use chrono::{DateTime, Utc};

use super::{
    ACCESS_TOKEN_LIFETIME, AccessTokenClaims, Benutzer, BenutzerId, RefreshToken, SessionId,
};

/// What a session-issuing use case hands back to the edge.
///
/// Domain data, not transport: the claims are plain Rust, and the API serialises them into
/// the JOSE envelope. Core never writes JSON or base64url (see `domain/konto/signing.rs`).
/// It lives here rather than in a use-case module because it is the shape of the domain's
/// own output — two use cases (`Anmeldung`, `RegistrierungAbschließen`) return it, and a
/// third (`Erneuerung`) returns its renewal half.
#[derive(Debug, Clone)]
pub struct SitzungAusgestellt {
    pub benutzer: Benutzer,
    pub session_id: SessionId,
    pub refresh_token: RefreshToken,
    pub claims: AccessTokenClaims,
}

/// The claim set of one freshly issued access token.
pub fn claims_for(
    benutzer_id: BenutzerId,
    session_id: SessionId,
    kid: super::SigningKeyId,
    issuer: &str,
    audience: &str,
    jetzt: DateTime<Utc>,
) -> AccessTokenClaims {
    AccessTokenClaims {
        sub: benutzer_id,
        sid: session_id,
        kid,
        iss: issuer.to_owned(),
        aud: audience.to_owned(),
        iat: jetzt,
        exp: jetzt + ACCESS_TOKEN_LIFETIME,
        jti: uuid::Uuid::new_v4().to_string(),
    }
}
