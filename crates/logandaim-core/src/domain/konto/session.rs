use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{BenutzerId, KontoFehler, SecretTokenHash, SigningKeyId};

/// Identity of one Anmelde-Sitzung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SessionId(pub Uuid);

/// The three live states of one Anmelde-Sitzung.
///
/// A session's lifecycle is a type, not a pair of optional timestamps: rotation consumes
/// [`state::Ok`] and yields [`state::Refreshed`], ending a session consumes either and
/// yields [`state::Revoked`]. A caller therefore cannot spend one refresh token twice, and
/// cannot start a renewal from a session that has already been rotated or ended.
///
/// State is stored in the row (`spent_at`, `revoked_at`) and reconstructed on load, because
/// SQLite has no sum types.
pub mod state {
    use chrono::{DateTime, Utc};

    /// Live: created, never rotated, never revoked. A refresh token is usable exactly
    /// once, so this is the only state a renewal may start from.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Ok {
        pub(super) expires_at: DateTime<Utc>,
    }

    /// Rotated: its refresh token was spent at `timestamp`. The session itself survives
    /// and its access token keeps working until `expires_at` (AC-7); presenting the spent
    /// token again is a replay and ends the whole family (rule 12).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Refreshed {
        pub(super) timestamp: DateTime<Utc>,
        pub(super) expires_at: DateTime<Utc>,
    }

    /// Ended: its access token fails from `timestamp` on, even though the signature still
    /// verifies (AC-9, AC-17). Terminal — a revoked session has no transition back.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Revoked {
        pub(super) timestamp: DateTime<Utc>,
        pub(super) expires_at: DateTime<Utc>,
    }
}

/// The read surface the three states share.
pub trait SessionState {
    fn expires_at(&self) -> DateTime<Utc>;
    fn spent_at(&self) -> Option<DateTime<Utc>>;
    fn revoked_at(&self) -> Option<DateTime<Utc>>;
}

impl SessionState for state::Ok {
    fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    fn spent_at(&self) -> Option<DateTime<Utc>> {
        None
    }

    fn revoked_at(&self) -> Option<DateTime<Utc>> {
        None
    }
}

impl SessionState for state::Refreshed {
    fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    fn spent_at(&self) -> Option<DateTime<Utc>> {
        Some(self.timestamp)
    }

    fn revoked_at(&self) -> Option<DateTime<Utc>> {
        None
    }
}

impl SessionState for state::Revoked {
    fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    /// Revocation is terminal, so "was the refresh token spent first" no longer changes
    /// any decision; the row keeps the revocation instead of both timestamps.
    fn spent_at(&self) -> Option<DateTime<Utc>> {
        None
    }

    fn revoked_at(&self) -> Option<DateTime<Utc>> {
        Some(self.timestamp)
    }
}

/// Creation input: everything of a Session except its identity and its lifecycle state.
///
/// `public_key` is the configured signing key's hex-encoded public half *at issue time*.
/// The row carries it, so an access token is verified against the key its own session was
/// issued under — AC-17 is a row check, not a key-table lookup.
#[derive(Debug, Clone)]
pub struct NeueSession {
    benutzer_id: BenutzerId,
    refresh_token_hash: SecretTokenHash,
    signing_key_id: SigningKeyId,
    public_key: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl NeueSession {
    pub fn neu(
        benutzer_id: BenutzerId,
        refresh_token_hash: SecretTokenHash,
        signing_key_id: SigningKeyId,
        public_key: impl Into<String>,
        jetzt: DateTime<Utc>,
    ) -> Self {
        Self {
            benutzer_id,
            refresh_token_hash,
            signing_key_id,
            public_key: public_key.into(),
            created_at: jetzt,
            expires_at: jetzt + super::REFRESH_TOKEN_LIFETIME,
        }
    }

    pub fn benutzer_id(&self) -> BenutzerId {
        self.benutzer_id
    }

    pub fn refresh_token_hash(&self) -> &SecretTokenHash {
        &self.refresh_token_hash
    }

    pub fn signing_key_id(&self) -> &SigningKeyId {
        &self.signing_key_id
    }

    pub fn public_key_hex(&self) -> &str {
        &self.public_key
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }
}

/// One logged-in state of one Benutzer on one device, in one lifecycle state.
///
/// Revocation is a row state, not a token expiry (SPEC-konto rule 15), so every request
/// that presents an access token checks this row. Rotation creates a new Session and
/// spends the old one; it never mutates a session back to life.
///
/// **Retention:** rows are kept when they are spent, revoked, or expired. Replay detection
/// depends on still finding the row that a spent refresh token names, and a revocation
/// timestamp is the record of what happened. A purge job may only remove rows that are
/// revoked or spent *and* whose `expires_at` is comfortably in the past — deleting an
/// unexpired or unspent row would either break replay detection or resurrect a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionIn<S> {
    id: SessionId,
    benutzer_id: BenutzerId,
    refresh_token_hash: SecretTokenHash,
    signing_key_id: SigningKeyId,
    public_key: String,
    created_at: DateTime<Utc>,
    state: S,
}

