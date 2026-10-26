//! Token material of the `konto` subdomain.
//!
//! Two kinds of secret live here:
//!
//! - **SecretToken** — a 256-bit CSPRNG value that only ever leaves the process inside a
//!   mail or an `HttpOnly` cookie. The database stores its SHA-256 hash and nothing else,
//!   so a database read cannot mint a valid token.
//! - **AccessToken** — the already-assembled JWT string. Core never builds or parses one;
//!   it travels opaquely so use cases can hand it back to the API edge unchanged.

use std::fmt;

use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use zeroize::Zeroizing;

use super::KontoFehler;

/// Number of random bytes behind every mailed or cookie-borne token.
const SECRET_TOKEN_BYTES: usize = 32;

/// A token as it exists in the mail or in the cookie. Never logged, never stored.
#[derive(Clone)]
pub struct SecretToken(Zeroizing<String>);

impl SecretToken {
    fn aus_basis64url(wert: String) -> Self {
        Self(Zeroizing::new(wert))
    }

    /// Parse a token that arrived from the outside (mail link, cookie, request body).
    ///
    /// Only shape is checked here; whether it names a live row is the use case's job.
    pub fn parse(wert: impl Into<String>) -> Result<Self, KontoFehler> {
        let wert = wert.into();
        let plausibel = wert.len() == BASIS64URL_LAENGE
            && wert
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_');
        if !plausibel {
            return Err(KontoFehler::TokenUngueltig);
        }
        Ok(Self::aus_basis64url(wert))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// SHA-256 of the token, as stored.
    pub fn hash(&self) -> TokenHash {
        hash_token(self)
    }
}

impl fmt::Debug for SecretToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretToken([redacted])")
    }
}

/// 32 random bytes encode to 43 base64url characters without padding.
const BASIS64URL_LAENGE: usize = 43;

/// SHA-256 digest of a [`SecretToken`], hex encoded: the only form that is stored.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenHash(String);

impl TokenHash {
    pub fn from_hex(hex: impl Into<String>) -> Result<Self, KontoFehler> {
        let hex = hex.into();
        let plausibel = hex.len() == 64 && hex.bytes().all(|byte| byte.is_ascii_hexdigit());
        if !plausibel {
            return Err(KontoFehler::TokenUngueltig);
        }
        Ok(Self(hex.to_ascii_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Constant-time comparison, so a lookup that compares hashes leaks no timing signal.
pub fn token_hashes_equal(links: &TokenHash, rechts: &TokenHash) -> bool {
    links.0.as_bytes().ct_eq(rechts.0.as_bytes()).into()
}

/// Fresh 256-bit secret, base64url encoded without padding.
pub fn generate_secret_token() -> Result<SecretToken, KontoFehler> {
    let mut bytes = [0u8; SECRET_TOKEN_BYTES];
    getrandom::fill(&mut bytes).map_err(|_| KontoFehler::ZufallFehlgeschlagen)?;
    Ok(SecretToken::aus_basis64url(basis64url(&bytes)))
}

/// SHA-256 over the token text. Microseconds, unlike a Passwort.
pub fn hash_token(token: &SecretToken) -> TokenHash {
    let digest = Sha256::digest(token.as_str().as_bytes());
    TokenHash(hex::encode(digest))
}

/// An access token already assembled at the HTTP edge.
///
/// Opaque to core by design: JWT envelope, base64url, and JSON are transport, and only
/// the API layer ever touches them (see the module doc of `signing.rs`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(wert: impl Into<String>) -> Self {
        Self(wert.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// The refresh token of one Anmelde-Sitzung, as handed to the client.
///
/// The client sees this only through the `__Host-` cookie; core hands it out and takes it
/// back, and stores nothing but its [`TokenHash`].
#[derive(Debug, Clone)]
pub struct RefreshToken(SecretToken);

impl RefreshToken {
    pub fn neu() -> Result<Self, KontoFehler> {
        Ok(Self(generate_secret_token()?))
    }

    pub fn parse(wert: impl Into<String>) -> Result<Self, KontoFehler> {
        Ok(Self(SecretToken::parse(wert)?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn hash(&self) -> TokenHash {
        self.0.hash()
    }
}

fn basis64url(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(ALPHABET[(triple >> 18) as usize & 0x3f] as char);
        out.push(ALPHABET[(triple >> 12) as usize & 0x3f] as char);
        if chunk.len() > 1 {
            out.push(ALPHABET[(triple >> 6) as usize & 0x3f] as char);
        }
        if chunk.len() > 2 {
            out.push(ALPHABET[triple as usize & 0x3f] as char);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn secret_token_hat_256_bit_und_basis64url_form() {
        let token = generate_secret_token().unwrap();
        assert_eq!(token.as_str().len(), BASIS64URL_LAENGE);
        assert!(SecretToken::parse(token.as_str()).is_ok());
    }

    #[test_log::test]
    fn zwei_tokens_sind_verschieden() {
        let erster = generate_secret_token().unwrap();
        let zweiter = generate_secret_token().unwrap();
        assert_ne!(erster.as_str(), zweiter.as_str());
    }

    #[test_log::test]
    fn token_hash_ist_sha256_hex() {
        let token = SecretToken::parse("A".repeat(43)).unwrap();
        let hash = hash_token(&token);
        assert_eq!(hash.as_str().len(), 64);
        // SHA-256 of 43 ASCII 'A's.
        let erwartet = hex::encode(Sha256::digest("A".repeat(43).as_bytes()));
        assert_eq!(hash.as_str(), erwartet);
    }

    #[test_log::test]
    fn hash_ist_stabil_und_vergleicht_konstantzeitig() {
        let token = generate_secret_token().unwrap();
        let erster = token.hash();
        let zweiter = token.hash();
        assert!(token_hashes_equal(&erster, &zweiter));
        assert!(!token_hashes_equal(
            &erster,
            &TokenHash::from_hex("0".repeat(64)).unwrap()
        ));
    }

    #[test_log::test]
    fn falsch_geformte_tokens_werden_abgelehnt() {
        assert!(SecretToken::parse("zu-kurz").is_err());
        assert!(SecretToken::parse("!".repeat(43)).is_err());
        assert!(SecretToken::parse("A".repeat(44)).is_err());
        assert!(TokenHash::from_hex("zz".repeat(32)).is_err());
    }

    #[test_log::test]
    fn debug_gibt_das_token_nicht_preis() {
        let token = generate_secret_token().unwrap();
        let text = format!("{token:?}");
        assert!(!text.contains(token.as_str()));
        assert!(text.contains("redacted"));
    }

    #[test_log::test]
    fn refresh_token_roundtrip() {
        let token = RefreshToken::neu().unwrap();
        let geladen = RefreshToken::parse(token.as_str()).unwrap();
        assert_eq!(token.as_str(), geladen.as_str());
        assert_eq!(token.hash(), geladen.hash());
    }
}
