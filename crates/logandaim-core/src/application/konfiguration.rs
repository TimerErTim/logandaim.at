//! Deployment-supplied data the `konto` subdomain needs but must not invent.
//!
//! Everything here is *data*, not behaviour: what the access token claims name as issuer and
//! audience. The algorithms, the lifetimes, and the rules stay in `domain/konto` and are
//! never configurable (see `domain/konto/mod.rs`). The mailed links are not here either: the
//! frontend *origin* is deployment data and the routes are the product's, so the sending
//! adapter spells the URL (see [`crate::ports::Mail`]).

/// Under which identity access tokens are issued.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KontoKonfiguration {
    /// `iss` claim of every access token this deployment issues.
    pub issuer: String,
    /// `aud` claim of every access token this deployment issues.
    pub audience: String,
}

impl KontoKonfiguration {
    pub fn neu(issuer: impl Into<String>, audience: impl Into<String>) -> Self {
        Self {
            issuer: issuer.into(),
            audience: audience.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn die_konfiguration_hält_nur_die_claim_identität() {
        let konfiguration = KontoKonfiguration::neu("logandaim.at", "logandaim-api");
        assert_eq!(konfiguration.issuer, "logandaim.at");
        assert_eq!(konfiguration.audience, "logandaim-api");
    }
}
