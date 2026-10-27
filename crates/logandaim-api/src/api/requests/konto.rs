//! Inbound requests of the `konto` subdomain: transport DTOs whose conversion into a use
//! case performs the domain validation once, at the edge. A conversion failure reuses the
//! use case's own `UngültigeAnfrage` variant, so no route invents a second error type.

use error_stack::Report;
use logandaim_core::{
    domain,
    ports::ClientKey,
    service::{
        Anmeldung, AnmeldungFehler, BenutzerLöschen, BenutzerLöschenFehler, PasswortVergessen,
        PasswortVergessenFehler, PasswortWiederherstellen, PasswortWiederherstellenFehler,
        RegistrierungAbschliessenFehler, RegistrierungAbschließen, RegistrierungStarten,
        RegistrierungStartenFehler,
    },
};

use crate::schema::EmailAdresse;

/// Registration step 1.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrierungStartRequest {
    pub e_mail_adresse: EmailAdresse,
}

/// Registration step 2.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrierungAbschlussRequest {
    pub token: String,
    pub passwort: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnmeldungRequest {
    pub e_mail_adresse: EmailAdresse,
    pub passwort: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswortVergessenRequest {
    pub e_mail_adresse: EmailAdresse,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswortWiederherstellenRequest {
    pub token: String,
    pub neues_passwort: String,
}

/// Deleting one's own Benutzer: the Passwort is re-entered, and the intent is explicit.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenutzerLöschungRequest {
    pub passwort: String,
    /// Must be `true`; a missing flag is a refusal, never a default.
    pub bestätigt: bool,
}

impl RegistrierungStartRequest {
    pub fn in_use_case(
        self,
        client_key: ClientKey,
    ) -> Result<RegistrierungStarten, Report<RegistrierungStartenFehler>> {
        let email = self
            .e_mail_adresse
            .try_into()
            .map_err(|_| Report::new(RegistrierungStartenFehler::UngültigeAnfrage))?;
        Ok(RegistrierungStarten { email, client_key })
    }
}

impl RegistrierungAbschlussRequest {
    pub fn in_use_case(
        self,
        client_key: ClientKey,
    ) -> Result<RegistrierungAbschließen, Report<RegistrierungAbschliessenFehler>> {
        let token = domain::konto::SecretToken::try_from(self.token)
            .map_err(|_| Report::new(RegistrierungAbschliessenFehler::UngültigeAnfrage))?;
        let passwort = domain::konto::KlartextPasswort::try_from(self.passwort)
            .map_err(|_| Report::new(RegistrierungAbschliessenFehler::UngültigeAnfrage))?;
        Ok(RegistrierungAbschließen {
            token,
            passwort,
            client_key,
        })
    }
}

impl AnmeldungRequest {
    pub fn in_use_case(self, client_key: ClientKey) -> Result<Anmeldung, Report<AnmeldungFehler>> {
        let email = self
            .e_mail_adresse
            .try_into()
            .map_err(|_| Report::new(AnmeldungFehler::ZugangsdatenFalsch))?;
        // A Passwort that breaks the length rule cannot be right, but it is also not a
        // reason to answer differently from a wrong one.
        let passwort = domain::konto::KlartextPasswort::try_from(self.passwort)
            .map_err(|_| Report::new(AnmeldungFehler::ZugangsdatenFalsch))?;
        Ok(Anmeldung {
            email,
            passwort,
            client_key,
        })
    }
}

impl PasswortVergessenRequest {
    pub fn in_use_case(
        self,
        client_key: ClientKey,
    ) -> Result<PasswortVergessen, Report<PasswortVergessenFehler>> {
        // A malformed address is answered like a free one: no address oracle.
        let email = self
            .e_mail_adresse
            .try_into()
            .map_err(|_| Report::new(PasswortVergessenFehler::UngültigeAnfrage))?;
        Ok(PasswortVergessen { email, client_key })
    }
}

impl PasswortWiederherstellenRequest {
    /// No throttle key: the token is single-use and short-lived, and a per-client limit
    /// would let one shared IP lock out a whole household.
    pub fn in_use_case(
        self,
    ) -> Result<PasswortWiederherstellen, Report<PasswortWiederherstellenFehler>> {
        let token = domain::konto::SecretToken::try_from(self.token)
            .map_err(|_| Report::new(PasswortWiederherstellenFehler::UngültigeAnfrage))?;
        let neues_passwort = domain::konto::KlartextPasswort::try_from(self.neues_passwort)
            .map_err(|_| Report::new(PasswortWiederherstellenFehler::UngültigeAnfrage))?;
        Ok(PasswortWiederherstellen {
            token,
            neues_passwort,
        })
    }
}

impl BenutzerLöschungRequest {
    pub fn in_use_case(self) -> Result<BenutzerLöschen, Report<BenutzerLöschenFehler>> {
        let passwort = domain::konto::KlartextPasswort::try_from(self.passwort)
            .map_err(|_| Report::new(BenutzerLöschenFehler::UngültigeAnfrage))?;
        Ok(BenutzerLöschen {
            passwort,
            bestätigt: self.bestätigt,
        })
    }
}
