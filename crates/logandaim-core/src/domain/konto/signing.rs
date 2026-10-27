//! Ed25519 signing over raw bytes.
//!
//! The domain owns the algorithm and the key material; the JWT envelope is transport and
//! lives in `logandaim-api`. Core therefore signs and verifies byte strings, and gains no
//! serialization dependency.

use std::fmt;

use ed25519_dalek::pkcs8::DecodePrivateKey;
use ed25519_dalek::{Signer, Verifier};
use zeroize::Zeroizing;

use super::KontoFehler;

/// Length of a raw Ed25519 signature.
pub const SIGNATURE_BYTES: usize = 64;

/// A raw 64-byte Ed25519 signature. The API base64url-encodes it; core never does.
#[derive(Clone, PartialEq, Eq)]
pub struct Signature([u8; SIGNATURE_BYTES]);

impl Signature {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, KontoFehler> {
        let bytes: [u8; SIGNATURE_BYTES] = bytes
            .try_into()
            .map_err(|_| KontoFehler::SignaturUngültig)?;
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

/// The configured private key. `Debug` never prints it, and the seed is zeroized on drop.
#[derive(Clone)]
pub struct SigningKeypair {
    schlüssel: ed25519_dalek::SigningKey,
}

impl fmt::Debug for SigningKeypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SigningKeypair")
            .field("schlüssel", &"[redacted]")
            .finish()
    }
}

impl SigningKeypair {
    /// Load a PKCS#8 v1 PEM (`-----BEGIN PRIVATE KEY-----`).
    pub fn from_pkcs8_pem(pem: &str) -> Result<Self, KontoFehler> {
        let pem = Zeroizing::new(pem.to_string());
        let schlüssel = ed25519_dalek::SigningKey::from_pkcs8_pem(&pem)
            .map_err(|_| KontoFehler::SigningKeyUngültig)?;
        Ok(Self { schlüssel })
    }

    /// Build from the 32 raw secret bytes.
    pub fn from_secret_bytes(bytes: &[u8; 32]) -> Self {
        Self {
            schlüssel: ed25519_dalek::SigningKey::from_bytes(bytes),
        }
    }

    /// Hex-encoded 32-byte public key. Every session row stores this, so an access token is
    /// verified against the key its own session was issued under.
    pub fn public_key_hex(&self) -> String {
        hex::encode(self.schlüssel.verifying_key().to_bytes())
    }

    pub fn sign(&self, nachricht: &[u8]) -> Signature {
        Signature(self.schlüssel.sign(nachricht).to_bytes())
    }

    /// Verify with the public key stored on the session row, not the configured keypair.
    pub fn verify_with_public_key_hex(
        öffentlicher_schlüssel_hex: &str,
        nachricht: &[u8],
        signature: &Signature,
    ) -> Result<(), KontoFehler> {
        let bytes = hex::decode(öffentlicher_schlüssel_hex)
            .map_err(|_| KontoFehler::SigningKeyUngültig)?;
        let bytes: [u8; 32] = bytes
            .try_into()
            .map_err(|_| KontoFehler::SigningKeyUngültig)?;
        let verifying_key = ed25519_dalek::VerifyingKey::from_bytes(&bytes)
            .map_err(|_| KontoFehler::SigningKeyUngültig)?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature.0);
        verifying_key
            .verify(nachricht, &signature)
            .map_err(|_| KontoFehler::SignaturUngültig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn keypair(seed: u8) -> SigningKeypair {
        SigningKeypair::from_secret_bytes(&[seed; 32])
    }

    #[test_log::test]
    fn signieren_und_verifizieren_roundtrip() {
        let keypair = keypair(1);
        let nachricht = b"eyJhbGciOiJFZERTQSJ9.eyJzdWIiOiIxIn0";
        let signatur = keypair.sign(nachricht);
        let hex = keypair.public_key_hex();
        assert!(SigningKeypair::verify_with_public_key_hex(&hex, nachricht, &signatur).is_ok());
    }

    #[test_log::test]
    fn jedes_byte_ist_signierbar() {
        let keypair = keypair(2);
        let nachricht: Vec<u8> = (0..=255u8).collect();
        let signatur = keypair.sign(&nachricht);
        let hex = keypair.public_key_hex();
        assert!(SigningKeypair::verify_with_public_key_hex(&hex, &nachricht, &signatur).is_ok());
    }

    #[test_log::test]
    fn gekipptes_byte_wird_abgelehnt() {
        let keypair = keypair(3);
        let signatur = keypair.sign(b"header.payload");
        assert!(matches!(
            SigningKeypair::verify_with_public_key_hex(
                &keypair.public_key_hex(),
                b"header.payloadX",
                &signatur
            ),
            Err(KontoFehler::SignaturUngültig)
        ));
    }

    #[test_log::test]
    fn falscher_schlüssel_wird_abgelehnt() {
        let signierer = keypair(4);
        let signatur = signierer.sign(b"header.payload");
        assert!(matches!(
            SigningKeypair::verify_with_public_key_hex(
                &keypair(5).public_key_hex(),
                b"header.payload",
                &signatur
            ),
            Err(KontoFehler::SignaturUngültig)
        ));
    }

    #[test_log::test]
    fn public_key_ist_hex_und_32_bytes_lang() {
        assert_eq!(keypair(6).public_key_hex().len(), 64);
    }

    #[test_log::test]
    fn kaputte_signaturlänge_wird_abgelehnt() {
        assert!(matches!(
            Signature::from_bytes(&[0u8; 63]),
            Err(KontoFehler::SignaturUngültig)
        ));
    }

    #[test_log::test]
    fn debug_gibt_den_privatschlüssel_nicht_preis() {
        assert!(format!("{:?}", keypair(8)).contains("redacted"));
    }

    #[test_log::test]
    fn kaputtes_pem_wird_abgelehnt() {
        let ergebnis = SigningKeypair::from_pkcs8_pem(
            "-----BEGIN PRIVATE KEY-----\nnope\n-----END PRIVATE KEY-----",
        );
        assert!(matches!(ergebnis, Err(KontoFehler::SigningKeyUngültig)));
    }
}
