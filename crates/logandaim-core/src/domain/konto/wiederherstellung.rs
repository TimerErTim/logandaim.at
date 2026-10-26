use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{KontoFehler, KontoId, PASSWORT_WIEDERHERSTELLUNG_GÜLTIGKEIT, TokenHash};

/// Identity of one Passwort-Wiederherstellung token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WiederherstellungId(pub Uuid);

/// Creation input of a Passwort-Wiederherstellung.
#[derive(Debug, Clone)]
pub struct NeueWiederherstellung {
    konto_id: KontoId,
    token_hash: TokenHash,
    erstellt_am: DateTime<Utc>,
}

impl NeueWiederherstellung {
    pub fn neu(konto_id: KontoId, token_hash: TokenHash, erstellt_am: DateTime<Utc>) -> Self {
        Self {
            konto_id,
            token_hash,
            erstellt_am,
        }
    }

    pub fn konto_id(&self) -> KontoId {
        self.konto_id
    }

    pub fn token_hash(&self) -> &TokenHash {
        &self.token_hash
    }

    pub fn erstellt_am(&self) -> DateTime<Utc> {
        self.erstellt_am
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.erstellt_am + PASSWORT_WIEDERHERSTELLUNG_GÜLTIGKEIT
    }
}

/// Authorisation to set a new Passwort on one Konto, mailed by `PasswortVergessen`.
///
/// Single use, bound to one Konto, valid 60 minutes; requesting a new one spends the
/// previous (SPEC-konto rule 7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswortWiederherstellung {
    id: WiederherstellungId,
    konto_id: KontoId,
    token_hash: TokenHash,
    expires_at: DateTime<Utc>,
    spent_at: Option<DateTime<Utc>>,
}

impl PasswortWiederherstellung {
    pub fn neu(id: WiederherstellungId, neu: NeueWiederherstellung) -> Self {
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
        id: WiederherstellungId,
        konto_id: KontoId,
        token_hash: TokenHash,
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

    pub fn id(&self) -> WiederherstellungId {
        self.id
    }

    pub fn konto_id(&self) -> KontoId {
        self.konto_id
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

    pub fn ist_gültig(&self, jetzt: DateTime<Utc>) -> bool {
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

    fn wiederherstellung() -> PasswortWiederherstellung {
        PasswortWiederherstellung::neu(
            WiederherstellungId(Uuid::nil()),
            NeueWiederherstellung::neu(
                KontoId(Uuid::nil()),
                TokenHash::from_hex("b".repeat(64)).unwrap(),
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
        token.verbrauchen(jetzt()).unwrap();
        assert!(matches!(
            token.verbrauchen(jetzt()),
            Err(KontoFehler::TokenVerbraucht)
        ));
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
