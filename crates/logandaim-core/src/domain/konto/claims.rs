//! Claim *semantics*, without any serialization.
//!
//! The JWT envelope (JSON, base64url, the JOSE header, `alg` pinning, `kid` comparison)
//! lives in `logandaim-api`. Core receives the already-parsed claim values and decides
//! whether they are acceptable: is the window open, does the issuer and audience match.
//! That split is why the serde derives live on the API-side twin and not here.

use chrono::{DateTime, Utc};

use super::{BenutzerId, CLOCK_SKEW, KontoFehler, SessionId, SigningKeyId};

/// The claims an access token carries, already deserialized by the API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessTokenClaims {
    /// The Benutzer the token acts for.
    pub sub: BenutzerId,
    /// The Anmelde-Sitzung whose revoked state decides acceptance.
    pub sid: SessionId,
    /// The signing key row that must be active.
    pub kid: SigningKeyId,
    pub iss: String,
    pub aud: String,
    pub iat: DateTime<Utc>,
    pub exp: DateTime<Utc>,
    /// Unique per token, so a replayed envelope is distinguishable in logs.
    pub jti: String,
}

impl AccessTokenClaims {
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
            kid: SigningKeyId::new("k1").unwrap(),
            iss: "logandaim.at".into(),
            aud: "logandaim-api".into(),
            iat,
            exp,
            jti: "jti".into(),
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