impl<S: SessionState> SessionIn<S> {
    pub fn id(&self) -> SessionId {
        self.id
    }

    pub fn benutzer_id(&self) -> BenutzerId {
        self.benutzer_id
    }

    pub fn refresh_token_hash(&self) -> &SecretTokenHash {
        &self.refresh_token_hash
    }

    /// The `kid` this session's access token names.
    pub fn signing_key_id(&self) -> &SigningKeyId {
        &self.signing_key_id
    }

    /// Hex-encoded public key this session's access tokens must verify against.
    pub fn public_key_hex(&self) -> &str {
        &self.public_key
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        self.state.expires_at()
    }

    pub fn spent_at(&self) -> Option<DateTime<Utc>> {
        self.state.spent_at()
    }

    pub fn revoked_at(&self) -> Option<DateTime<Utc>> {
        self.state.revoked_at()
    }

    pub fn ist_abgelaufen(&self, jetzt: DateTime<Utc>) -> bool {
        self.expires_at() <= jetzt
    }

    pub fn ist_verbraucht(&self) -> bool {
        self.state.spent_at().is_some()
    }

    pub fn ist_widerrufen(&self) -> bool {
        self.state.revoked_at().is_some()
    }

    /// Whether an access token issued under this session is still accepted.
    ///
    /// Deliberately weaker than a renewal test: a spent refresh token does not end the
    /// session, so the access token handed out before a rotation keeps working until it
    /// expires (AC-7). Only revocation and expiry end it (AC-9, AC-17).
    pub fn ist_annehmbar(&self, jetzt: DateTime<Utc>) -> bool {
        !self.ist_widerrufen() && !self.ist_abgelaufen(jetzt)
    }

    /// End the session whatever its state: its access token fails on the next request
    /// (AC-9, AC-17). Idempotent, so re-revoking keeps the first timestamp.
    pub fn widerrufen(self, jetzt: DateTime<Utc>) -> SessionIn<state::Revoked> {
        let expires_at = self.expires_at();
        SessionIn {
            id: self.id,
            benutzer_id: self.benutzer_id,
            refresh_token_hash: self.refresh_token_hash,
            signing_key_id: self.signing_key_id,
            public_key: self.public_key,
            created_at: self.created_at,
            state: state::Revoked {
                timestamp: self.state.revoked_at().unwrap_or(jetzt),
                expires_at,
            },
        }
    }
}

