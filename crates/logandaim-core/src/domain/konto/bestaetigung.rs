use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{EMAIL_BESTÄTIGUNG_GÜLTIGKEIT, SecretTokenHash};
use crate::domain::EmailAdresse;

/// Identity of one E-Mail-Adresse-Bestätigung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BestätigungId(pub Uuid);

/// Creation input of an E-Mail-Adresse-Bestätigung.
#[derive(Debug, Clone)]
pub struct NeueBestätigung {
    email: EmailAdresse,
    token_hash: SecretTokenHash,
    created_at: DateTime<Utc>,
}

impl NeueBestätigung {
    pub fn neu(
        email: EmailAdresse,
        token_hash: SecretTokenHash,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            email,
            token_hash,
            created_at,
        }
    }

    pub fn email(&self) -> &EmailAdresse {
        &self.email
    }

    pub fn token_hash(&self) -> &SecretTokenHash {
        &self.token_hash
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.created_at + EMAIL_BESTÄTIGUNG_GÜLTIGKEIT
    }
}

/// Proof that one address is reachable, mailed by registration step 1.
///
/// Keyed by the address, never by a `KontoId`: at step 1 no Konto exists yet (SPEC-konto
/// rule 3). Spending it is what authorises step 2, and step 2 still refuses if the address
/// gained a Konto in the meantime (rule 4, AC-20).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailBestätigung {
    id: BestätigungId,
    email: EmailAdresse,
    token_hash: SecretTokenHash,
    expires_at: DateTime<Utc>,
    spent_at: Option<DateTime<Utc>>,
}

impl EmailBestätigung {
    pub fn neu(id: BestätigungId, neu: NeueBestätigung) -> Self {
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
        id: BestätigungId,
        email: EmailAdresse,
        token_hash: SecretTokenHash,
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

    pub fn id(&self) -> BestätigungId {
        self.id
    }

    pub fn email(&self) -> &EmailAdresse {
        &self.email
    }

    pub fn token_hash(&self) -> &SecretTokenHash {
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
    pub fn ist_gültig(&self, jetzt: DateTime<Utc>) -> bool {
        !self.ist_verbraucht() && !self.ist_abgelaufen(jetzt)
    }

    /// Spend this row. Idempotent, so a resend loop that spends every open row of an
    /// address does not fail on one that was already spent.
    pub fn verbrauchen(&mut self, jetzt: DateTime<Utc>) {
        self.spent_at.get_or_insert(jetzt);
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    fn jetzt() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn bestätigung() -> EmailBestätigung {
        EmailBestätigung::neu(
            BestätigungId(Uuid::nil()),
            NeueBestätigung::neu(
                EmailAdresse::normalisiert("anna@muster.de").unwrap(),
                SecretTokenHash::from_hex("a".repeat(64)).unwrap(),
                jetzt(),
            ),
        )
    }

    #[test_log::test]
    fn neue_bestätigung_gilt_24_stunden() {
        let bestätigung = bestätigung();
        assert_eq!(bestätigung.expires_at(), jetzt() + Duration::hours(24));
        assert!(bestätigung.ist_gültig(jetzt() + Duration::hours(23)));
        assert!(!bestätigung.ist_gültig(jetzt() + Duration::hours(24)));
    }

    #[test_log::test]
    fn zweite_verwendung_wird_abgelehnt() {
        let mut bestätigung = bestätigung();
        bestätigung.verbrauchen(jetzt());
        // Spending twice keeps the first timestamp instead of failing.
        bestätigung.verbrauchen(jetzt() + Duration::minutes(1));
        assert_eq!(bestätigung.spent_at(), Some(jetzt()));
        assert!(bestätigung.ist_verbraucht());
        assert!(!bestätigung.ist_gültig(jetzt()));
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let mut original = bestätigung();
        original.verbrauchen(jetzt());
        let geladen = EmailBestätigung::from_parts(
            original.id(),
            original.email().clone(),
            original.token_hash().clone(),
            original.expires_at(),
            original.spent_at(),
        );
        assert_eq!(original, geladen);
    }
}
