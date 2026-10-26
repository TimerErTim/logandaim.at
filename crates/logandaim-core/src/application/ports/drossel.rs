use std::net::IpAddr;

use chrono::{DateTime, Duration, Utc};

use crate::application::ResultReport;

/// What a client is trying to do, so each action has its own budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DrosselAktion {
    /// Failed logins: 20 per 15 minutes (SPEC-konto rule 5, AC-15).
    Anmeldung,
    /// Registration step 1: 5 per hour (rule 5, AC-16).
    RegistrierungStart,
    /// Registration step 2: 5 per hour (rule 5, AC-16).
    RegistrierungAbschluss,
    /// Password recovery request: 5 per hour (rule 5, AC-16).
    Wiederherstellung,
    /// Session renewal: 5 per hour (rule 5, AC-16).
    Erneuerung,
}

impl DrosselAktion {
    /// Failed attempts allowed inside one window.
    pub fn schwelle(self) -> u32 {
        match self {
            Self::Anmeldung => 20,
            Self::RegistrierungStart
            | Self::RegistrierungAbschluss
            | Self::Wiederherstellung
            | Self::Erneuerung => 5,
        }
    }

    /// Length of the window the threshold applies to.
    pub fn fenster(self) -> Duration {
        match self {
            Self::Anmeldung => Duration::minutes(15),
            Self::RegistrierungStart
            | Self::RegistrierungAbschluss
            | Self::Wiederherstellung
            | Self::Erneuerung => Duration::hours(1),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Anmeldung => "anmeldung",
            Self::RegistrierungStart => "registrierung-start",
            Self::RegistrierungAbschluss => "registrierung-abschluss",
            Self::Wiederherstellung => "wiederherstellung",
            Self::Erneuerung => "erneuerung",
        }
    }
}

/// Who is being throttled: one client, identified by its IP and a client fingerprint.
///
/// Never an E-Mail-Adresse. A per-address limit would let anyone lock an owner out by
/// guessing their address (SPEC-konto rule 5, Q-16).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KundeSchlüssel {
    pub ip: IpAddr,
    pub fingerprint: String,
}

impl KundeSchlüssel {
    pub fn neu(ip: IpAddr, fingerprint: impl Into<String>) -> Self {
        Self {
            ip,
            fingerprint: fingerprint.into(),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum DrosselFehler {
    #[error("zu viele Versuche; bitte später erneut versuchen")]
    ZuVieleVersuche,
    #[error("die Drossel ist nicht erreichbar")]
    NichtVerfügbar,
}

/// Client-scoped rate limiting. An infrastructure port, not domain behaviour: the
/// thresholds above are the product rule, the store is an adapter choice.
#[async_trait::async_trait]
pub trait Drossel: Send + Sync {
    /// Count one attempt. `Err(ZuVieleVersuche)` once the budget is spent; the caller
    /// answers 429 and does the work only on `Ok`.
    async fn prüfe_und_zähle(
        &self,
        kunde: &KundeSchlüssel,
        aktion: DrosselAktion,
        jetzt: DateTime<Utc>,
    ) -> ResultReport<(), DrosselFehler>;

    /// Forget one action's counter for this client. Called on a successful login, so an
    /// owner who mistyped is not punished for the rest of the window (rule 5).
    async fn zurücksetzen(
        &self,
        kunde: &KundeSchlüssel,
        aktion: DrosselAktion,
    ) -> ResultReport<(), DrosselFehler>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn schwellen_entsprechen_der_spezifikation() {
        assert_eq!(DrosselAktion::Anmeldung.schwelle(), 20);
        assert_eq!(DrosselAktion::Anmeldung.fenster(), Duration::minutes(15));
        for aktion in [
            DrosselAktion::RegistrierungStart,
            DrosselAktion::RegistrierungAbschluss,
            DrosselAktion::Wiederherstellung,
            DrosselAktion::Erneuerung,
        ] {
            assert_eq!(aktion.schwelle(), 5);
            assert_eq!(aktion.fenster(), Duration::hours(1));
        }
    }
}
