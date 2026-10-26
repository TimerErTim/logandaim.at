use chrono::{DateTime, Utc};

use super::KontoFehler;

/// Identity of one signing key row: the `kid` an access token names.
///
/// Ids come from configuration and are published in the token header, so the shape is
/// constrained to something that is safe in a JOSE header and readable in a log line.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SigningKeyId(String);

impl SigningKeyId {
    pub fn new(wert: impl Into<String>) -> Result<Self, KontoFehler> {
        let wert = wert.into();
        let plausibel = !wert.is_empty()
            && wert.len() <= 64
            && wert
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_');
        if !plausibel {
            return Err(KontoFehler::SigningKeyUngültig);
        }
        Ok(Self(wert))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One row of the key table: which public key may verify an access token, and whether
/// it may still do so.
///
/// The private key never lives here; it comes from deployment configuration. Retiring a
/// key is a row update, not a validity window (SPEC-konto rule 15, AC-17).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigningKey {
    kid: SigningKeyId,
    öffentlicher_schlüssel: String,
    aktiv: bool,
    erstellt_am: DateTime<Utc>,
    außer_betrieb_am: Option<DateTime<Utc>>,
}

impl SigningKey {
    pub fn neu(
        kid: SigningKeyId,
        öffentlicher_schlüssel: impl Into<String>,
        erstellt_am: DateTime<Utc>,
    ) -> Self {
        Self {
            kid,
            öffentlicher_schlüssel: öffentlicher_schlüssel.into(),
            aktiv: true,
            erstellt_am,
            außer_betrieb_am: None,
        }
    }

    pub fn from_parts(
        kid: SigningKeyId,
        öffentlicher_schlüssel: impl Into<String>,
        aktiv: bool,
        erstellt_am: DateTime<Utc>,
        außer_betrieb_am: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            kid,
            öffentlicher_schlüssel: öffentlicher_schlüssel.into(),
            aktiv,
            erstellt_am,
            außer_betrieb_am,
        }
    }

    pub fn kid(&self) -> &SigningKeyId {
        &self.kid
    }

    /// Hex-encoded Ed25519 public key, as the startup row stores it.
    pub fn öffentlicher_schlüssel(&self) -> &str {
        &self.öffentlicher_schlüssel
    }

    pub fn aktiv(&self) -> bool {
        self.aktiv
    }

    pub fn ist_aktiv(&self) -> bool {
        self.aktiv
    }

    pub fn erstellt_am(&self) -> DateTime<Utc> {
        self.erstellt_am
    }

    pub fn außer_betrieb_am(&self) -> Option<DateTime<Utc>> {
        self.außer_betrieb_am
    }

    /// Retire this key. Idempotent; an already retired row keeps its first timestamp so
    /// what happened when stays readable.
    pub fn außer_betrieb_nehmen(&mut self, jetzt: DateTime<Utc>) {
        self.aktiv = false;
        self.außer_betrieb_am.get_or_insert(jetzt);
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    fn jetzt() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    #[test_log::test]
    fn signing_key_id_wird_geprüft() {
        assert!(SigningKeyId::new("").is_err());
        assert!(SigningKeyId::new("mit leerzeichen").is_err());
        assert!(SigningKeyId::new("k-2026_10").is_ok());
    }

    #[test_log::test]
    fn neue_schlüssel_sind_aktiv() {
        let schlüssel = SigningKey::neu(SigningKeyId::new("k1").unwrap(), "abcd", jetzt());
        assert!(schlüssel.ist_aktiv());
        assert_eq!(schlüssel.außer_betrieb_am(), None);
    }

    #[test_log::test]
    fn außer_betrieb_nehmen_ist_idempotent() {
        let mut schlüssel = SigningKey::neu(SigningKeyId::new("k1").unwrap(), "abcd", jetzt());
        schlüssel.außer_betrieb_nehmen(jetzt());
        schlüssel.außer_betrieb_nehmen(jetzt() + Duration::hours(1));
        assert!(!schlüssel.ist_aktiv());
        assert_eq!(schlüssel.außer_betrieb_am(), Some(jetzt()));
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let original = SigningKey::neu(SigningKeyId::new("k1").unwrap(), "abcd", jetzt());
        let geladen = SigningKey::from_parts(
            original.kid().clone(),
            original.öffentlicher_schlüssel(),
            original.aktiv(),
            original.erstellt_am(),
            original.außer_betrieb_am(),
        );
        assert_eq!(original, geladen);
    }
}
