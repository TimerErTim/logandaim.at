use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{BenutzerId, SigningKeyId, TokenHash};

/// Identity of one Anmelde-Sitzung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SessionId(pub Uuid);

/// Creation input: everything of a Session except its identity.
#[derive(Debug, Clone)]
pub struct NeueSession {
    benutzer_id: BenutzerId,
    refresh_token_hash: TokenHash,
    signing_key_id: SigningKeyId,
    erstellt_am: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl NeueSession {
    pub fn neu(
        benutzer_id: BenutzerId,
        refresh_token_hash: TokenHash,
        signing_key_id: SigningKeyId,
        erstellt_am: DateTime<Utc>,
    ) -> Self {
        Self {
            benutzer_id,
            refresh_token_hash,
            signing_key_id,
            erstellt_am,
            expires_at: erstellt_am + super::REFRESH_TOKEN_LIFETIME,
        }
    }

    pub fn benutzer_id(&self) -> BenutzerId {
        self.benutzer_id
    }

    pub fn refresh_token_hash(&self) -> &TokenHash {
        &self.refresh_token_hash
    }

    pub fn signing_key_id(&self) -> &SigningKeyId {
        &self.signing_key_id
    }

    pub fn erstellt_am(&self) -> DateTime<Utc> {
        self.erstellt_am
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }
}

/// One logged-in state of one Benutzer on one device.
///
/// Revocation is a row state, not a token expiry (SPEC-konto rule 15), so every request
/// that presents an access token checks this row. Rotation creates a new Session and
/// spends the old one; it never mutates a session back to life.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    id: SessionId,
    benutzer_id: BenutzerId,
    refresh_token_hash: TokenHash,
    signing_key_id: SigningKeyId,
    erstellt_am: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    spent_at: Option<DateTime<Utc>>,
    revoked_at: Option<DateTime<Utc>>,
}

impl Session {
    pub fn neu(id: SessionId, neu: NeueSession) -> Self {
        Self {
            id,
            benutzer_id: neu.benutzer_id,
            refresh_token_hash: neu.refresh_token_hash,
            signing_key_id: neu.signing_key_id,
            erstellt_am: neu.erstellt_am,
            expires_at: neu.expires_at,
            spent_at: None,
            revoked_at: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: SessionId,
        benutzer_id: BenutzerId,
        refresh_token_hash: TokenHash,
        signing_key_id: SigningKeyId,
        erstellt_am: DateTime<Utc>,
        expires_at: DateTime<Utc>,
        spent_at: Option<DateTime<Utc>>,
        revoked_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            benutzer_id,
            refresh_token_hash,
            signing_key_id,
            erstellt_am,
            expires_at,
            spent_at,
            revoked_at,
        }
    }

    pub fn id(&self) -> SessionId {
        self.id
    }

    pub fn benutzer_id(&self) -> BenutzerId {
        self.benutzer_id
    }

    pub fn refresh_token_hash(&self) -> &TokenHash {
        &self.refresh_token_hash
    }

    pub fn signing_key_id(&self) -> &SigningKeyId {
        &self.signing_key_id
    }

