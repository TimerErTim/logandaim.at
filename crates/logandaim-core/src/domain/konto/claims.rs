//! Claim *semantics*, without any serialization.
//!
//! The JWT envelope (JSON, base64url, the JOSE header, `alg` pinning) lives in
//! `logandaim-api`. Core receives the already-parsed claim values and decides whether they
//! are acceptable: is the window open, does the issuer and audience match.

use chrono::{DateTime, Utc};

use crate::domain::konto::ACCESS_TOKEN_LIFETIME;

use super::{BenutzerId, CLOCK_SKEW, KontoFehler, SessionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenId(pub uuid::Uuid);

/// The claims an access token carries, already deserialized by the API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessTokenClaims {
    /// The Benutzer the token acts for.
    pub sub: BenutzerId,
    /// The Anmelde-Sitzung whose revoked state decides acceptance.
    pub sid: SessionId,
    pub iss: String,
    pub aud: String,
    pub iat: DateTime<Utc>,
    pub exp: DateTime<Utc>,
    /// Unique per token, so a replayed envelope is distinguishable in logs.
    pub jti: TokenId,
}

impl AccessTokenClaims {
    pub fn neu(
        benutzer_id: BenutzerId,
        session_id: SessionId,
        issuer: &str,
        audience: &str,
        jetzt: DateTime<Utc>,
        token_id: TokenId,
    ) -> Self {
        Self {
            sub: benutzer_id,
            sid: session_id,
            iss: issuer.to_owned(),
            aud: audience.to_owned(),
            iat: jetzt,
            exp: jetzt + ACCESS_TOKEN_LIFETIME,
            jti: token_id,
        }
    }

    /// `exp` in the future and `iat` not implausibly ahead, both with [`CLOCK_SKEW`]
    /// tolerance so a slightly fast issuer and a slightly slow verifier agree.
    pub fn prüfe_zeitfenster(&self, jetzt: DateTime<Utc>) -> Result<(), KontoFehler> {
        if jetzt - CLOCK_SKEW >= self.exp {
            return Err(KontoFehler::TokenAbgelaufen);
        }
        if self.iat - CLOCK_SKEW > jetzt {
            return Err(KontoFehler::TokenNochNichtGültig);
        }
        if self.exp <= self.iat {
            return Err(KontoFehler::TokenUngültig);
        }
        Ok(())
    }

    /// `iss` and `aud` must be exactly the configured pair; no prefix or suffix match.
    pub fn prüfe_herkunft(&self, iss: &str, aud: &str) -> Result<(), KontoFehler> {
        if self.iss != iss || self.aud != aud {
            return Err(KontoFehler::TokenHerkunftUngültig);
        }
        Ok(())
    }

    /// Whether this token names `benutzer_id`.
    pub fn gilt_fuer(&self, benutzer_id: BenutzerId) -> bool {
        self.sub == benutzer_id
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use uuid::Uuid;

    use super::*;
    use crate::domain::konto::{ACCESS_TOKEN_LIFETIME, CLOCK_SKEW};

    fn jetzt() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    fn claims(iat: DateTime<Utc>, exp: DateTime<Utc>) -> AccessTokenClaims {
        AccessTokenClaims {
            sub: BenutzerId(Uuid::nil()),
            sid: SessionId(Uuid::nil()),
            iss: "logandaim.at".into(),
            aud: "logandaim-api".into(),
            iat,
            exp,
            jti: TokenId(Uuid::nil()),
        }
    }

    #[test_log::test]
    fn gültiges_fenster_wird_akzeptiert() {
        let claims = claims(jetzt(), jetzt() + ACCESS_TOKEN_LIFETIME);
        assert!(claims.prüfe_zeitfenster(jetzt()).is_ok());
    }

    #[test_log::test]
    fn abgelaufenes_token_wird_abgelehnt() {
        let claims = claims(jetzt(), jetzt() + ACCESS_TOKEN_LIFETIME);
        // Past `exp` by more than the tolerated skew.
        assert!(matches!(
            claims.prüfe_zeitfenster(jetzt() + ACCESS_TOKEN_LIFETIME + CLOCK_SKEW),
            Err(KontoFehler::TokenAbgelaufen)
        ));
    }

    #[test_log::test]
    fn uhrzeitabweichung_wird_toleriert() {
        let claims = claims(jetzt(), jetzt() + ACCESS_TOKEN_LIFETIME);
        // A verifier whose clock runs CLOCK_SKEW behind still accepts.
        assert!(
            claims
                .prüfe_zeitfenster(jetzt() + ACCESS_TOKEN_LIFETIME - CLOCK_SKEW)
                .is_ok()
        );
    }

    #[test_log::test]
    fn iat_in_der_zukunft_wird_abgelehnt() {
        let claims = claims(
            jetzt() + Duration::minutes(5),
            jetzt() + ACCESS_TOKEN_LIFETIME,
        );
        assert!(matches!(
            claims.prüfe_zeitfenster(jetzt()),
            Err(KontoFehler::TokenNochNichtGültig)
        ));
    }

    #[test_log::test]
    fn iat_innerhalb_der_toleranz_wird_akzeptiert() {
        let claims = claims(jetzt() + CLOCK_SKEW, jetzt() + ACCESS_TOKEN_LIFETIME);
        assert!(claims.prüfe_zeitfenster(jetzt()).is_ok());
    }

    #[test_log::test]
    fn fenster_mit_exp_vor_iat_wird_abgelehnt() {
        let claims = claims(jetzt(), jetzt());
        assert!(matches!(
            claims.prüfe_zeitfenster(jetzt()),
            Err(KontoFehler::TokenUngültig)
        ));
    }

    #[test_log::test]
    fn herkunft_muss_exakt_stimmen() {
        let claims = claims(jetzt(), jetzt() + ACCESS_TOKEN_LIFETIME);
        assert!(
            claims
                .prüfe_herkunft("logandaim.at", "logandaim-api")
                .is_ok()
        );
        assert!(matches!(
            claims.prüfe_herkunft("logandaim.at.evil", "logandaim-api"),
            Err(KontoFehler::TokenHerkunftUngültig)
        ));
        assert!(matches!(
            claims.prüfe_herkunft("logandaim.at", "other"),
            Err(KontoFehler::TokenHerkunftUngültig)
        ));
    }
}
