use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Identity of one Benutzer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BenutzerId(pub Uuid);

/// Account-level authority of a Benutzer.
///
/// Everything inside one Anlage is an `AnlageVerwaltung` and lives in the Anlage
/// aggregate, not here (SPEC-konto rules 16, 17).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Rolle {
    /// Every registered person.
    #[default]
    Benutzer,
    /// Spans the product; provisioned out of band, never by an API path.
    Systemadministrator,
}

impl Rolle {
    /// Single stored spelling, shared by every adapter.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Benutzer => "Benutzer",
            Self::Systemadministrator => "Systemadministrator",
        }
    }
}

impl std::str::FromStr for Rolle {
    type Err = KontoFehler;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Benutzer" => Ok(Self::Benutzer),
            "Systemadministrator" => Ok(Self::Systemadministrator),
            _ => Err(KontoFehler::RolleUnbekannt),
        }
    }
}

use super::KontoFehler;

/// Creation input: everything of a Benutzer except its identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeuerBenutzer {
    rolle: Rolle,
    erstellt_am: DateTime<Utc>,
}

impl NeuerBenutzer {
    pub fn neu(rolle: Rolle, erstellt_am: DateTime<Utc>) -> Self {
        Self { rolle, erstellt_am }
    }

    /// The only role a registration may produce (rule 9: no self-service elevation).
    pub fn registrierung(erstellt_am: DateTime<Utc>) -> Self {
        Self::neu(Rolle::Benutzer, erstellt_am)
    }

    pub fn rolle(&self) -> Rolle {
        self.rolle
    }

    pub fn erstellt_am(&self) -> DateTime<Utc> {
        self.erstellt_am
    }
}

/// The logical identity the product knows: one person's account.
///
/// Created only by a completed registration (SPEC-konto rule 3), so there is no
/// unconfirmed state to represent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Benutzer {
    id: BenutzerId,
    rolle: Rolle,
    erstellt_am: DateTime<Utc>,
}

impl Benutzer {
    /// Fresh Benutzer with a caller-provided identity.
    pub fn neu(id: BenutzerId, neu: NeuerBenutzer) -> Self {
        Self {
            id,
            rolle: neu.rolle,
            erstellt_am: neu.erstellt_am,
        }
    }

    /// Rebuild a persisted Benutzer.
    pub fn from_parts(id: BenutzerId, rolle: Rolle, erstellt_am: DateTime<Utc>) -> Self {
        Self {
            id,
            rolle,
            erstellt_am,
        }
    }

    pub fn id(&self) -> BenutzerId {
        self.id
    }

    pub fn rolle(&self) -> Rolle {
        self.rolle
    }

    pub fn erstellt_am(&self) -> DateTime<Utc> {
        self.erstellt_am
    }

    pub fn ist_systemadministrator(&self) -> bool {
        self.rolle == Rolle::Systemadministrator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn jetzt() -> DateTime<Utc> {
        DateTime::from_timestamp(1_700_000_000, 0).unwrap()
    }

    #[test_log::test]
    fn registrierung_ist_immer_ein_benutzer() {
        let benutzer = Benutzer::neu(
            BenutzerId(Uuid::nil()),
            NeuerBenutzer::registrierung(jetzt()),
        );
        assert_eq!(benutzer.rolle(), Rolle::Benutzer);
        assert!(!benutzer.ist_systemadministrator());
    }

    #[test_log::test]
    fn rolle_roundtrip() {
        for rolle in [Rolle::Benutzer, Rolle::Systemadministrator] {
            assert_eq!(rolle.as_str().parse::<Rolle>().unwrap(), rolle);
        }
    }

    #[test_log::test]
    fn unbekannte_rolle_wird_abgelehnt() {
        let ergebnis = "Administrator".parse::<Rolle>();
        assert!(matches!(ergebnis, Err(KontoFehler::RolleUnbekannt)));
    }

    #[test_log::test]
    fn from_parts_roundtrip() {
        let neu = NeuerBenutzer::neu(Rolle::Systemadministrator, jetzt());
        let original = Benutzer::neu(BenutzerId(Uuid::nil()), neu);
        let geladen = Benutzer::from_parts(original.id(), original.rolle(), original.erstellt_am());
        assert_eq!(original, geladen);
        assert!(geladen.ist_systemadministrator());
    }
}
