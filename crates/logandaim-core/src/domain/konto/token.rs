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
use zeroize::Zeroizing;

use super::KontoFehler;

/// Number of random bytes behind every mailed or cookie-borne token.
const SECRET_TOKEN_BYTES: usize = 32;

/// 32 random bytes encode to 43 base64url characters without padding.
const BASIS64URL_LÄNGE: usize = 43;

/// A token as it exists in the mail or in the cookie. Never logged, never stored.
///
/// The value is held as raw bytes, not as text: the base64url spelling is derived on
/// demand, so no parse step can leave a half-decoded token behind, and the byte buffer is
/// zeroized when the token is dropped.
#[derive(Clone)]
pub struct SecretToken(Zeroizing<[u8; SECRET_TOKEN_BYTES]>);

impl SecretToken {
    /// Fresh 256-bit secret.
    pub fn random() -> Result<Self, KontoFehler> {
        let mut bytes = [0u8; SECRET_TOKEN_BYTES];
        getrandom::fill(&mut bytes).map_err(|_| KontoFehler::ZufallFehlgeschlagen)?;
        Ok(Self(Zeroizing::new(bytes)))
    }

    /// The base64url (unpadded) spelling a client sees in a link or a cookie.
    pub fn base64url(&self) -> Zeroizing<String> {
        Zeroizing::new(basis64url(&self.0[..]))
    }

    /// SHA-256 of the token, as stored.
    pub fn hash(&self) -> SecretTokenHash {
        SecretTokenHash::from(self)
    }
}

impl fmt::Debug for SecretToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretToken([redacted])")
    }
}

impl TryFrom<&str> for SecretToken {
    type Error = KontoFehler;

    /// Parse a token that arrived from the outside (mail link, cookie, request body).
    ///
    /// Only shape is checked here; whether it names a live row is the use case's job.
    fn try_from(wert: &str) -> Result<Self, Self::Error> {
        if wert.len() != BASIS64URL_LÄNGE
            || !wert
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
        {
            return Err(KontoFehler::TokenUngültig);
        }
        let mut bytes = [0u8; SECRET_TOKEN_BYTES];
        basis64url_decode(wert, &mut bytes)?;
        Ok(Self(Zeroizing::new(bytes)))
    }
}

impl TryFrom<String> for SecretToken {
    type Error = KontoFehler;

    fn try_from(wert: String) -> Result<Self, Self::Error> {
        Self::try_from(wert.as_str())
    }
}

/// SHA-256 digest of a [`SecretToken`], hex encoded: the only form that is stored.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SecretTokenHash(String);

