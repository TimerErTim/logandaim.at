use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{EMAIL_BESTAETIGUNG_GUELTIGKEIT, KontoFehler, TokenHash};
use crate::domain::EmailAdresse;

/// Identity of one E-Mail-Adresse-Bestätigung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BestaetigungId(pub Uuid);

/// Creation input of an E-Mail-Adresse-Bestätigung.
#[derive(Debug, Clone)]
pub struct NeueBestaetigung {
    email: EmailAdresse,
    token_hash: TokenHash,
    erstellt_am: DateTime<Utc>,
}

impl NeueBestaetigung {
    pub fn neu(email: EmailAdresse, token_hash: TokenHash, erstellt_am: DateTime<Utc>) -> Self {
        Self {
            email,
            token_hash,
            erstellt_am,
        }
    }

    pub fn email(&self) -> &EmailAdresse {
        &self.email
    }

    pub fn token_hash(&self) -> &TokenHash {
        &self.token_hash
    }

    pub fn erstellt_am(&self) -> DateTime<Utc> {
        self.erstellt_am
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.erstellt_am + EMAIL_BESTAETIGUNG_GUELTIGKEIT
    }
}

/// Proof that one address is reachable, mailed by registration step 1.
///
/// Keyed by the address, never by a `KontoId`: at step 1 no Konto exists yet (SPEC-konto
/// rule 3). Spending it is what authorises step 2, and step 2 still refuses if the address
/// gained a Konto in the meantime (rule 4, AC-20).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailBestaetigung {
    id: BestaetigungId,
    email: EmailAdresse,
    token_hash: TokenHash,
    expires_at: DateTime<Utc>,
    spent_at: Option<DateTime<Utc>>,
}

impl EmailBestaetigung {
    pub fn neu(id: BestaetigungId, neu: NeueBestaetigung) -> Self {
        let expires_at = neu.expires_at();
        Self {
            id,
            email: neu.email,
            token_hash: neu.token_hash,
            expires_at,
            spent_at: None,
        }
    }

    pub fn from_parts(
        id: BestaetigungId,
        email: EmailAdresse,
        token_hash: TokenHash,
        expires_at: DateTime<Utc>,
        spent_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            email,
            token_hash,
            expires_at,
            spent_at,
        }
    }

    pub fn id(&self) -> BestaetigungId {
        self.id
    }

    pub fn email(&self) -> &EmailAdresse {
        &self.email
    }

    pub fn token_hash(&self) -> &TokenHash {
        &self.token_hash
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn spent_at(&self) -> Option<DateTime<Utc>> {
        self.spent_at
    }

    pub fn ist_verbraucht(&self) -> bool {
        self.spent_at.is_some()
    }

    pub fn ist_abgelaufen(&self, jetzt: DateTime<Utc>) -> bool {
        self.expires_at <= jetzt
    }

    /// Usable exactly once and only inside its 24 hours (rule 13, AC-4).
    pub fn ist_gueltig(&self, jetzt: DateTime<Utc>) -> bool {
        !self.ist_verbraucht() && !self.ist_abgelaufen(jetzt)
    }

    pub fn verbrauchen(&mut self, jetzt: DateTime<Utc>) -> Result<(), KontoFehler> {
        if self.spent_at.is_some() {
            return Err(KontoFehler::TokenVerbraucht);
        }
        self.spent_at = Some(jetzt);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    fn jetzt() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn bestaetigung() -> EmailBestaetigung {
        EmailBestaetigung::neu(
            BestaetigungId(Uuid::nil()),
            NeueBestaetigung::neu(
                EmailAdresse::normalisiert("anna@muster.de").unwrap(),
                TokenHash::from_hex("a".repeat(64)).unwrap(),
                jetzt(),
            ),
        )
    }

    #[test_log::test]
    fn neue_bestaetigung_gilt_24_stunden() {
        let bestaetigung = bestaetigung();
        assert_eq!(bestaetigung.expires_at(), jetzt() + Duration::hours(24));
        assert!(bestaetigung.ist_gueltig(jetzt() + Duration::hours(23)));
        assert!(!bestaetigung.ist_gueltig(jetzt() + Duration::hours(24)));
    }

    #[test_log::test]
    fn zweite_verwendung_wird_abgelehnt() {
        let mut bestaetigung = bestaetigung();
        assert!(bestaetigung.verbrauchen(jetzt()).is_ok());
        assert!(matches!(
            bestaetigung.verbrauchen(jetzt()),
            Err(KontoFehler::TokenVerbraucht)
        ));
        assert!(!bestaetigung.ist_gueltig(jetzt()));
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let mut original = bestaetigung();
        original.verbrauchen(jetzt()).unwrap();
        let geladen = EmailBestaetigung::from_parts(
            original.id(),
            original.email().clone(),
            original.token_hash().clone(),
            original.expires_at(),
            original.spent_at(),
        );
        assert_eq!(original, geladen);
    }
}
