//! Optional contact details of an `Anlage`.
//!
//! Every field is independently optional (SPEC-stammdaten rule 3, AC-1).

use crate::domain::{EmailAdresse, Mobilnummer};

#[derive(Debug, thiserror::Error)]
#[error("die Website-URL ist ungültig: {0}")]
pub struct WebsiteUrlFehler(String);

/// The protocol candidate of a URL text: everything before the first `:` that precedes any
/// path, query, or fragment. `None` means the text carries no protocol at all.
fn protokoll(roh: &str) -> Option<&str> {
    let ende = roh.find(['/', '?', '#']).unwrap_or(roh.len());
    roh[..ende].find(':').map(|position| &roh[..position])
}

/// A website reference, held as a fully qualified `http` or `https` URL.
///
/// Parsing prepends `https://` when the protocol is missing (`www.breitenstein.at`), and
/// refuses any other protocol as well as a host without a dot. The stored value is the
/// normalised URL text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WebsiteUrl {
    url: String,
}

impl WebsiteUrl {
    pub fn neu<S: AsRef<str>>(wert: S) -> Result<Self, WebsiteUrlFehler> {
        let roh = wert.as_ref().trim();
        if roh.is_empty() {
            return Err(WebsiteUrlFehler(roh.to_owned()));
        }

        let (url, rest) = match protokoll(roh) {
            // A dot in the candidate means a host with a port, not a protocol
            // (`www.breitenstein.at:8080`).
            Some(candidate) if candidate.contains('.') => (format!("https://{roh}"), roh),
            Some(candidate) => {
                if !matches!(candidate.to_ascii_lowercase().as_str(), "http" | "https") {
                    return Err(WebsiteUrlFehler(roh.to_owned()));
                }
                let rest = &roh[candidate.len() + 1..];
                let Some(rest) = rest.strip_prefix("//") else {
                    return Err(WebsiteUrlFehler(roh.to_owned()));
                };
                (roh.to_owned(), rest)
            }
            None => (format!("https://{roh}"), roh),
        };

        let host = rest
            .split(['/', '?', '#'])
            .next()
            .unwrap_or_default()
            .rsplit('@')
            .next()
            .unwrap_or_default();
        let host = host.split(':').next().unwrap_or_default();
        // Syntactically a domain: at least "some text.some text", no empty label.
        let labels: Vec<&str> = host.split('.').collect();
        if labels.len() < 2
            || labels
                .iter()
                .any(|label| label.is_empty() || label.chars().any(char::is_whitespace))
        {
            return Err(WebsiteUrlFehler(roh.to_owned()));
        }

        Ok(Self { url })
    }

    pub fn as_str(&self) -> &str {
        &self.url
    }
}

impl AsRef<str> for WebsiteUrl {
    fn as_ref(&self) -> &str {
        &self.url
    }
}

impl std::fmt::Display for WebsiteUrl {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.url)
    }
}

impl std::str::FromStr for WebsiteUrl {
    type Err = WebsiteUrlFehler;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::neu(value)
    }
}

/// Contact details of one Anlage. All fields may be omitted, individually.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AnlageKontakt {
    name: Option<String>,
    email: Option<EmailAdresse>,
    telefon: Option<Mobilnummer>,
    website: Option<WebsiteUrl>,
}

impl AnlageKontakt {
    pub fn neu(
        name: Option<impl AsRef<str>>,
        email: Option<EmailAdresse>,
        telefon: Option<Mobilnummer>,
        website: Option<WebsiteUrl>,
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

    pub fn website(&self) -> Option<&WebsiteUrl> {
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
    fn fehlendes_protokoll_wird_zu_https() {
        let url = WebsiteUrl::neu("www.breitenstein.at").unwrap();
        assert_eq!(url.as_str(), "https://www.breitenstein.at");
        assert_eq!(
            WebsiteUrl::neu("  breitenstein.at/verein ")
                .unwrap()
                .as_str(),
            "https://breitenstein.at/verein"
        );
    }

    #[test_log::test]
    fn http_und_https_bleiben_erhalten() {
        assert_eq!(
            WebsiteUrl::neu("http://breitenstein.at").unwrap().as_str(),
            "http://breitenstein.at"
        );
        assert_eq!(
            WebsiteUrl::neu("HTTPS://breitenstein.at").unwrap().as_str(),
            "HTTPS://breitenstein.at"
        );
    }

    #[test_log::test]
    fn fremdes_protokoll_wird_abgelehnt() {
        for wert in ["ht://www.breitenstein.at", "ftp://breitenstein.at"] {
            assert!(WebsiteUrl::neu(wert).is_err(), "{wert:?} muss scheitern");
        }
    }

    #[test_log::test]
    fn domain_muss_einen_punkt_enthalten() {
        for wert in ["breitenstein", "https://breitenstein", "https://", "a..b"] {
            assert!(WebsiteUrl::neu(wert).is_err(), "{wert:?} muss scheitern");
        }
        assert!(WebsiteUrl::neu("mailto:verein@breitenstein.at").is_err());
    }

    #[test_log::test]
    fn ein_port_gehoert_zum_host_nicht_zum_protokoll() {
        assert_eq!(
            WebsiteUrl::neu("www.breitenstein.at:8080/verein")
                .unwrap()
                .as_str(),
            "https://www.breitenstein.at:8080/verein"
        );
    }

    #[test_log::test]
    fn leere_website_wird_abgelehnt() {
        assert!(WebsiteUrl::neu("  ").is_err());
    }
}