impl SessionIn<state::Ok> {
    /// A freshly created session, live from `jetzt` until its refresh token expires.
    pub fn neu(
        id: SessionId,
        benutzer_id: BenutzerId,
        refresh_token_hash: SecretTokenHash,
        signing_key_id: SigningKeyId,
        public_key: impl Into<String>,
        jetzt: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            benutzer_id,
            refresh_token_hash,
            signing_key_id,
            public_key: public_key.into(),
            created_at: jetzt,
            state: state::Ok {
                expires_at: jetzt + super::REFRESH_TOKEN_LIFETIME,
            },
        }
    }

    /// Build a live session from a [`NeueSession`] and its caller-supplied identity.
    pub fn aus(id: SessionId, neu: NeueSession) -> Self {
        Self::neu(
            id,
            neu.benutzer_id,
            neu.refresh_token_hash,
            neu.signing_key_id,
            neu.public_key,
            neu.created_at,
        )
    }

    /// Live: never rotated, never revoked, inside its window. Only askable of this state,
    /// which is what makes "renew a spent session" unrepresentable.
    pub fn ist_gültig(&self, jetzt: DateTime<Utc>) -> bool {
        !self.ist_abgelaufen(jetzt)
    }

    /// Spend this refresh token and describe its successor.
    ///
    /// Consumes the live state, so spending twice cannot compile. Returns the row to write
    /// back (`Refreshed`) and the row to create for the successor.
    pub fn erneuern(
        self,
        refresh_token_hash: SecretTokenHash,
        jetzt: DateTime<Utc>,
    ) -> (SessionIn<state::Refreshed>, NeueSession) {
        let expires_at = self.expires_at();
        let folge = NeueSession {
            benutzer_id: self.benutzer_id,
            refresh_token_hash,
            signing_key_id: self.signing_key_id.clone(),
            public_key: self.public_key.clone(),
            created_at: jetzt,
            expires_at: jetzt + super::REFRESH_TOKEN_LIFETIME,
        };
        let verbraucht = SessionIn {
            id: self.id,
            benutzer_id: self.benutzer_id,
            refresh_token_hash: self.refresh_token_hash,
            signing_key_id: self.signing_key_id,
            public_key: self.public_key,
            created_at: self.created_at,
            state: state::Refreshed {
                timestamp: jetzt,
                expires_at,
            },
        };
        (verbraucht, folge)
    }
}

/// One loaded session row, as a closed set of lifecycle states.
///
/// Repositories load this and write it back; a use case matches on the variant to decide
/// what is allowed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Session {
    Ok(SessionIn<state::Ok>),
    Refreshed(SessionIn<state::Refreshed>),
    Revoked(SessionIn<state::Revoked>),
}

