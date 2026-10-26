use crate::ResultReport;
use crate::domain::konto::{Signature, SigningKeyId, SigningKeypair};
use crate::ports::{Signing, SigningError};

/// The configured Ed25519 keypair as an application port.
///
/// Thin by design: the algorithm is domain code, and this adapter only carries key
/// material from startup configuration to the places that compose and verify tokens.
pub struct InMemorySigning {
    keypair: SigningKeypair,
}

impl InMemorySigning {
    pub fn new(keypair: SigningKeypair) -> Self {
        Self { keypair }
    }

    /// Wrap a 32-byte seed. Tests and local development use this; production loads a
    /// PKCS#8 PEM instead.
    pub fn from_secret_bytes(kid: SigningKeyId, bytes: &[u8; 32]) -> Self {
        Self::new(SigningKeypair::from_secret_bytes(kid, bytes))
    }

    /// A fresh keypair from the OS CSPRNG.
    ///
    /// For a throwaway deployment: nothing has to be generated beforehand, and a restart
    /// simply invalidates the sessions it issued. A long-lived deployment configures a
    /// PEM instead, because a key that changes on restart is a key no issued token can
    /// outlive.
    pub fn random(kid: SigningKeyId) -> Self {
        let mut seed = [0u8; 32];
        loop {
            match getrandom::fill(&mut seed) {
                Ok(()) => break Self::from_secret_bytes(kid, &seed),
                Err(error) => {
                    tracing::warn!(%error, "OS randomness unavailable; retrying");
                }
            }
        }
    }

    pub fn keypair(&self) -> &SigningKeypair {
        &self.keypair
    }
}

impl Signing for InMemorySigning {
    fn kid(&self) -> &SigningKeyId {
        self.keypair.kid()
    }

    fn public_key_hex(&self) -> String {
        self.keypair.public_key_hex()
    }

    fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }

    fn verify(&self, message: &[u8], signature: &Signature) -> ResultReport<(), SigningError> {
        self.keypair
            .verify(message, signature)
            .map_err(|_| error_stack::Report::new(SigningError::InvalidSignature))
    }
}

/// Verify a signature against a public key loaded from a stored row, not the configured
/// keypair: that is what makes AC-17's "stored public key / signature mismatch -> 401"
/// checkable without a key registry.
pub fn verify_with_public_key_hex(
    public_key_hex: &str,
    message: &[u8],
    signature: &Signature,
) -> ResultReport<(), SigningError> {
    SigningKeypair::verify_with_public_key_hex(public_key_hex, message, signature)
        .map_err(|_| error_stack::Report::new(SigningError::InvalidSignature))
}