    pub fn erstellt_am(&self) -> DateTime<Utc> {
        self.erstellt_am
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    pub fn spent_at(&self) -> Option<DateTime<Utc>> {
        self.spent_at
    }

    pub fn revoked_at(&self) -> Option<DateTime<Utc>> {
        self.revoked_at
    }

    pub fn ist_abgelaufen(&self, jetzt: DateTime<Utc>) -> bool {
        self.expires_at <= jetzt
    }

    pub fn ist_widerrufen(&self) -> bool {
        self.revoked_at.is_some()
    }

    pub fn ist_verbraucht(&self) -> bool {
        self.spent_at.is_some()
    }

    /// Live: not revoked, not spent, not expired. This is what a *renewal* requires: the
    /// refresh token must not have been spent yet.
    pub fn ist_gueltig(&self, jetzt: DateTime<Utc>) -> bool {
        !self.ist_widerrufen() && !self.ist_verbraucht() && !self.ist_abgelaufen(jetzt)
    }

    /// Whether an access token issued under this session is still accepted.
    ///
    /// Deliberately weaker than [`Session::ist_gueltig`]: a spent refresh token does not
    /// end the session, so the access token handed out before a rotation keeps working
    /// until it expires (AC-7). Only revocation and expiry end it (AC-9, AC-17).
    pub fn ist_annehmbar(&self, jetzt: DateTime<Utc>) -> bool {
        !self.ist_widerrufen() && !self.ist_abgelaufen(jetzt)
    }

    /// Spend the refresh token. The session itself survives until it is revoked or
    /// expires, so the access token issued under it keeps working (AC-7).
    pub fn verbrauchen(&mut self, jetzt: DateTime<Utc>) {
        self.spent_at.get_or_insert(jetzt);
    }

    /// End the session: its access token fails on the next request (AC-9, AC-17).
    pub fn widerrufen(&mut self, jetzt: DateTime<Utc>) {
        self.revoked_at.get_or_insert(jetzt);
    }

    /// A renewal never mutates this session; it creates its successor.
    pub fn erneuern(&self, refresh_token_hash: TokenHash, jetzt: DateTime<Utc>) -> NeueSession {
        NeueSession {
            benutzer_id: self.benutzer_id,
            refresh_token_hash,
            signing_key_id: self.signing_key_id.clone(),
            erstellt_am: jetzt,
            expires_at: jetzt + super::REFRESH_TOKEN_LIFETIME,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    fn jetzt() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn hash() -> TokenHash {
        TokenHash::from_hex("a".repeat(64)).unwrap()
    }

    fn kid() -> SigningKeyId {
        SigningKeyId::new("k1").unwrap()
    }

    fn session() -> Session {
        Session::neu(
            SessionId(Uuid::nil()),
            NeueSession::neu(BenutzerId(Uuid::nil()), hash(), kid(), jetzt()),
        )
    }

    #[test_log::test]
    fn neue_session_ist_gueltig_und_laeuft_in_einem_monat_ab() {
        let session = session();
        assert!(session.ist_gueltig(jetzt()));
        assert_eq!(session.expires_at(), jetzt() + Duration::days(30));
    }

    #[test_log::test]
    fn abgelaufene_session_ist_ungueltig() {
        let session = session();
        assert!(session.ist_abgelaufen(jetzt() + Duration::days(30)));
        assert!(!session.ist_gueltig(jetzt() + Duration::days(30)));
    }

    #[test_log::test]
    fn verbrauchte_session_ist_ungueltig() {
        let mut session = session();
        session.verbrauchen(jetzt());
        assert!(session.ist_verbraucht());
        assert!(!session.ist_gueltig(jetzt()));
        // Spending twice keeps the first timestamp.
        session.verbrauchen(jetzt() + Duration::minutes(1));
        assert_eq!(session.spent_at(), Some(jetzt()));
    }

    #[test_log::test]
    fn verbrauchte_session_laesst_das_alte_access_token_weiterleben() {
        // AC-7: rotating the refresh token must not invalidate the access token that was
        // issued with it; only revocation or expiry does (AC-9, AC-17).
        let mut session = session();
        session.verbrauchen(jetzt());
        assert!(session.ist_annehmbar(jetzt()));
    }

    #[test_log::test]
    fn widerrufene_session_ist_endgueltig_ungueltig() {
        let mut session = session();
        session.widerrufen(jetzt());
        assert!(!session.ist_annehmbar(jetzt()));
        assert!(!session.ist_annehmbar(jetzt() + Duration::minutes(1)));
    }

    #[test_log::test]
    fn widerrufene_session_ist_undgueltig() {
        let mut session = session();
        session.widerrufen(jetzt());
        assert!(session.ist_widerrufen());
        assert!(!session.ist_gueltig(jetzt()));
    }

    #[test_log::test]
    fn erneuerung_erzeugt_eine_neue_session() {
        let session = session();
        let neu = session.erneuern(hash(), jetzt());
        assert_eq!(neu.benutzer_id(), session.benutzer_id());
        assert_eq!(neu.signing_key_id(), session.signing_key_id());
        assert_eq!(neu.expires_at(), jetzt() + Duration::days(30));
        // The old session was not touched by rotation.
        assert!(session.ist_gueltig(jetzt()));
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let original = session();
        let geladen = Session::from_parts(
            original.id(),
            original.benutzer_id(),
            original.refresh_token_hash().clone(),
            original.signing_key_id().clone(),
            original.erstellt_am(),
            original.expires_at(),
            original.spent_at(),
            original.revoked_at(),
        );
        assert_eq!(original, geladen);
    }

    #[test_log::test]
    fn signing_key_id_wird_geprueft() {
        assert!(SigningKeyId::new("").is_err());
        assert!(SigningKeyId::new("mit leerzeichen").is_err());
        assert!(SigningKeyId::new("k-2026_10").is_ok());
    }
}