impl Session {
    /// Map a stored row back to its state. Revocation wins over a spent timestamp: it is
    /// terminal, and a row can carry both after a replay ended the family.
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: SessionId,
        benutzer_id: BenutzerId,
        refresh_token_hash: SecretTokenHash,
        signing_key_id: SigningKeyId,
        public_key: String,
        created_at: DateTime<Utc>,
        expires_at: DateTime<Utc>,
        spent_at: Option<DateTime<Utc>>,
        revoked_at: Option<DateTime<Utc>>,
    ) -> Self {
        let gemeinsame = SessionIn {
            id,
            benutzer_id,
            refresh_token_hash,
            signing_key_id,
            public_key,
            created_at,
            state: (),
        };
        match (spent_at, revoked_at) {
            (_, Some(timestamp)) => Self::Revoked(gemeinsame.mit_state(state::Revoked {
                timestamp,
                expires_at,
            })),
            (Some(timestamp), None) => Self::Refreshed(gemeinsame.mit_state(state::Refreshed {
                timestamp,
                expires_at,
            })),
            (None, None) => Self::Ok(gemeinsame.mit_state(state::Ok { expires_at })),
        }
    }

    pub fn id(&self) -> SessionId {
        match self {
            Self::Ok(session) => session.id,
            Self::Refreshed(session) => session.id,
            Self::Revoked(session) => session.id,
        }
    }

    pub fn benutzer_id(&self) -> BenutzerId {
        match self {
            Self::Ok(session) => session.benutzer_id,
            Self::Refreshed(session) => session.benutzer_id,
            Self::Revoked(session) => session.benutzer_id,
        }
    }

    pub fn refresh_token_hash(&self) -> &SecretTokenHash {
        match self {
            Self::Ok(session) => &session.refresh_token_hash,
            Self::Refreshed(session) => &session.refresh_token_hash,
            Self::Revoked(session) => &session.refresh_token_hash,
        }
    }

    pub fn signing_key_id(&self) -> &SigningKeyId {
        match self {
            Self::Ok(session) => &session.signing_key_id,
            Self::Refreshed(session) => &session.signing_key_id,
            Self::Revoked(session) => &session.signing_key_id,
        }
    }

    pub fn public_key_hex(&self) -> &str {
        match self {
            Self::Ok(session) => &session.public_key,
            Self::Refreshed(session) => &session.public_key,
            Self::Revoked(session) => &session.public_key,
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        match self {
            Self::Ok(session) => session.created_at,
            Self::Refreshed(session) => session.created_at,
            Self::Revoked(session) => session.created_at,
        }
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        match self {
            Self::Ok(session) => session.state.expires_at,
            Self::Refreshed(session) => session.state.expires_at,
            Self::Revoked(session) => session.state.expires_at,
        }
    }

    pub fn spent_at(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Ok(session) => session.state.spent_at(),
            Self::Refreshed(session) => session.state.spent_at(),
            Self::Revoked(session) => session.state.spent_at(),
        }
    }

    pub fn revoked_at(&self) -> Option<DateTime<Utc>> {
        match self {
            Self::Ok(session) => session.state.revoked_at(),
            Self::Refreshed(session) => session.state.revoked_at(),
            Self::Revoked(session) => session.state.revoked_at(),
        }
    }

    pub fn ist_verbraucht(&self) -> bool {
        self.spent_at().is_some()
    }

    pub fn ist_widerrufen(&self) -> bool {
        self.revoked_at().is_some()
    }

    pub fn ist_annehmbar(&self, jetzt: DateTime<Utc>) -> bool {
        match self {
            Self::Ok(session) => session.ist_annehmbar(jetzt),
            Self::Refreshed(session) => session.ist_annehmbar(jetzt),
            Self::Revoked(session) => session.ist_annehmbar(jetzt),
        }
    }

    /// End this session whatever its current state, keeping the row (retention: replay
    /// detection needs it).
    pub fn widerrufen(self, jetzt: DateTime<Utc>) -> Session {
        match self {
            Self::Ok(session) => Self::Revoked(session.widerrufen(jetzt)),
            Self::Refreshed(session) => Self::Revoked(session.widerrufen(jetzt)),
            Self::Revoked(session) => Self::Revoked(session),
        }
    }

    /// The live state, or why this row may not start a renewal.
    pub fn into_live(self) -> Result<SessionIn<state::Ok>, SessionNotLive> {
        match self {
            Self::Ok(session) => Ok(session),
            Self::Refreshed(_) => Err(SessionNotLive::Spent),
            Self::Revoked(_) => Err(SessionNotLive::Revoked),
        }
    }
}

/// Why a loaded session may not start a renewal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionNotLive {
    /// Its refresh token was already spent: a replay (rule 12).
    Spent,
    /// It was ended, so nothing may renew it (AC-9).
    Revoked,
}

impl From<SessionNotLive> for KontoFehler {
    fn from(_: SessionNotLive) -> Self {
        KontoFehler::TokenVerbraucht
    }
}

