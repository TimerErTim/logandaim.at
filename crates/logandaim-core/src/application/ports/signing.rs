use crate::application::ResultReport;
use crate::domain::konto::{Signature, SigningKeyId};

#[derive(Debug, thiserror::Error)]
pub enum SigningError {
    #[error("die Signatur ist ungültig")]
    InvalidSignature,
    #[error("die Signatur konnte nicht erzeugt werden")]
    SigningFailed,
}

/// Ed25519 signing as an application port.
///
/// The algorithm lives in the domain (`domain::konto::signing`); this trait only lets the
/// application compose a token and verify one without holding a keypair. The concrete key
/// arrives at startup through an adapter, so nothing above infrastructure knows whether it
/// came from a file, an environment variable, or a test fixture.
pub trait Signing: Send + Sync {
    /// Key identifier stamped into the token header. It names the key, not the session;
    /// a session row stores the matching public key hex so verification never consults a
    /// key registry.
    fn kid(&self) -> &SigningKeyId;

    /// The configured public key as lowercase hex, written onto every session row this
    /// keypair issues.
    fn public_key_hex(&self) -> String;

    fn sign(&self, message: &[u8]) -> Signature;

    fn verify(&self, message: &[u8], signature: &Signature) -> ResultReport<(), SigningError>;
}
