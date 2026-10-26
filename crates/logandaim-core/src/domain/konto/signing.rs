//! Ed25519 signing over raw bytes.
//!
//! The domain owns the algorithm, the key material, and the claim *semantics*. It does not
//! own the token's envelope: base64url, JOSE headers, and JSON are transport, they are only
//! ever used at the HTTP edge, and they live in `logandaim-api`. Core therefore signs a
//! byte string and verifies a byte string — the API hands it `"header.payload"` and takes
//! back a signature, or the reverse.
//!
//! Consequences that are deliberate, not incidental:
//!
//! - `logandaim-core` gains no `serde_json`, no `base64`, and no JOSE dependency.
//! - There is no `Signer` port. Swapping the algorithm invalidates every issued token, so
//!   it must be a domain refactor, not a wiring line.

use std::fmt;

use ed25519_dalek::pkcs8::DecodePrivateKey;
use ed25519_dalek::{Signer, Verifier};
use zeroize::Zeroizing;

use super::{KontoFehler, SigningKeyId};

/// Length of a raw Ed25519 signature.
pub const SIGNATURE_BYTES: usize = 64;

/// A raw 64-byte Ed25519 signature. The API base64url-encodes it; core never does.
#[derive(Clone, PartialEq, Eq)]
pub struct Signature([u8; SIGNATURE_BYTES]);

impl Signature {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KontoFehler> {
        let bytes: [u8; SIGNATURE_BYTES] = bytes
            .try_into()
            .map_err(|_| KontoFehler::SignaturUngueltig)?;
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; SIGNATURE_BYTES] {
        &self.0
    }

    pub fn to_bytes(&self) -> [u8; SIGNATURE_BYTES] {
        self.0
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Signature(")?;
        write!(f, "{}", hex::encode(self.0))?;
        f.write_str(")")
    }
}

/// The configured private key plus the row id it is published under.
///
/// Built once at startup from deployment configuration. The private half never reaches a
/// database, a log line, or an error: `Debug` prints the `kid` only, and the seed is
/// zeroized when the keypair is dropped.
#[derive(Clone)]
pub struct SigningKeypair {
    kid: SigningKeyId,
    schluessel: ed25519_dalek::SigningKey,
}

impl fmt::Debug for SigningKeypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SigningKeypair")
            .field("kid", &self.kid)
            .field("schluessel", &"[redacted]")
            .finish()
    }
}

impl SigningKeypair {
    /// Load a PKCS#8 v1 PEM (`-----BEGIN PRIVATE KEY-----`).
    ///
    /// A malformed key is a startup error; there is no generated fallback, because a
    /// silently created key would invalidate every session on the next restart.
    pub fn from_pkcs8_pem(kid: SigningKeyId, pem: &str) -> Result<Self, KontoFehler> {
        let pem = Zeroizing::new(pem.to_string());
        let schluessel = ed25519_dalek::SigningKey::from_pkcs8_pem(&pem)
            .map_err(|_| KontoFehler::SigningKeyUngueltig)?;
        Ok(Self { kid, schluessel })
    }

    /// Build from the 32 raw secret bytes; only tests and key generation use this.
    pub fn from_secret_bytes(kid: SigningKeyId, bytes: &[u8; 32]) -> Self {
        Self {
            kid,
            schluessel: ed25519_dalek::SigningKey::from_bytes(bytes),
        }
    }

    pub fn kid(&self) -> &SigningKeyId {
        &self.kid
    }

