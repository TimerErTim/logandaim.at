//! Passwort hashing — deliberately concrete domain code.
//!
//! There is no `PasswordHasher` port. The algorithm, its parameters, and the stored
//! format are part of this subdomain's contract: changing them invalidates every stored
//! hash and must surface as a refactor of domain, storage, and configuration, never as a
//! wiring line. See `mod.rs`.

use std::fmt;
use std::str::FromStr;

use argon2::password_hash::phc::PasswordHash as PhcPasswordHash;
use argon2::password_hash::phc::Salt;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, PasswordVerifier, Version};
use zeroize::Zeroizing;

use super::KontoFehler;

/// OWASP minimum for argon2id: m=19 MiB, t=2, p=1, 32-byte output.
///
/// Not negotiable per environment or per test. A `#[cfg(test)]` variant would let a
/// wrong production parameter ship silently, which is the one thing this module forbids.
const ARGON2_M_COST_KIB: u32 = 19 * 1024;
const ARGON2_T_COST: u32 = 2;
const ARGON2_P_COST: u32 = 1;
const ARGON2_OUTPUT_LEN: usize = 32;
/// Salt length, from the PHC recommendation.
const SALT_BYTES: usize = 16;

/// Secret of one Konto, in clear text, only while it is being set or checked.
///
/// Length is counted in characters (`8..=128`), not bytes: a German umlaut or an emoji
/// is one character to the person typing it. `Debug` never reveals the value, and the
/// buffer is zeroized on drop.
pub struct KlartextPasswort(Zeroizing<String>);

impl KlartextPasswort {
    pub fn neu(passwort: impl Into<String>) -> Result<Self, KontoFehler> {
        let passwort = passwort.into();
        let laenge = passwort.chars().count();
        if laenge < super::MIN_PASSWORT_LAENGE || laenge > super::MAX_PASSWORT_LAENGE {
            return Err(KontoFehler::PasswortLaengeUngueltig { laenge });
        }
        Ok(Self(Zeroizing::new(passwort)))
    }

    fn als_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Debug for KlartextPasswort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("KlartextPasswort([redacted])")
    }
}

impl TryFrom<&str> for KlartextPasswort {
    type Error = KontoFehler;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::neu(value)
    }
}

impl TryFrom<String> for KlartextPasswort {
    type Error = KontoFehler;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::neu(value)
    }
}

/// A stored credential: the argon2id PHC string, never the Passwort.
#[derive(Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    /// Rebuild a stored hash, rejecting anything this module did not write.
    pub fn from_phc(phc: impl Into<String>) -> Result<Self, KontoFehler> {
        let phc = phc.into();
        let parsed =
            PhcPasswordHash::from_str(&phc).map_err(|_| KontoFehler::PasswortHashUngueltig)?;
        if parsed.algorithm.as_str() != ALGORITHM_IDENT {
            return Err(KontoFehler::PasswortHashUngueltig);
        }
        Ok(Self(phc))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for PasswordHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The PHC string carries the salt and the digest; neither is a secret, but it is
        // still a credential and it never belongs in a log line by accident.
        f.write_str("PasswordHash([redacted])")
    }
}

/// PHC identifier this module writes and accepts.
const ALGORITHM_IDENT: &str = "argon2id";

fn argon2() -> Argon2<'static> {
    let params = Params::new(
        ARGON2_M_COST_KIB,
        ARGON2_T_COST,
        ARGON2_P_COST,
        Some(ARGON2_OUTPUT_LEN),
    )
    .expect("argon2 parameters are compile-time constants and valid");
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

/// Hash a Passwort with a fresh random salt.
///
/// Costs tens of milliseconds by design. Called only where a Passwort is written:
/// `RegistrierungAbschliessen` and `PasswortWiederherstellen`.
pub fn hash_password(passwort: &KlartextPasswort) -> Result<PasswordHash, KontoFehler> {
    let mut salt_bytes = [0u8; SALT_BYTES];
    getrandom::fill(&mut salt_bytes).map_err(|_| KontoFehler::ZufallFehlgeschlagen)?;
    let salt = Salt::new(&salt_bytes).map_err(|_| KontoFehler::PasswortHashFehler)?;
    let phc = argon2()
        .hash_password_with_salt(passwort.als_bytes(), &salt)
        .map_err(|_| KontoFehler::PasswortHashFehler)?;
    Ok(PasswordHash(phc.to_string()))
}

