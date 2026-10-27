//! The JOSE envelope of an access token — and nothing else.
//!
//! Core owns the algorithm (`Ed25519`), the key material, and the claim *semantics*: it
//! signs and verifies raw bytes. This module owns the transport: JSON, base64url, the
//! JOSE header, and the `alg` pin. That is why `serde` and `base64` live here and not in
//! `logandaim-core`.
//!
//! **Decoding does not verify a signature.** The key a token must be checked against is
//! the public key stored on its session row, and only the database knows that. So `decode`
//! returns the claims *plus* the raw `header.payload` and the signature, and `SessionPrüfen`
//! verifies against the row. That is what makes AC-17 a row check rather than a
//! configuration check.
//!
//! Verify order inside `SessionPrüfen`: `alg` is pinned to `EdDSA` before the payload is
//! decoded, then the Ed25519 signature over the ASCII `header.payload` is checked against
//! the public key on the session row, then the row's `Benutzer` and lifecycle state. Every
//! failure is 401; the caller never learns which step failed.

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use error_stack::Report;
use logandaim_core::{
    ResultReport,
    application::AccessError,
    domain::konto::{AccessToken, AccessTokenClaims, BenutzerId, SessionId, Signature},
    ports::Signing,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The only `alg` this deployment issues or accepts.
pub const SIGNATUR_ALGORITHMUS: &str = "EdDSA";

/// Claims as they travel on the wire. The plain-Rust twin in core carries the semantics;
/// this one exists solely to be (de)serialized, so it names raw `Uuid`s and converts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenClaimsDto {
    pub sub: Uuid,
    pub sid: Uuid,
    pub iss: String,
    pub aud: String,
    pub iat: DateTime<Utc>,
    pub exp: DateTime<Utc>,
    pub jti: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JoseHeader {
    alg: String,
    typ: String,
}

/// A structurally valid token that still has to be checked against its session row.
///
/// The signature is *not* verified here: the key lives on the session row, so verification
/// is `SessionPrüfen`'s job.
#[derive(Debug, Clone)]
pub struct DecodedToken {
    pub claims: AccessTokenClaims,
    /// The ASCII `header.payload` the signature was produced over.
    pub signed_message: String,
    pub signature: Signature,
}

fn ungültig() -> Report<AccessError> {
    Report::new(AccessError::NotAuthenticated)
}

/// Encode `claims` into a signed compact JWS.
///
/// The signature is produced by the domain over the raw `header.payload` ASCII, so a
/// different algorithm cannot be swapped in at this layer.
pub fn encode(
    claims: &AccessTokenClaims,
    signing: &dyn Signing,
) -> ResultReport<AccessToken, AccessError> {
    let header = JoseHeader {
        alg: SIGNATUR_ALGORITHMUS.to_owned(),
        typ: "JWT".to_owned(),
    };
    let dto = AccessTokenClaimsDto {
        sub: claims.sub.0,
        sid: claims.sid.0,
        iss: claims.iss.clone(),
        aud: claims.aud.clone(),
        iat: claims.iat,
        exp: claims.exp,
        jti: claims.jti.clone(),
    };

    let header = serde_json::to_vec(&header).map_err(|_| ungültig())?;
    let payload = serde_json::to_vec(&dto).map_err(|_| ungültig())?;

    let mut signierte_nachricht = String::with_capacity(header.len() + payload.len() + 1);
    signierte_nachricht.push_str(&URL_SAFE_NO_PAD.encode(&header));
    signierte_nachricht.push('.');
    signierte_nachricht.push_str(&URL_SAFE_NO_PAD.encode(&payload));

    let signatur = signing.sign(signierte_nachricht.as_bytes());
    let token = format!(
        "{signierte_nachricht}.{}",
        URL_SAFE_NO_PAD.encode(signatur.as_bytes())
    );
    Ok(AccessToken::new(token))
}

/// Parse a compact JWS into its claims and the bytes plus signature to verify.
///
/// Structure and the `alg`/`typ` pin are checked; nothing about the key, because the key is
/// per session row and only the database holds it.
pub fn decode(token: &str) -> ResultReport<DecodedToken, AccessError> {
    let mut teile = token.split('.');
    let (Some(header_b64), Some(payload_b64), Some(signatur_b64)) =
        (teile.next(), teile.next(), teile.next())
    else {
        return Err(ungültig());
    };
    if teile.next().is_some() {
        return Err(ungültig());
    }

    let header_roh = URL_SAFE_NO_PAD.decode(header_b64).map_err(|_| ungültig())?;
    let header: JoseHeader = serde_json::from_slice(&header_roh).map_err(|_| ungültig())?;

    // Pin the algorithm before anything in the payload is trusted. `none` and every other
    // value fail here, including a well-formed header naming HS256.
    if header.alg != SIGNATUR_ALGORITHMUS || header.typ != "JWT" {
        return Err(ungültig());
    }

    let payload_roh = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|_| ungültig())?;
    let dto: AccessTokenClaimsDto = serde_json::from_slice(&payload_roh).map_err(|_| ungültig())?;

    let signatur_bytes = URL_SAFE_NO_PAD
        .decode(signatur_b64)
        .map_err(|_| ungültig())?;
    let signatur = Signature::from_bytes(&signatur_bytes).map_err(|_| ungültig())?;
    let signierte_nachricht = token[..header_b64.len() + 1 + payload_b64.len()].to_owned();

    Ok(DecodedToken {
        claims: AccessTokenClaims {
            sub: BenutzerId(dto.sub),
            sid: SessionId(dto.sid),
            iss: dto.iss,
            aud: dto.aud,
            iat: dto.iat,
            exp: dto.exp,
            jti: dto.jti,
        },
        signed_message: signierte_nachricht,
        signature: signatur,
    })
}

