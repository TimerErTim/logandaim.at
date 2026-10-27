//! The `konto` supporting subdomain: Benutzer, Konto, Anmelde-Sitzungen, and their crypto.
//!
//! This is a supporting subdomain rather than a generic mechanism. It owns its interfaces,
//! its rules, and the algorithms behind its credentials, and its use cases decide access
//! themselves (SPEC-konto rule 17).
//!
//! ## Crypto lives here, deliberately and concretely
//!
//! Argon2id parameterisation, the token-hash choice, the signing algorithm, and claim
//! validation are properties of this subdomain's contract. There is therefore **no**
//! `PasswordHasher` port and **no** `Signer` port: a different algorithm invalidates every
//! stored hash and every issued token, so it must surface as a refactor across domain,
//! storage, and configuration — never as a line of wiring. Only *key material* and the
//! *SMTP host* come from deployment configuration, because those are data, not behaviour.
//!
//! ## Vocabulary
//!
//! Product nouns stay German (`Benutzer`, `Konto`, `EmailBestätigung`,
//! `PasswortWiederherstellung`). Auth-technical terms are English, as German engineering
//! usage has them: `Session`, `AccessToken`, `RefreshToken`, `SecretToken`,
//! `SecretTokenHash`, `SigningKey`, `PasswordHash`, `Signature`. Timestamps are English too
//! (`created_at`, `expires_at`, `spent_at`, `revoked_at`), matching the column names.
//!
//! ## The API owns the JWT envelope
//!
//! Core signs and verifies raw bytes ([`SigningKeypair`]) and validates claim *semantics*
//! ([`AccessTokenClaims`]). It never writes or reads JSON, base64url, or a JOSE header:
//! all of that is transport, it is only used at the HTTP edge, and it lives in
//! `logandaim-api`. `logandaim-core` therefore depends on no serialization crate.
//!
//! ## Session rows are retained
//!
//! A session row is never deleted when it is spent, revoked, or expired: replay detection
//! depends on still finding the row a spent refresh token names. See [`SessionIn`].

use chrono::Duration;

mod benutzer;
mod bestaetigung;
mod claims;
mod konto;
mod passwort;
mod passwort_wiederherstellung;
mod session;
mod session_ausgestellt;
mod signing;
mod token;

pub use benutzer::{Benutzer, BenutzerId, NeuerBenutzer, Rolle};
pub use bestaetigung::{BestätigungId, EmailBestätigung, NeueBestätigung};
pub use claims::AccessTokenClaims;
pub use konto::{Konto, KontoId, NeuesKonto};
pub use passwort::{KlartextPasswort, PasswordHash, dummy_verify, hash_password, verify_password};
pub use passwort_wiederherstellung::{
    NeuePasswortWiederherstellung, PasswortWiederherstellung, PasswortWiederherstellungId,
};
pub use session::{
    NeueSession, Session, SessionId, SessionIn, SessionNotLive, SessionState, state,
};
pub use session_ausgestellt::{SessionAusgestellt, claims_for};
pub use signing::{SIGNATURE_BYTES, Signature, SigningKeyId, SigningKeypair};
pub use token::{AccessToken, RefreshToken, SecretToken, SecretTokenHash};

/// Shortest accepted Passwort, in characters.
pub const MIN_PASSWORT_LÄNGE: usize = 8;
/// Longest accepted Passwort, in characters. Caps the argon2 input.
pub const MAX_PASSWORT_LÄNGE: usize = 128;

/// Lifetime of an E-Mail-Adresse-Bestätigung.
pub const EMAIL_BESTÄTIGUNG_GÜLTIGKEIT: Duration = Duration::hours(24);
/// Lifetime of a Passwort-Wiederherstellung token.
pub const PASSWORT_WIEDERHERSTELLUNG_GÜLTIGKEIT: Duration = Duration::minutes(60);
/// Lifetime of one access token.
pub const ACCESS_TOKEN_LIFETIME: Duration = Duration::hours(24);
/// Lifetime of one refresh token.
pub const REFRESH_TOKEN_LIFETIME: Duration = Duration::days(30);
/// Tolerated clock difference between issuer and verifier.
pub const CLOCK_SKEW: Duration = Duration::seconds(60);

/// A rule of the `konto` subdomain was violated.
///
/// Messages are German UI language and never carry a secret: no Passwort, no token, no
/// hash, no key material.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum KontoFehler {
    #[error(
        "das Passwort muss zwischen {MIN_PASSWORT_LÄNGE} und {MAX_PASSWORT_LÄNGE} Zeichen lang sein, war aber {länge}"
    )]
    PasswortLängeUngültig { länge: usize },
    #[error("das Passwort konnte nicht geprüft werden")]
    PasswortHashFehler,
    #[error("der gespeicherte Passwort-Hash ist ungültig")]
    PasswortHashUngültig,
    #[error("das Token ist ungültig")]
    TokenUngültig,
    #[error("das Token wurde bereits verwendet")]
    TokenVerbraucht,
    #[error("das Token ist abgelaufen")]
    TokenAbgelaufen,
    #[error("das Token ist noch nicht gültig")]
    TokenNochNichtGültig,
    #[error("das Token stammt nicht von diesem Dienst")]
    TokenHerkunftUngültig,
    #[error("die Signatur ist ungültig")]
    SignaturUngültig,
    #[error("der Signaturschlüssel ist ungültig")]
    SigningKeyUngültig,
    #[error("die Rolle ist unbekannt")]
    RolleUnbekannt,
    #[error("es konnte kein Zufall erzeugt werden")]
    ZufallFehlgeschlagen,
}
