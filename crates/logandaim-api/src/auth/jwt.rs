//! The JOSE envelope of an access token — and nothing else.
//!
//! Core owns the algorithm (`Ed25519`), the key material, and the claim *semantics*: it
//! signs and verifies raw bytes. This module owns the transport: JSON, base64url, the
//! JOSE header, the `alg` pin, and the `kid` comparison. That is why `serde` and `base64`
//! live here and not in `logandaim-core`.
//!
//! Verify order matters and is asserted by the tests below:
//!
//! 1. structure (`header.payload.signature`),
//! 2. `alg` **pinned to `EdDSA` before the payload is decoded** — `none` and every other
//!    algorithm are rejected here, never by a "supported algorithms" list,
//! 3. `kid` equal to the configured key's id,
//! 4. the Ed25519 signature over the ASCII `header.payload`,
//! 5. only then deserialize the claims.
//!
//! Every failure is [`ZugriffFehler::NichtAngemeldet`]; the caller answers 401 and never
//! learns which step failed.

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use error_stack::Report;
use logandaim_core::{
    ResultReport,
    application::ZugriffFehler,
    domain::konto::{
        AccessToken, AccessTokenClaims, BenutzerId, SessionId, Signature, SigningKeyId,
        SigningKeypair,
    },
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
    pub kid: String,
    pub iss: String,
    pub aud: String,
    pub iat: DateTime<Utc>,
    pub exp: DateTime<Utc>,
    pub jti: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JoseHeader {
    alg: String,
    kid: String,
    typ: String,
}

fn ungültig() -> Report<ZugriffFehler> {
    Report::new(ZugriffFehler::NichtAngemeldet)
}

/// Encode `claims` into a signed compact JWS.
///
/// The signature is produced by the domain over the raw `header.payload` ASCII, so a
/// different algorithm cannot be swapped in at this layer.
pub fn encode(
    claims: &AccessTokenClaims,
    signing_key: &SigningKeypair,
) -> ResultReport<AccessToken, ZugriffFehler> {
    let header = JoseHeader {
        alg: SIGNATUR_ALGORITHMUS.to_owned(),
        kid: signing_key.kid().as_str().to_owned(),
        typ: "JWT".to_owned(),
    };
    let dto = AccessTokenClaimsDto {
        sub: claims.sub.0,
        sid: claims.sid.0,
        kid: claims.kid.as_str().to_owned(),
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

    let signatur = signing_key.sign(signierte_nachricht.as_bytes());
    let token = format!(
        "{signierte_nachricht}.{}",
        URL_SAFE_NO_PAD.encode(signatur.as_bytes())
    );
    Ok(AccessToken::new(token))
}

/// Verify a compact JWS and hand back the claims core must then check semantically
/// (window, issuer, audience, live session row).
pub fn decode(
    token: &str,
    signing_key: &SigningKeypair,
) -> ResultReport<AccessTokenClaims, ZugriffFehler> {
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

    // Step 2: pin the algorithm before anything in the payload is trusted. `none` and
    // every other value fail here, including a well-formed header naming HS256.
    if header.alg != SIGNATUR_ALGORITHMUS || header.typ != "JWT" {
        return Err(ungültig());
    }

    // Step 3: the header must name the key this deployment holds.
    if header.kid != signing_key.kid().as_str() {
        return Err(ungültig());
    }

    // Step 4: signature over the raw ASCII, verified by the domain.
    let signatur_bytes = URL_SAFE_NO_PAD
        .decode(signatur_b64)
        .map_err(|_| ungültig())?;
    let signatur = Signature::from_bytes(&signatur_bytes).map_err(|_| ungültig())?;
    let signierte_nachricht = &token[..header_b64.len() + 1 + payload_b64.len()];
    signing_key
        .verify(signierte_nachricht.as_bytes(), &signatur)
        .map_err(|_| ungültig())?;

    // Step 5: only now is the payload parsed.
    let payload_roh = URL_SAFE_NO_PAD
        .decode(payload_b64)
        .map_err(|_| ungültig())?;
    let dto: AccessTokenClaimsDto = serde_json::from_slice(&payload_roh).map_err(|_| ungültig())?;

    let kid = SigningKeyId::new(dto.kid).map_err(|_| ungültig())?;

    Ok(AccessTokenClaims {
        sub: BenutzerId(dto.sub),
        sid: SessionId(dto.sid),
        kid,
        iss: dto.iss,
        aud: dto.aud,
        iat: dto.iat,
        exp: dto.exp,
        jti: dto.jti,
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
    use logandaim_core::domain::konto::ACCESS_TOKEN_LIFETIME;

    fn keypair() -> SigningKeypair {
        SigningKeypair::from_secret_bytes(
            SigningKeyId::new("test-key").expect("static kid"),
            &[7u8; 32],
        )
    }

    fn claims() -> AccessTokenClaims {
        let jetzt = DateTime::from_timestamp(1_700_000_000, 0).expect("fixed instant");
        AccessTokenClaims {
            sub: BenutzerId(Uuid::nil()),
            sid: SessionId(Uuid::nil()),
            kid: SigningKeyId::new("test-key").expect("static kid"),
            iss: "logandaim.at".into(),
            aud: "logandaim-api".into(),
            iat: jetzt,
            exp: jetzt + ACCESS_TOKEN_LIFETIME,
            jti: "jti".into(),
        }
    }

    #[test_log::test]
    fn encode_und_decode_roundtrip() {
        let keypair = keypair();
        let original = claims();
        let token = encode(&original, &keypair).expect("encoding succeeds");
        let gelesen = decode(token.as_str(), &keypair).expect("decoding succeeds");
        assert_eq!(gelesen, original);
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
        assert!(decode(&manipuliert, &keypair).is_err());
    }

    #[test_log::test]
    fn alg_none_wird_abgelehnt() {
        let keypair = keypair();
        let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"none","kid":"test-key","typ":"JWT"}"#);
        let payload = URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&AccessTokenClaimsDto {
                sub: Uuid::nil(),
                sid: Uuid::nil(),
                kid: "test-key".into(),
                iss: "logandaim.at".into(),
                aud: "logandaim-api".into(),
                iat: DateTime::from_timestamp(1_700_000_000, 0).unwrap(),
                exp: DateTime::from_timestamp(1_700_086_400, 0).unwrap(),
                jti: "jti".into(),
            })
            .expect("dto serializes"),
        );
        let token = format!("{header}.{payload}.");
        assert!(decode(&token, &keypair).is_err());
    }

    #[test_log::test]
    fn hs256_wird_abgelehnt() {
        let keypair = keypair();
        let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"HS256","kid":"test-key","typ":"JWT"}"#);
        let payload = URL_SAFE_NO_PAD.encode(br#"{"sub":"00000000-0000-0000-0000-000000000000"}"#);
        let token = format!("{header}.{payload}.AAAA");
        assert!(decode(&token, &keypair).is_err());
    }

    #[test_log::test]
    fn fremder_kid_wird_abgelehnt() {
        let keypair = keypair();
        let token = encode(&claims(), &keypair).expect("encoding succeeds");

        let fremd = SigningKeypair::from_secret_bytes(
            SigningKeyId::new("other-key").expect("static kid"),
            &[9u8; 32],
        );
        assert!(decode(token.as_str(), &fremd).is_err());
    }

    #[test_log::test]
    fn fremder_schlüssel_wird_abgelehnt() {
        let keypair = keypair();
        let token = encode(&claims(), &keypair).expect("encoding succeeds");

        // Same kid, different material: the signature check is what rejects it.
        let gleichnamig = SigningKeypair::from_secret_bytes(
            SigningKeyId::new("test-key").expect("static kid"),
            &[9u8; 32],
        );
        assert!(decode(token.as_str(), &gleichnamig).is_err());
    }

    #[test_log::test]
    fn strukturell_kaputte_tokens_werden_abgelehnt() {
        let keypair = keypair();
        for token in ["", "abc", "a.b", "a.b.c.d", ".."] {
            assert!(decode(token, &keypair).is_err(), "{token} wurde akzeptiert");
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