/// Split one `Authorization` header value into the bearer token.
///
/// `None` when the scheme is not `Bearer` or the value is empty. The header is split once,
/// so a token containing a space cannot smuggle a second scheme.
pub fn bearer_token(kopf: &str) -> Option<&str> {
    let (schema, wert) = kopf.split_once(' ')?;
    if !schema.eq_ignore_ascii_case("Bearer") {
        return None;
    }
    let wert = wert.trim();
    (!wert.is_empty()).then_some(wert)
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;
    use logandaim_core::adapters::InMemorySigning;
    use logandaim_core::domain::konto::{ACCESS_TOKEN_LIFETIME, SigningKeypair};

    fn keypair() -> InMemorySigning {
        InMemorySigning::from_secret_bytes(&[7u8; 32])
    }

    fn claims() -> AccessTokenClaims {
        let jetzt = DateTime::from_timestamp(1_700_000_000, 0).expect("fixed instant");
        AccessTokenClaims {
            sub: BenutzerId(Uuid::nil()),
            sid: SessionId(Uuid::nil()),
            iss: "logandaim.at".into(),
            aud: "logandaim-api".into(),
            iat: jetzt,
            exp: jetzt + ACCESS_TOKEN_LIFETIME,
            jti: "jti".into(),
        }
    }

    /// The same call `SessionPrüfen` makes, with the key a session row would carry.
    fn verify(gelesen: &DecodedToken, signing: &InMemorySigning) -> bool {
        SigningKeypair::verify_with_public_key_hex(
            &signing.public_key_hex(),
            gelesen.signed_message.as_bytes(),
            &gelesen.signature,
        )
        .is_ok()
    }

    #[test_log::test]
    fn encode_und_decode_roundtrip() {
        let keypair = keypair();
        let original = claims();
        let token = encode(&original, &keypair).expect("encoding succeeds");
        let gelesen = decode(token.as_str()).expect("decoding succeeds");
        assert_eq!(gelesen.claims, original);
        assert!(verify(&gelesen, &keypair));
    }

    #[test_log::test]
    fn manipuliertes_byte_wird_abgelehnt() {
        let keypair = keypair();
        let token = encode(&claims(), &keypair).expect("encoding succeeds");
        let mut bytes = token.as_str().as_bytes().to_vec();
        let punkt = bytes
            .iter()
            .position(|byte| *byte == b'.')
            .expect("there is a separator");
        bytes[punkt + 2] = if bytes[punkt + 2] == b'A' { b'B' } else { b'A' };
        let manipuliert = String::from_utf8(bytes).expect("still ASCII");
        // Structure may survive; the signature does not.
        if let Ok(gelesen) = decode(&manipuliert) {
            assert!(!verify(&gelesen, &keypair));
        }
    }

    #[test_log::test]
    fn alg_none_wird_abgelehnt() {
        let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"none","typ":"JWT"}"#);
        let payload = URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&AccessTokenClaimsDto {
                sub: Uuid::nil(),
                sid: Uuid::nil(),
                iss: "logandaim.at".into(),
                aud: "logandaim-api".into(),
                iat: DateTime::from_timestamp(1_700_000_000, 0).unwrap(),
                exp: DateTime::from_timestamp(1_700_086_400, 0).unwrap(),
                jti: "jti".into(),
            })
            .expect("dto serializes"),
        );
        let token = format!("{header}.{payload}.");
        assert!(decode(&token).is_err());
    }

    #[test_log::test]
    fn hs256_wird_abgelehnt() {
        let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256","typ":"JWT"}"#);
        let payload = URL_SAFE_NO_PAD.encode(br#"{"sub":"00000000-0000-0000-0000-000000000000"}"#);
        let token = format!("{header}.{payload}.AAAA");
        assert!(decode(&token).is_err());
    }

    #[test_log::test]
    fn fremder_schlüssel_wird_abgelehnt() {
        let keypair = keypair();
        let token = encode(&claims(), &keypair).expect("encoding succeeds");
        let gelesen = decode(token.as_str()).expect("structure is fine");

        // Different material: the row's public key is what rejects it.
        let gleichnamig = InMemorySigning::from_secret_bytes(&[9u8; 32]);
        assert!(!verify(&gelesen, &gleichnamig));
    }

    #[test_log::test]
    fn strukturell_kaputte_tokens_werden_abgelehnt() {
        for token in ["", "abc", "a.b", "a.b.c.d", ".."] {
            assert!(decode(token).is_err(), "{token} wurde akzeptiert");
        }
    }

    #[test_log::test]
    fn bearer_parsing() {
        assert_eq!(bearer_token("Bearer abc"), Some("abc"));
        assert_eq!(bearer_token("bearer abc"), Some("abc"));
        assert_eq!(bearer_token("Basic abc"), None);
        assert_eq!(bearer_token("Bearer "), None);
        assert_eq!(bearer_token("abc"), None);
        // Only one split: a second scheme cannot be smuggled in.
        assert_eq!(bearer_token("Bearer a b"), Some("a b"));
    }

    #[test_log::test]
    fn access_token_lebensdauer_fließt_in_exp() {
        let claims = claims();
        assert_eq!(claims.exp - claims.iat, ACCESS_TOKEN_LIFETIME);
        assert_eq!(ACCESS_TOKEN_LIFETIME, Duration::hours(24));
    }
}