    /// Hex-encoded 32-byte public key, exactly what the `signing_keys` row stores.
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.schluessel.verifying_key().to_bytes())
    }

    /// Sign raw bytes. The caller assembles whatever envelope it needs around them.
    pub fn sign(&self, nachricht: &[u8]) -> Signature {
        Signature(self.schluessel.sign(nachricht).to_bytes())
    }

    /// Verify raw bytes against this keypair's public half.
    pub fn verify(&self, nachricht: &[u8], signature: &Signature) -> Result<(), KontoFehler> {
        let signature = ed25519_dalek::Signature::from_bytes(&signature.0);
        self.schluessel
            .verifying_key()
            .verify(nachricht, &signature)
            .map_err(|_| KontoFehler::SignaturUngueltig)
    }

    /// Verify with a public key that arrived from a key row, so a request signed by a
    /// different key is rejected even when that key is valid on its own.
    pub fn verify_with_public_key_hex(
        oeffentlicher_schluessel_hex: &str,
        nachricht: &[u8],
        signature: &Signature,
    ) -> Result<(), KontoFehler> {
        let bytes = hex::decode(oeffentlicher_schluessel_hex)
            .map_err(|_| KontoFehler::SigningKeyUngueltig)?;
        let bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|_| KontoFehler::SigningKeyUngueltig)?;
        let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(&bytes)
            .map_err(|_| KontoFehler::SigningKeyUngueltig)?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature.0);
        verifying_key
            .verify(nachricht, &signature)
            .map_err(|_| KontoFehler::SignaturUngueltig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keypair(seed: u8) -> SigningKeypair {
        SigningKeypair::from_secret_bytes(SigningKeyId::new("test-key").unwrap(), &[seed; 32])
    }

    #[test_log::test]
    fn signieren_und_verifizieren_roundtrip() {
        let keypair = keypair(1);
        let nachricht = b"eyJhbGciOiJFZERTQSJ9.eyJzdWIiOiIxIn0";
        let signatur = keypair.sign(nachricht);
        assert!(keypair.verify(nachricht, &signatur).is_ok());
    }

    #[test_log::test]
    fn jedes_byte_ist_signierbar() {
        let keypair = keypair(2);
        let nachricht: Vec<u8> = (0..=255u8).collect();
        let signatur = keypair.sign(&nachricht);
        assert!(keypair.verify(&nachricht, &signatur).is_ok());
    }

    #[test_log::test]
    fn gekipptes_byte_wird_abgelehnt() {
        let keypair = keypair(3);
        let signatur = keypair.sign(b"header.payload");
        assert!(matches!(
            keypair.verify(b"header.payloae", &signatur),
            Err(KontoFehler::SignaturUngueltig)
        ));
    }

    #[test_log::test]
    fn falscher_schluessel_wird_abgelehnt() {
        let signierer = keypair(4);
        let pruefer = keypair(5);
        let signatur = signierer.sign(b"header.payload");
        assert!(matches!(
            pruefer.verify(b"header.payload", &signatur),
            Err(KontoFehler::SignaturUngueltig)
        ));
    }

    #[test_log::test]
    fn public_key_verifikation_ueber_die_schluesselzeile() {
        let signierer = keypair(6);
        let signatur = signierer.sign(b"header.payload");
        let hex = signierer.public_key_hex();
        assert_eq!(hex.len(), 64);
        assert!(
            SigningKeypair::verify_with_public_key_hex(&hex, b"header.payload", &signatur).is_ok()
        );
        let fremder = keypair(7).public_key_hex();
        assert!(matches!(
            SigningKeypair::verify_with_public_key_hex(&fremder, b"header.payload", &signatur),
            Err(KontoFehler::SignaturUngueltig)
        ));
    }

    #[test_log::test]
    fn kaputte_signaturlaenge_wird_abgelehnt() {
        assert!(matches!(
            Signature::from_bytes(&[0u8; 63]),
            Err(KontoFehler::SignaturUngueltig)
        ));
    }

    #[test_log::test]
    fn debug_gibt_den_privatschluessel_nicht_preis() {
        let keypair = keypair(8);
        let text = format!("{keypair:?}");
        assert!(text.contains("redacted"));
        assert!(text.contains("test-key"));
    }

    #[test_log::test]
    fn kaputtes_pem_wird_abgelehnt() {
        let ergebnis = SigningKeypair::from_pkcs8_pem(
            SigningKeyId::new("k1").unwrap(),
            "-----BEGIN PRIVATE KEY-----\nnope\n-----END PRIVATE KEY-----",
        );
        assert!(matches!(ergebnis, Err(KontoFehler::SigningKeyUngueltig)));
    }
}
