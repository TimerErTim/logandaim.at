//! Ed25519 key material as an application port.
//!
//! Thin by design: the algorithm is domain code, and this adapter only carries key material
//! from startup configuration to the places that sign and verify tokens.

use crate::ResultReport;
use crate::domain::konto::{Signature, SigningKeypair};
use crate::ports::{Signing, SigningError};

pub struct InMemorySigning {
    keypair: SigningKeypair,
}

impl InMemorySigning {
    pub fn new(keypair: SigningKeypair) -> Self {
        Self { keypair }
    }

    /// Wrap a 32-byte seed; tests and local development use this.
    pub fn from_secret_bytes(bytes: &[u8; 32]) -> Self {
        Self::new(SigningKeypair::from_secret_bytes(bytes))
    }

    /// A fresh keypair from the OS CSPRNG, so a throwaway deployment needs no key file.
    pub fn random() -> Self {
        let mut seed = [0u8; 32];
        loop {
            match getrandom::fill(&mut seed) {
                Ok(()) => break Self::from_secret_bytes(&seed),
                Err(error) => tracing::warn!(%error, "OS randomness unavailable; retrying"),
            }
        }
    }

    pub fn keypair(&self) -> &SigningKeypair {
        &self.keypair
    }
}

impl Default for InMemorySigning {
    fn default() -> Self {
        Self::random()
    }
}

impl Signing for InMemorySigning {
    fn public_key_hex(&self) -> String {
        self.keypair.public_key_hex()
    }

    fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }

    fn verify(&self, message: &[u8], signature: &Signature) -> ResultReport<(), SigningError> {
        SigningKeypair::verify_with_public_key_hex(&self.public_key_hex(), message, signature)
            .map_err(|_| error_stack::Report::new(SigningError::InvalidSignature))
    }
}

/// Verify against a public key loaded from a stored row, not the configured keypair: that is
/// what makes AC-17's "stored public key / signature mismatch -> 401" checkable.
pub fn verify_with_public_key_hex(
    public_key_hex: &str,
    message: &[u8],
    signature: &Signature,
) -> ResultReport<(), SigningError> {
    SigningKeypair::verify_with_public_key_hex(public_key_hex, message, signature)
        .map_err(|_| error_stack::Report::new(SigningError::InvalidSignature))
}