/// Check a Passwort against a stored hash.
///
/// The parameters come from the stored PHC string, so a hash written years ago keeps
/// verifying even after the module's defaults move.
pub fn verify_password(
    passwort: &KlartextPasswort,
    hash: &PasswordHash,
) -> Result<bool, KontoFehler> {
    let parsed =
        PhcPasswordHash::from_str(hash.as_str()).map_err(|_| KontoFehler::PasswortHashUngueltig)?;
    match argon2().verify_password(passwort.als_bytes(), &parsed) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false),
        Err(_) => Err(KontoFehler::PasswortHashFehler),
    }
}

/// A fixed hash checked when no Konto matches the address.
///
/// Keeps the unknown-address path of `Anmeldung` as expensive as the wrong-Passwort
/// path, so the response time does not disclose account existence (SPEC-konto rule 4).
pub fn dummy_verify(passwort: &KlartextPasswort) -> Result<(), KontoFehler> {
    static DUMMY_HASH: std::sync::OnceLock<PasswordHash> = std::sync::OnceLock::new();
    let hash = DUMMY_HASH.get_or_init(|| {
        // A hash of a value nobody can type, computed once per process.
        let unmoeglich = KlartextPasswort::neu("\u{0}logandaim-dummy-verify\u{0}")
            .expect("the dummy Passwort satisfies the length rule");
        hash_password(&unmoeglich).expect("argon2 hashes the dummy Passwort")
    });
    let _ = verify_password(passwort, hash)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn phc_string_traegt_die_owasp_parameter() {
        let passwort = KlartextPasswort::neu("korrekt-pferd-batterie").unwrap();
        let hash = hash_password(&passwort).unwrap();
        assert!(
            hash.as_str().starts_with("$argon2id$v=19$m=19456,t=2,p=1$"),
            "unerwartete PHC-Parameter: {}",
            hash.as_str()
        );
        assert_eq!(hash.as_str().matches('$').count(), 5);
    }

    #[test_log::test]
    fn hash_und_verify_stimmen_ueberein() {
        let passwort = KlartextPasswort::neu("korrekt-pferd-batterie").unwrap();
        let hash = hash_password(&passwort).unwrap();
        assert!(verify_password(&passwort, &hash).unwrap());
    }

    #[test_log::test]
    fn falsches_passwort_wird_abgelehnt() {
        let passwort = KlartextPasswort::neu("korrekt-pferd-batterie").unwrap();
        let hash = hash_password(&passwort).unwrap();
        let falsch = KlartextPasswort::neu("korrekt-pferd-batterie!").unwrap();
        assert!(!verify_password(&falsch, &hash).unwrap());
    }

    #[test_log::test]
    fn gleiches_passwort_ergibt_verschiedene_hashes() {
        let passwort = KlartextPasswort::neu("korrekt-pferd-batterie").unwrap();
        let erster = hash_password(&passwort).unwrap();
        let zweiter = hash_password(&passwort).unwrap();
        assert_ne!(erster, zweiter, "salt must be fresh per hash");
    }

    #[test_log::test]
    fn laenge_zaehlt_zeichen_nicht_bytes() {
        // Eight umlauts are eight characters but sixteen bytes.
        assert!(KlartextPasswort::neu("ääääääää").is_ok());
        assert!(matches!(
            KlartextPasswort::neu("äääääää"),
            Err(KontoFehler::PasswortLaengeUngueltig { laenge: 7 })
        ));
    }

    #[test_log::test]
    fn laengengrenzen_werden_geprueft() {
        assert!(KlartextPasswort::neu("a".repeat(8)).is_ok());
        assert!(KlartextPasswort::neu("a".repeat(128)).is_ok());
        assert!(KlartextPasswort::neu("a".repeat(129)).is_err());
        assert!(KlartextPasswort::neu("a".repeat(7)).is_err());
    }

    #[test_log::test]
    fn debug_gibt_das_passwort_nicht_preis() {
        let passwort = KlartextPasswort::neu("korrekt-pferd-batterie").unwrap();
        let text = format!("{passwort:?}");
        assert!(!text.contains("korrekt-pferd-batterie"));
        assert!(text.contains("redacted"));
    }

    #[test_log::test]
    fn fremder_algorithmus_wird_abgelehnt() {
        assert!(matches!(
            PasswordHash::from_phc("$scrypt$v=1$m=1,t=1,p=1$c2FsdA$aGFzaA"),
            Err(KontoFehler::PasswortHashUngueltig)
        ));
        assert!(matches!(
            PasswordHash::from_phc("kein phc string"),
            Err(KontoFehler::PasswortHashUngueltig)
        ));
    }

    #[test_log::test]
    fn dummy_verify_akzeptiert_jedes_gueltige_passwort() {
        let passwort = KlartextPasswort::neu("irgendwas-langes").unwrap();
        assert!(dummy_verify(&passwort).is_ok());
    }
}