impl SecretTokenHash {
    pub fn from_hex(hex: impl Into<String>) -> Result<Self, KontoFehler> {
        let hex = hex.into();
        let plausibel = hex.len() == 64 && hex.bytes().all(|byte| byte.is_ascii_hexdigit());
        if !plausibel {
            return Err(KontoFehler::TokenUngültig);
        }
        Ok(Self(hex.to_ascii_lowercase()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Constant-time comparison, so a lookup that compares hashes leaks no timing signal.
    ///
    /// Every byte is visited and the differences are accumulated; there is no early return
    /// and no branch on the value, which is what a plain `==` on `str` would introduce.
    pub fn constant_time_eq(&self, other: &Self) -> bool {
        constant_time_eq(self.0.as_bytes(), other.0.as_bytes())
    }
}

impl From<&SecretToken> for SecretTokenHash {
    fn from(token: &SecretToken) -> Self {
        // SHA-256 over the token text. Microseconds, unlike a Passwort.
        let digest = Sha256::digest(basis64url(&token.0[..]).as_bytes());
        Self(hex::encode(digest))
    }
}

/// Compare two byte slices without leaking where they differ.
///
/// Lengths of stored hashes are public (every one is 64 hex characters), so a length
/// mismatch may return early; the byte comparison itself accumulates every difference
/// instead of returning at the first one.
fn constant_time_eq(links: &[u8], rechts: &[u8]) -> bool {
    if links.len() != rechts.len() {
        return false;
    }
    let mut unterschied = 0u8;
    for (links, rechts) in links.iter().zip(rechts) {
        unterschied |= links ^ rechts;
    }
    unterschied == 0
}

/// A complete compact JWS: `header.payload.signature`, already base64url-encoded and
/// assembled by the API edge, which owns the JOSE envelope. Core treats it as opaque text
/// and never reads or writes its parts.
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
/// back, and stores nothing but its [`SecretTokenHash`].
#[derive(Debug, Clone)]
pub struct RefreshToken(SecretToken);

impl RefreshToken {
    pub fn random() -> Result<Self, KontoFehler> {
        Ok(Self(SecretToken::random()?))
    }

    pub fn base64url(&self) -> Zeroizing<String> {
        self.0.base64url()
    }

    pub fn hash(&self) -> SecretTokenHash {
        self.0.hash()
    }
}

impl TryFrom<&str> for RefreshToken {
    type Error = KontoFehler;

    fn try_from(wert: &str) -> Result<Self, Self::Error> {
        Ok(Self(SecretToken::try_from(wert)?))
    }
}

impl TryFrom<String> for RefreshToken {
    type Error = KontoFehler;

    fn try_from(wert: String) -> Result<Self, Self::Error> {
        Self::try_from(wert.as_str())
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

/// Decode 43 unpadded base64url characters into exactly 32 bytes.
fn basis64url_decode(text: &str, out: &mut [u8; SECRET_TOKEN_BYTES]) -> Result<(), KontoFehler> {
    let mut puffer: u32 = 0;
    let mut bits = 0u32;
    let mut index = 0usize;
    for byte in text.bytes() {
        let wert = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            _ => return Err(KontoFehler::TokenUngültig),
        };
        puffer = (puffer << 6) | u32::from(wert);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            if index >= out.len() {
                return Err(KontoFehler::TokenUngültig);
            }
            out[index] = ((puffer >> bits) & 0xff) as u8;
            index += 1;
        }
    }
    if index == out.len() {
        Ok(())
    } else {
        Err(KontoFehler::TokenUngültig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn secret_token_hat_256_bit_und_basis64url_form() {
        let token = SecretToken::random().unwrap();
        assert_eq!(token.base64url().len(), BASIS64URL_LÄNGE);
        assert!(SecretToken::try_from(token.base64url().as_str()).is_ok());
    }

    #[test_log::test]
    fn zwei_tokens_sind_verschieden() {
        let erster = SecretToken::random().unwrap();
        let zweiter = SecretToken::random().unwrap();
        assert_ne!(erster.base64url().as_str(), zweiter.base64url().as_str());
    }

    #[test_log::test]
    fn parsing_ist_ein_roundtrip_über_die_bytes() {
        let token = SecretToken::random().unwrap();
        let geladen = SecretToken::try_from(token.base64url().as_str()).unwrap();
        assert_eq!(
            token.base64url().as_str(),
            geladen.base64url().as_str(),
            "die Bytes überleben Kodierung und Dekodierung"
        );
        assert_eq!(token.hash(), geladen.hash());
    }

    #[test_log::test]
    fn token_hash_ist_sha256_hex() {
        let token = SecretToken::try_from("A".repeat(43)).unwrap();
        let hash = SecretTokenHash::from(&token);
        assert_eq!(hash.as_str().len(), 64);
        let erwartet = hex::encode(Sha256::digest("A".repeat(43).as_bytes()));
        assert_eq!(hash.as_str(), erwartet);
    }

    #[test_log::test]
    fn hash_ist_stabil_und_vergleicht_konstantzeitig() {
        let token = SecretToken::random().unwrap();
        let erster = token.hash();
        let zweiter = token.hash();
        assert!(erster.constant_time_eq(&zweiter));
        assert!(!erster.constant_time_eq(&SecretTokenHash::from_hex("0".repeat(64)).unwrap()));
    }

    #[test_log::test]
    fn konstantzeit_vergleich_erkennt_jedes_gekippte_byte() {
        let basis = SecretTokenHash::from_hex("a".repeat(64)).unwrap();
        for stelle in [0usize, 31, 63] {
            let mut bytes = basis.as_str().as_bytes().to_vec();
            bytes[stelle] = b'b';
            let anders = SecretTokenHash::from_hex(String::from_utf8(bytes).unwrap()).unwrap();
            assert!(!basis.constant_time_eq(&anders), "Byte {stelle}");
        }
        // Different lengths are unequal, and the comparison never panics.
        assert!(!constant_time_eq(b"abc", b"abcd"));
    }

    #[test_log::test]
    fn falsch_geformte_tokens_werden_abgelehnt() {
        assert!(SecretToken::try_from("zu-kurz").is_err());
        assert!(SecretToken::try_from("!".repeat(43).as_str()).is_err());
        assert!(SecretToken::try_from("A".repeat(44).as_str()).is_err());
        assert!(SecretTokenHash::from_hex("zz".repeat(32)).is_err());
    }

    #[test_log::test]
    fn debug_gibt_das_token_nicht_preis() {
        let token = SecretToken::random().unwrap();
        let text = format!("{token:?}");
        assert!(!text.contains(token.base64url().as_str()));
        assert!(text.contains("redacted"));
    }

    #[test_log::test]
    fn refresh_token_roundtrip() {
        let token = RefreshToken::random().unwrap();
        let geladen = RefreshToken::try_from(token.base64url().as_str()).unwrap();
        assert_eq!(token.base64url().as_str(), geladen.base64url().as_str());
        assert_eq!(token.hash(), geladen.hash());
    }
}
