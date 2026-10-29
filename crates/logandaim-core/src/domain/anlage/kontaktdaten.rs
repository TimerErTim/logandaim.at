//! Optional contact details of an `Anlage`.
//!
//! Every field is independently optional (SPEC-stammdaten rule 3, AC-1).

use crate::domain::{EmailAdresse, Mobilnummer};

#[derive(Debug, thiserror::Error)]
#[error("die Website ist ungültig: {0}")]
pub struct WebsiteFehler(String);

/// A non-empty website reference, stored as given.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Website(String);

impl Website {
    pub fn neu<S: AsRef<str>>(wert: S) -> Result<Self, WebsiteFehler> {
        let wert = wert.as_ref().trim();
        if wert.is_empty() {
            return Err(WebsiteFehler(wert.to_owned()));
        }
        Ok(Self(wert.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Contact details of one Anlage. All fields may be omitted, individually.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AnlageKontakt {
    name: Option<String>,
    email: Option<EmailAdresse>,
    telefon: Option<Mobilnummer>,
    website: Option<Website>,
}

impl AnlageKontakt {
    pub fn neu(
        name: Option<impl AsRef<str>>,
        email: Option<EmailAdresse>,
        telefon: Option<Mobilnummer>,
        website: Option<Website>,
    ) -> Self {
        let name = name
            .map(|name| name.as_ref().trim().to_owned())
            .filter(|name| !name.is_empty());
        Self {
            name,
            email,
            telefon,
            website,
        }
    }

    /// No contact details at all: the state every Anlage may start in.
    pub fn leer() -> Self {
        Self::default()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn email(&self) -> Option<&EmailAdresse> {
        self.email.as_ref()
    }

    pub fn telefon(&self) -> Option<&Mobilnummer> {
        self.telefon.as_ref()
    }

    pub fn website(&self) -> Option<&Website> {
        self.website.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn alle_felder_duerfen_fehlen() {
        let kontakt = AnlageKontakt::leer();
        assert_eq!(kontakt.name(), None);
        assert_eq!(kontakt.email(), None);
        assert_eq!(kontakt.telefon(), None);
        assert_eq!(kontakt.website(), None);
    }

    #[test_log::test]
    fn jedes_feld_ist_einzeln_optional() {
        let kontakt = AnlageKontakt::neu(
            Some("  "),
            Some(EmailAdresse::normalisiert("verein@example.at").unwrap()),
            None,
            None,
        );
        assert_eq!(kontakt.name(), None);
        assert!(kontakt.email().is_some());
        assert_eq!(kontakt.telefon(), None);
    }

    #[test_log::test]
    fn leere_website_wird_abgelehnt() {
        assert!(Website::neu("  ").is_err());
    }
}
