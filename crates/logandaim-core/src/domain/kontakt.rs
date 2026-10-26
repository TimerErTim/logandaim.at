use std::convert::TryFrom;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("Ungültige E-Mail-Adresse: {0}")]
pub struct EmailAdresseValidierungsfehler(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAdresse(String);

/// Longest address accepted (RFC 5321 path limit).
const MAX_EMAIL_LAENGE: usize = 254;

impl EmailAdresse {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, EmailAdresseValidierungsfehler> {
        let s_ref = s.as_ref();
        if s_ref.contains('@') && s_ref.contains('.') {
            Ok(Self(s_ref.to_string()))
        } else {
            Err(EmailAdresseValidierungsfehler(s_ref.to_string()))
        }
    }

    /// The login identifier of a [`Konto`](crate::domain::konto::Konto): trimmed, lowercase,
    /// and unique across all Konten (SPEC-konto rule 2).
    ///
    /// Folding stops at case: dots and `+` suffixes stay, because they are different
    /// mailboxes on providers other than Gmail, and merging them would hand one person
    /// another person's account.
    pub fn normalisiert<S: AsRef<str>>(s: S) -> Result<Self, EmailAdresseValidierungsfehler> {
        let roh = s.as_ref();
        let getrimmt = roh.trim();
        let normalisiert = getrimmt.to_lowercase();
        if normalisiert.len() > MAX_EMAIL_LAENGE {
            return Err(EmailAdresseValidierungsfehler(roh.to_string()));
        }
        EmailAdresse::new(normalisiert)
    }
}

impl TryFrom<String> for EmailAdresse {
    type Error = EmailAdresseValidierungsfehler;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        EmailAdresse::new(s)
    }
}

impl TryFrom<&str> for EmailAdresse {
    type Error = EmailAdresseValidierungsfehler;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        EmailAdresse::new(s)
    }
}

impl AsRef<str> for EmailAdresse {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Error)]
#[error("Ungültige Mobilnummer: {0}")]
pub struct MobilnummerValidierungsfehler(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mobilnummer(String);

impl Mobilnummer {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, MobilnummerValidierungsfehler> {
        let s_ref = s.as_ref();
        let valid = {
            let body = if s_ref.starts_with('+') {
                &s_ref[1..]
            } else {
                s_ref
            };
            body.chars().all(|c| c.is_ascii_digit()) && body.len() >= 7 && body.len() <= 15
        };
        if valid {
            Ok(Self(s_ref.to_string()))
        } else {
            Err(MobilnummerValidierungsfehler(s_ref.to_string()))
        }
    }
}

impl TryFrom<String> for Mobilnummer {
    type Error = MobilnummerValidierungsfehler;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Mobilnummer::new(s)
    }
}

impl TryFrom<&str> for Mobilnummer {
    type Error = MobilnummerValidierungsfehler;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Mobilnummer::new(s)
    }
}

impl AsRef<str> for Mobilnummer {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn email_accepts_valid_address() {
        let email = EmailAdresse::new("anna@muster.de").unwrap();
        assert_eq!(email.as_ref(), "anna@muster.de");
    }

    #[test_log::test]
    fn email_rejects_missing_at() {
        assert!(EmailAdresse::new("anna.muster.de").is_err());
    }

    #[test_log::test]
    fn email_rejects_missing_dot() {
        assert!(EmailAdresse::new("anna@musterde").is_err());
    }

    #[test_log::test]
    fn normalisiert_trimmt_und_senkt() {
        let email = EmailAdresse::normalisiert("  Anna.Muster@Example.AT  ").unwrap();
        assert_eq!(email.as_ref(), "anna.muster@example.at");
    }

    #[test_log::test]
    fn normalisiert_faltet_weder_punkte_noch_plus_suffixe() {
        let mit_punkt = EmailAdresse::normalisiert("anna.muster@example.at").unwrap();
        let ohne_punkt = EmailAdresse::normalisiert("annamuster@example.at").unwrap();
        assert_ne!(mit_punkt, ohne_punkt);

        let mit_plus = EmailAdresse::normalisiert("anna+konto@example.at").unwrap();
        let ohne_plus = EmailAdresse::normalisiert("anna@example.at").unwrap();
        assert_ne!(mit_plus, ohne_plus);
    }

    #[test_log::test]
    fn normalisiert_ist_idempotent() {
        let einmal = EmailAdresse::normalisiert("Anna@Example.AT").unwrap();
        let zweimal =
            EmailAdresse::normalisiert(einmal.as_ref()).expect("a normalized address is valid");
        assert_eq!(einmal, zweimal);
    }

    #[test_log::test]
    fn normalisiert_lehnt_zu_lange_adresse_ab() {
        let lokal = "a".repeat(MAX_EMAIL_LAENGE);
        let adresse = format!("{lokal}@example.at");
        assert!(EmailAdresse::normalisiert(&adresse).is_err());
    }

    #[test_log::test]
    fn mobilnummer_accepts_digits() {
        let nummer = Mobilnummer::new("1234567890").unwrap();
        assert_eq!(nummer.as_ref(), "1234567890");
    }

    #[test_log::test]
    fn mobilnummer_accepts_plus_prefix() {
        let nummer = Mobilnummer::new("+431234567890").unwrap();
        assert_eq!(nummer.as_ref(), "+431234567890");
    }

    #[test_log::test]
    fn mobilnummer_rejects_too_short() {
        assert!(Mobilnummer::new("123456").is_err());
    }

    #[test_log::test]
    fn mobilnummer_rejects_non_digits() {
        assert!(Mobilnummer::new("+43 699 12345678").is_err());
        assert!(Mobilnummer::new("abc1234567").is_err());
    }
}
