//! The executing identity of one use-case run.
//!
//! Every `App::execute` / `App::execute_fn` receives one, injected by the API layer from
//! the verified access token. It is the code expression of SPEC-konto rule 17: a use case
//! decides authorisation itself, from the aggregates it loads, and never trusts a
//! permission string handed in by an endpoint.
//!
//! Fail-closed: [`AuthContext::Anonymous`] denies, and there is no third state that
//! grants anything.

use error_stack::Report;

use super::ResultReport;
use crate::domain::konto::{BenutzerId, SessionId};

/// Who is executing the use case.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthContext {
    /// No verified access token. Everything that needs an identity is refused.
    Anonymous,
    /// A verified, live Anmelde-Sitzung.
    Authenticated {
        benutzer_id: BenutzerId,
        session_id: SessionId,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum ZugriffFehler {
    #[error("für diese Aktion ist eine Anmeldung erforderlich")]
    NichtAngemeldet,
    #[error("dieser Benutzer darf diese Aktion nicht ausführen")]
    NichtErlaubt,
}

impl AuthContext {
    pub fn benutzer_id(&self) -> Option<BenutzerId> {
        match self {
            Self::Anonymous => None,
            Self::Authenticated { benutzer_id, .. } => Some(*benutzer_id),
        }
    }

    pub fn session_id(&self) -> Option<SessionId> {
        match self {
            Self::Anonymous => None,
            Self::Authenticated { session_id, .. } => Some(*session_id),
        }
    }

    pub fn ist_angemeldet(&self) -> bool {
        matches!(self, Self::Authenticated { .. })
    }

    /// The identity pair, or a refusal. Every use case that acts on personal data starts
    /// with this call, in its own body.
    pub fn require_authenticated(&self) -> ResultReport<(BenutzerId, SessionId), ZugriffFehler> {
        match self {
            Self::Anonymous => Err(Report::new(ZugriffFehler::NichtAngemeldet)),
            Self::Authenticated {
                benutzer_id,
                session_id,
            } => Ok((*benutzer_id, *session_id)),
        }
    }

    /// The signed-in Benutzer, or a refusal.
    pub fn require_benutzer(&self) -> ResultReport<BenutzerId, ZugriffFehler> {
        self.require_authenticated()
            .map(|(benutzer_id, _)| benutzer_id)
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    fn id() -> (BenutzerId, SessionId) {
        (BenutzerId(Uuid::nil()), SessionId(Uuid::nil()))
    }

    #[test_log::test]
    fn anonym_verweigert() {
        let auth = AuthContext::Anonymous;
        assert!(!auth.ist_angemeldet());
        assert_eq!(auth.benutzer_id(), None);
        assert!(auth.require_authenticated().is_err());
        assert!(auth.require_benutzer().is_err());
    }

    #[test_log::test]
    fn angemeldet_liefert_die_identitaet() {
        let (benutzer_id, session_id) = id();
        let auth = AuthContext::Authenticated {
            benutzer_id,
            session_id,
        };
        assert!(auth.ist_angemeldet());
        assert_eq!(auth.benutzer_id(), Some(benutzer_id));
        assert_eq!(auth.session_id(), Some(session_id));
        assert_eq!(
            auth.require_authenticated().unwrap(),
            (benutzer_id, session_id)
        );
    }
}
