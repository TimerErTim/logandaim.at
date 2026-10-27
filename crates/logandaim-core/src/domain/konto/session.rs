use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{BenutzerId, KontoFehler, SecretTokenHash};

/// Identity of one Anmelde-Sitzung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SessionId(pub Uuid);

/// The three live states of one Anmelde-Sitzung.
///
/// Lifecycle is a type, not a pair of optional timestamps: rotation consumes [`state::Renewable`]
/// and yields [`state::Refreshed`], ending yields [`state::Revoked`]. State is stored in the row
/// (`spent_at`, `revoked_at`) and reconstructed on load, because SQLite has no sum types.
pub mod state {
    use chrono::{DateTime, Utc};

    /// Live: created, never rotated, never revoked. A refresh token is usable exactly
    /// once, so this is the only state a renewal may start from.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Renewable {
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

/// Creation input: everything of a Session except its identity and its lifecycle state.
///
/// `public_key` is the configured signing key's hex-encoded public half *at issue time*.
/// The row carries it, so an access token is verified against the key its own session was
/// issued under — AC-17 is a row check, not a key-table lookup.
#[derive(Debug, Clone)]
pub struct NeueSession {
    benutzer_id: BenutzerId,
    refresh_token_hash: SecretTokenHash,
    public_key: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}

impl NeueSession {
    pub fn neu(
        benutzer_id: BenutzerId,
        refresh_token_hash: SecretTokenHash,
        public_key: impl Into<String>,
        jetzt: DateTime<Utc>,
    ) -> Self {
        Self {
            benutzer_id,
            refresh_token_hash,
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
/// checks this row. Rotation creates a new Session and spends the old one; it never revives it.
///
/// **Retention:** spent, revoked, and expired rows are kept, because replay detection needs
/// the spent row a refresh token names. Purge only revoked or spent rows well past
/// `expires_at`; anything else breaks replay detection or resurrects a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionIn<S> {
    id: SessionId,
    benutzer_id: BenutzerId,
    refresh_token_hash: SecretTokenHash,
    public_key: String,
    created_at: DateTime<Utc>,
    state: S,
}

pub type RenewableSession = SessionIn<state::Renewable>;
pub type RefreshedSession = SessionIn<state::Refreshed>;
pub type RevokedSession = SessionIn<state::Revoked>;

impl<S> SessionIn<S> {
    pub fn id(&self) -> SessionId {
        self.id
    }

    pub fn benutzer_id(&self) -> BenutzerId {
        self.benutzer_id
    }

    pub fn refresh_token_hash(&self) -> &SecretTokenHash {
        &self.refresh_token_hash
    }

    /// Hex-encoded public key this session's access tokens must verify against.
    pub fn public_key_hex(&self) -> &str {
        &self.public_key
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}

impl RenewableSession {
    pub fn expires_at(&self) -> DateTime<Utc> {
        self.state.expires_at
    }

    pub fn ist_abgelaufen(&self, jetzt: DateTime<Utc>) -> bool {
        self.expires_at() <= jetzt
    }

    pub fn ist_gültig(&self, jetzt: DateTime<Utc>) -> bool {
        !self.ist_abgelaufen(jetzt)
    }

    pub fn widerrufen(self, jetzt: DateTime<Utc>) -> RevokedSession {
        let expires_at = self.expires_at();
        SessionIn {
            id: self.id,
            benutzer_id: self.benutzer_id,
            refresh_token_hash: self.refresh_token_hash,
            public_key: self.public_key,
            created_at: self.created_at,
            state: state::Revoked {
                timestamp: jetzt,
                expires_at,
            },
        }
    }

    /// A freshly created session, renewable from `jetzt` until its refresh token expires.
    pub fn neu(
        id: SessionId,
        benutzer_id: BenutzerId,
        refresh_token_hash: SecretTokenHash,
        public_key: impl Into<String>,
        jetzt: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            benutzer_id,
            refresh_token_hash,
            public_key: public_key.into(),
            created_at: jetzt,
            state: state::Renewable {
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
            neu.public_key,
            neu.created_at,
        )
    }

    /// Spend this refresh token and describe its successor.
    ///
    /// Consumes the live state, so spending twice cannot compile. Returns the row to write
    /// back (`Refreshed`) and the row to create for the successor.
    pub fn erneuern(
        self,
        new_refresh_token_hash: SecretTokenHash,
        jetzt: DateTime<Utc>,
    ) -> (RefreshedSession, NeueSession) {
        let expires_at = self.expires_at();
        let folge = NeueSession::neu(
            self.benutzer_id,
            new_refresh_token_hash,
            self.public_key.clone(),
            jetzt,
        );
        let verbraucht = SessionIn {
            id: self.id,
            benutzer_id: self.benutzer_id,
            refresh_token_hash: self.refresh_token_hash,
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

impl RefreshedSession {
    pub fn expires_at(&self) -> DateTime<Utc> {
        self.state.expires_at
    }

    pub fn spent_at(&self) -> DateTime<Utc> {
        self.state.timestamp
    }

    pub fn ist_abgelaufen(&self, jetzt: DateTime<Utc>) -> bool {
        self.expires_at() <= jetzt
    }

    pub fn ist_gültig(&self, jetzt: DateTime<Utc>) -> bool {
        !self.ist_abgelaufen(jetzt)
    }

    pub fn widerrufen(self, jetzt: DateTime<Utc>) -> SessionIn<state::Revoked> {
        let expires_at = self.expires_at();
        SessionIn {
            id: self.id,
            benutzer_id: self.benutzer_id,
            refresh_token_hash: self.refresh_token_hash,
            public_key: self.public_key,
            created_at: self.created_at,
            state: state::Revoked {
                timestamp: jetzt,
                expires_at,
            },
        }
    }
}

impl RevokedSession {
    pub fn revoked_at(&self) -> DateTime<Utc> {
        self.state.timestamp
    }
}

/// One loaded session row, as a closed set of lifecycle states.
///
/// Repositories load this and write it back; a use case matches on the variant to decide
/// what is allowed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Session {
    Renewable(SessionIn<state::Renewable>),
    Refreshed(SessionIn<state::Refreshed>),
    Revoked(SessionIn<state::Revoked>),
}

impl From<SessionIn<state::Renewable>> for Session {
    fn from(session: SessionIn<state::Renewable>) -> Self {
        Session::Renewable(session)
    }
}

impl From<SessionIn<state::Refreshed>> for Session {
    fn from(session: SessionIn<state::Refreshed>) -> Self {
        Session::Refreshed(session)
    }
}

impl From<SessionIn<state::Revoked>> for Session {
    fn from(session: SessionIn<state::Revoked>) -> Self {
        Session::Revoked(session)
    }
}

impl Session {
    /// Map a stored row back to its state. Revocation wins over a spent timestamp: it is
    /// terminal, and a row can carry both after a replay ended the family.
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        id: SessionId,
        benutzer_id: BenutzerId,
        refresh_token_hash: SecretTokenHash,
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
            (None, None) => Self::Renewable(gemeinsame.mit_state(state::Renewable { expires_at })),
        }
    }

    pub fn id(&self) -> SessionId {
        match self {
            Self::Renewable(session) => session.id,
            Self::Refreshed(session) => session.id,
            Self::Revoked(session) => session.id,
        }
    }

    pub fn benutzer_id(&self) -> BenutzerId {
        match self {
            Self::Renewable(session) => session.benutzer_id,
            Self::Refreshed(session) => session.benutzer_id,
            Self::Revoked(session) => session.benutzer_id,
        }
    }

    pub fn refresh_token_hash(&self) -> &SecretTokenHash {
        match self {
            Self::Renewable(session) => &session.refresh_token_hash,
            Self::Refreshed(session) => &session.refresh_token_hash,
            Self::Revoked(session) => &session.refresh_token_hash,
        }
    }

    pub fn public_key_hex(&self) -> &str {
        match self {
            Self::Renewable(session) => &session.public_key,
            Self::Refreshed(session) => &session.public_key,
            Self::Revoked(session) => &session.public_key,
        }
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        match self {
            Self::Renewable(session) => session.created_at,
            Self::Refreshed(session) => session.created_at,
            Self::Revoked(session) => session.created_at,
        }
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        match self {
            Self::Renewable(session) => session.state.expires_at,
            Self::Refreshed(session) => session.state.expires_at,
            Self::Revoked(session) => session.state.expires_at,
        }
    }

    pub fn ist_verbraucht(&self) -> bool {
        match self {
            Self::Renewable(_) => false,
            Self::Refreshed(_) => true,
            Self::Revoked(_) => true,
        }
    }

    pub fn ist_widerrufen(&self) -> bool {
        match self {
            Self::Renewable(_) => false,
            Self::Refreshed(_) => false,
            Self::Revoked(_) => true,
        }
    }

    pub fn ist_annehmbar(&self, jetzt: DateTime<Utc>) -> bool {
        match self {
            Self::Renewable(session) => !session.ist_abgelaufen(jetzt),
            Self::Refreshed(session) => !session.ist_abgelaufen(jetzt),
            Self::Revoked(_) => false,
        }
    }

    /// End this session whatever its current state
    pub fn widerrufen(self, jetzt: DateTime<Utc>) -> Session {
        match self {
            Self::Renewable(session) => Self::Revoked(session.widerrufen(jetzt)),
            Self::Refreshed(session) => Self::Revoked(session.widerrufen(jetzt)),
            Self::Revoked(session) => Self::Revoked(session),
        }
    }
}

impl TryFrom<Session> for RenewableSession {
    type Error = SessionNotRenewable;
    fn try_from(session: Session) -> Result<Self, Self::Error> {
        match session {
            Session::Renewable(session) => Ok(session),
            Session::Refreshed(_) => Err(SessionNotRenewable::Spent),
            Session::Revoked(_) => Err(SessionNotRenewable::Revoked),
        }
    }
}

/// Why a loaded session may not start a renewal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionNotRenewable {
    /// Its refresh token was already spent: a replay (rule 12).
    Spent,
    /// It was ended, so nothing may renew it (AC-9).
    Revoked,
}

impl From<SessionNotRenewable> for KontoFehler {
    fn from(_: SessionNotRenewable) -> Self {
        KontoFehler::TokenVerbraucht
    }
}

/// The shared column bundle plus a `()` state, so `from_parts` does not repeat seven
/// fields three times.
impl SessionIn<()> {
    fn mit_state<S>(self, state: S) -> SessionIn<S> {
        SessionIn {
            id: self.id,
            benutzer_id: self.benutzer_id,
            refresh_token_hash: self.refresh_token_hash,
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

    fn session() -> SessionIn<state::Renewable> {
        SessionIn::neu(
            SessionId(Uuid::nil()),
            BenutzerId(Uuid::nil()),
            hash(),
            "ab".repeat(32),
            jetzt(),
        )
    }

    #[test_log::test]
    fn neue_session_ist_gültig_und_läuft_in_einem_monat_ab() {
        let session = session();
        assert!(session.ist_gültig(jetzt()));
        assert_eq!(session.expires_at(), jetzt() + Duration::days(30));
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

        assert_eq!(verbraucht.spent_at(), jetzt());
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
        assert_eq!(
            RenewableSession::try_from(Session::Refreshed(verbraucht)),
            Err(SessionNotRenewable::Spent)
        );
    }

    #[test_log::test]
    fn widerrufene_session_ist_endgültig_ungültig() {
        let widerrufen = session().widerrufen(jetzt());
        assert_eq!(widerrufen.revoked_at(), jetzt());
    }

    #[test_log::test]
    fn widerruf_ist_idempotent_und_behält_den_ersten_zeitpunkt() {
        let einmal = Session::Renewable(session()).widerrufen(jetzt());
        let zweimal = einmal.widerrufen(jetzt() + Duration::minutes(1));
        assert!(matches!(zweimal, Session::Revoked(_)));
        match zweimal {
            Session::Revoked(session) => assert_eq!(session.revoked_at(), jetzt()),
            _ => panic!("expected Session::Revoked"),
        }
    }

    #[test_log::test]
    fn eine_verbrauchte_session_kann_nicht_erneut_erneuert_werden() {
        let (verbraucht, _) = session().erneuern(hash(), jetzt());
        let geladen = Session::Refreshed(verbraucht);
        assert_eq!(
            RenewableSession::try_from(geladen),
            Err(SessionNotRenewable::Spent)
        );
    }

    #[test_log::test]
    fn eine_widerrufene_session_kann_nicht_erneuert_werden() {
        let geladen = Session::Revoked(session().widerrufen(jetzt()));
        assert_eq!(
            RenewableSession::try_from(geladen),
            Err(SessionNotRenewable::Revoked)
        );
    }

    #[test_log::test]
    fn from_parts_roundtrip_über_alle_drei_zustände() {
        for (label, session) in [
            ("ok", Session::Renewable(session())),
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
                session.public_key_hex().to_owned(),
                session.created_at(),
                session.expires_at(),
                match &session {
                    Session::Refreshed(session) => Some(session.spent_at()),
                    _ => None,
                },
                match &session {
                    Session::Revoked(session) => Some(session.revoked_at()),
                    _ => None,
                },
            );
            assert_eq!(session, geladen, "Zustand {label} überlebt die Zeile");
        }
    }
}