/// The shared column bundle plus a `()` state, so `from_parts` does not repeat seven
/// fields three times.
impl SessionIn<()> {
    fn mit_state<S: SessionState>(self, state: S) -> SessionIn<S> {
        SessionIn {
            id: self.id,
            benutzer_id: self.benutzer_id,
            refresh_token_hash: self.refresh_token_hash,
            signing_key_id: self.signing_key_id,
            public_key: self.public_key,
            created_at: self.created_at,
            state,
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

    fn hash() -> SecretTokenHash {
        SecretTokenHash::from_hex("a".repeat(64)).unwrap()
    }

    fn kid() -> SigningKeyId {
        SigningKeyId::new("k1").unwrap()
    }

    fn session() -> SessionIn<state::Ok> {
        SessionIn::neu(
            SessionId(Uuid::nil()),
            BenutzerId(Uuid::nil()),
            hash(),
            kid(),
            "ab".repeat(32),
            jetzt(),
        )
    }

    #[test_log::test]
    fn neue_session_ist_gültig_und_läuft_in_einem_monat_ab() {
        let session = session();
        assert!(session.ist_gültig(jetzt()));
        assert_eq!(session.expires_at(), jetzt() + Duration::days(30));
        assert_eq!(session.spent_at(), None);
        assert_eq!(session.revoked_at(), None);
    }

    #[test_log::test]
    fn abgelaufene_session_ist_ungültig() {
        let session = session();
        assert!(session.ist_abgelaufen(jetzt() + Duration::days(30)));
        assert!(!session.ist_gültig(jetzt() + Duration::days(30)));
    }

    #[test_log::test]
    fn erneuerung_macht_ok_zu_refreshed_und_legt_die_folge_an() {
        let session = session();
        let (verbraucht, folge) = session.erneuern(hash(), jetzt());

        assert!(verbraucht.ist_verbraucht());
        assert_eq!(verbraucht.spent_at(), Some(jetzt()));
        assert_eq!(folge.benutzer_id(), BenutzerId(Uuid::nil()));
        assert_eq!(folge.public_key_hex(), "ab".repeat(32));
        assert_eq!(folge.expires_at(), jetzt() + Duration::days(30));
    }

    #[test_log::test]
    fn verbrauchte_session_lässt_das_alte_access_token_weiterleben() {
        // AC-7: rotating the refresh token must not invalidate the access token that was
        // issued with it; only revocation or expiry does (AC-9, AC-17).
        let (verbraucht, _) = session().erneuern(hash(), jetzt());
        // Reaching a renewal is what the type state forbids; validity (its window) is
        // still open, but only `Ok` can answer that question.
        assert!(verbraucht.ist_annehmbar(jetzt()));
        assert_eq!(
            Session::Refreshed(verbraucht).into_live(),
            Err(SessionNotLive::Spent)
        );
    }

    #[test_log::test]
    fn widerrufene_session_ist_endgültig_ungültig() {
        let widerrufen = session().widerrufen(jetzt());
        assert_eq!(widerrufen.revoked_at(), Some(jetzt()));
        assert!(!widerrufen.ist_annehmbar(jetzt()));
        assert!(!widerrufen.ist_annehmbar(jetzt() + Duration::minutes(1)));
    }

    #[test_log::test]
    fn widerruf_ist_idempotent_und_behält_den_ersten_zeitpunkt() {
        let einmal = Session::Ok(session()).widerrufen(jetzt());
        let zweimal = einmal.widerrufen(jetzt() + Duration::minutes(1));
        assert_eq!(zweimal.revoked_at(), Some(jetzt()));
    }

    #[test_log::test]
    fn eine_verbrauchte_session_kann_nicht_erneut_erneuert_werden() {
        let (verbraucht, _) = session().erneuern(hash(), jetzt());
        let geladen = Session::Refreshed(verbraucht);
        assert_eq!(geladen.into_live(), Err(SessionNotLive::Spent));
    }

    #[test_log::test]
    fn eine_widerrufene_session_kann_nicht_erneuert_werden() {
        let geladen = Session::Revoked(session().widerrufen(jetzt()));
        assert_eq!(geladen.into_live(), Err(SessionNotLive::Revoked));
    }

    #[test_log::test]
    fn from_parts_roundtrip_über_alle_drei_zustände() {
        for (label, session) in [
            ("ok", Session::Ok(session())),
            (
                "refreshed",
                Session::Refreshed(session().erneuern(hash(), jetzt()).0),
            ),
            ("revoked", Session::Revoked(session().widerrufen(jetzt()))),
        ] {
            let geladen = Session::from_parts(
                session.id(),
                session.benutzer_id(),
                session.refresh_token_hash().clone(),
                session.signing_key_id().clone(),
                session.public_key_hex().to_owned(),
                session.created_at(),
                session.expires_at(),
                session.spent_at(),
                session.revoked_at(),
            );
            assert_eq!(session, geladen, "Zustand {label} überlebt die Zeile");
        }
    }

    #[test_log::test]
    fn signing_key_id_wird_geprüft() {
        assert!(SigningKeyId::new("").is_err());
        assert!(SigningKeyId::new("mit leerzeichen").is_err());
        assert!(SigningKeyId::new("k-2026_10").is_ok());
    }
}
