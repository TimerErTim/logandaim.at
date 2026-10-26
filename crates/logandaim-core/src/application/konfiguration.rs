//! Deployment-supplied data the `konto` subdomain needs but must not invent.
//!
//! Everything here is *data*, not behaviour: where a mailed link points, what the token
//! claims name as issuer and audience. The algorithms, the lifetimes, and the rules stay
//! in `domain/konto` and are never configurable (see `domain/konto/mod.rs`).

/// Where the two mailed links point and under which identity access tokens are issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KontoKonfiguration {
    /// Frontend route that completes registration. The mailed link is this plus
    /// `?token=<SecretToken>`.
    pub bestaetigung_link: String,
    /// Frontend route that sets a new Passwort after a recovery request.
    pub wiederherstellung_link: String,
    /// `iss` claim of every access token this deployment issues.
    pub issuer: String,
    /// `aud` claim of every access token this deployment issues.
    pub audience: String,
}

impl KontoKonfiguration {
    pub fn neu(
        bestaetigung_link: impl Into<String>,
        wiederherstellung_link: impl Into<String>,
        issuer: impl Into<String>,
        audience: impl Into<String>,
    ) -> Self {
        Self {
            bestaetigung_link: bestaetigung_link.into(),
            wiederherstellung_link: wiederherstellung_link.into(),
            issuer: issuer.into(),
            audience: audience.into(),
        }
    }

    /// The link a confirmation mail carries. The token is a query parameter, because
    /// the frontend route exchanges it with the API itself (SPEC-konto Q-17).
    pub fn bestaetigung_url(&self, token: &str) -> String {
        format!("{}?token={token}", self.bestaetigung_link)
    }

    pub fn wiederherstellung_url(&self, token: &str) -> String {
        format!("{}?token={token}", self.wiederherstellung_link)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn konfiguration() -> KontoKonfiguration {
        KontoKonfiguration::neu(
            "https://logandaim.at/registrierung",
            "https://logandaim.at/passwort",
            "logandaim.at",
            "logandaim-api",
        )
    }

    #[test_log::test]
    fn links_tragen_das_token_als_query_parameter() {
        let konfiguration = konfiguration();
        assert_eq!(
            konfiguration.bestaetigung_url("abc"),
            "https://logandaim.at/registrierung?token=abc"
        );
        assert_eq!(
            konfiguration.wiederherstellung_url("abc"),
            "https://logandaim.at/passwort?token=abc"
        );
    }
}
