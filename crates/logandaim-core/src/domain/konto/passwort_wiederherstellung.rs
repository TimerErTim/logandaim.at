use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{KontoId, PASSWORT_WIEDERHERSTELLUNG_GÜLTIGKEIT, SecretTokenHash};

/// Identity of one Passwort-Wiederherstellung token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PasswortWiederherstellungId(pub Uuid);

/// Creation input of a Passwort-Wiederherstellung.
#[derive(Debug, Clone)]
pub struct NeuePasswortWiederherstellung {
    konto_id: KontoId,
    token_hash: SecretTokenHash,
    created_at: DateTime<Utc>,
}

impl NeuePasswortWiederherstellung {
    pub fn neu(konto_id: KontoId, token_hash: SecretTokenHash, created_at: DateTime<Utc>) -> Self {
        Self {
            konto_id,
            token_hash,
            created_at,
        }
    }

    pub fn konto_id(&self) -> KontoId {
        self.konto_id
    }

    pub fn token_hash(&self) -> &SecretTokenHash {
        &self.token_hash
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.created_at + PASSWORT_WIEDERHERSTELLUNG_GÜLTIGKEIT
    }
}

/// Authorisation to set a new Passwort on one Konto, mailed by `PasswortVergessen`.
///
/// Single use, bound to one Konto, valid 60 minutes; requesting a new one spends the
/// previous (SPEC-konto rule 7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswortWiederherstellung {
    id: PasswortWiederherstellungId,
    konto_id: KontoId,
    token_hash: SecretTokenHash,
    expires_at: DateTime<Utc>,
    spent_at: Option<DateTime<Utc>>,
}

impl PasswortWiederherstellung {
    pub fn neu(id: PasswortWiederherstellungId, neu: NeuePasswortWiederherstellung) -> Self {
        let expires_at = neu.expires_at();
        Self {
            id,
            konto_id: neu.konto_id,
            token_hash: neu.token_hash,
            expires_at,
            spent_at: None,
        }
    }

    pub fn from_parts(
        id: PasswortWiederherstellungId,
        konto_id: KontoId,
        token_hash: SecretTokenHash,
        expires_at: DateTime<Utc>,
        spent_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            konto_id,
            token_hash,
            expires_at,
            spent_at,
        }
    }

    pub fn id(&self) -> PasswortWiederherstellungId {
        self.id
    }

    pub fn konto_id(&self) -> KontoId {
        self.konto_id
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

    pub fn ist_gültig(&self, jetzt: DateTime<Utc>) -> bool {
        !self.ist_verbraucht() && !self.ist_abgelaufen(jetzt)
    }

    /// Spend this row. Idempotent, so spending every open row of a Konto keeps the first
    /// timestamp of each instead of failing on one that was already spent.
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

    fn wiederherstellung() -> PasswortWiederherstellung {
        PasswortWiederherstellung::neu(
            PasswortWiederherstellungId(Uuid::nil()),
            NeuePasswortWiederherstellung::neu(
                KontoId(Uuid::nil()),
                SecretTokenHash::from_hex("b".repeat(64)).unwrap(),
                jetzt(),
            ),
        )
    }

    #[test_log::test]
    fn gilt_60_minuten() {
        let token = wiederherstellung();
        assert_eq!(token.expires_at(), jetzt() + Duration::minutes(60));
        assert!(token.ist_gültig(jetzt() + Duration::minutes(59)));
        assert!(!token.ist_gültig(jetzt() + Duration::minutes(60)));
    }

    #[test_log::test]
    fn zweite_verwendung_wird_abgelehnt() {
        let mut token = wiederherstellung();
        token.verbrauchen(jetzt());
        token.verbrauchen(jetzt() + Duration::minutes(1));
        assert_eq!(token.spent_at(), Some(jetzt()));
        assert!(!token.ist_gültig(jetzt()));
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let original = wiederherstellung();
        let geladen = PasswortWiederherstellung::from_parts(
            original.id(),
            original.konto_id(),
            original.token_hash().clone(),
            original.expires_at(),
            original.spent_at(),
        );
        assert_eq!(original, geladen);
    }
}
