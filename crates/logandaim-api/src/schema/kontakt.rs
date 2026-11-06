use logandaim_core::domain;

/// E-Mail-Adresse
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::NewType))]
#[cfg_attr(feature = "openapi", oai(example))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct EmailAdresse(String);

#[cfg(feature = "openapi")]
impl poem_openapi::types::Example for EmailAdresse {
    fn example() -> Self {
        Self("test@example.com".to_string())
    }
}

impl From<domain::EmailAdresse> for EmailAdresse {
    fn from(email: domain::EmailAdresse) -> Self {
        Self(email.as_ref().to_string())
    }
}

impl TryFrom<EmailAdresse> for domain::EmailAdresse {
    type Error = domain::kontakt::EmailAdresseValidierungsfehler;
    fn try_from(email: EmailAdresse) -> Result<Self, Self::Error> {
        domain::EmailAdresse::normalisiert(email.0)
    }
}
