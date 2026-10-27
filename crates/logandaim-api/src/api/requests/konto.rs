//! Inbound requests of the `konto` subdomain.
//!
//! Pure transport DTOs: they carry strings and the conversion into a use case performs the
//! domain validation once, at the edge. Normalisation of an address, the Passwort length
//! rule, and token parsing all happen in core, so the rules stay in one place and this file
//! cannot drift from them.
//!
//! A conversion failure is not a use-case refusal: the use case never ran. `in_use_case`
//! therefore returns the *use case's* `UngültigeAnfrage` variant, so the caller still sees
//! one 400 and no route has to invent a second error type.

use error_stack::Report;
use logandaim_core::{
    domain::{
        EmailAdresse,
        konto::{KlartextPasswort, SecretToken},
    },
    ports::ClientKey,
    service::{
        Anmeldung, AnmeldungFehler, BenutzerLöschen, BenutzerLöschenFehler, PasswortVergessen,
        PasswortVergessenFehler, PasswortWiederherstellen, PasswortWiederherstellenFehler,
        RegistrierungAbschliessenFehler, RegistrierungAbschließen, RegistrierungStarten,
        RegistrierungStartenFehler,
    },
};

/// Registration step 1.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct RegistrierungStartAnfrage {
    pub e_mail_adresse: String,
}

/// Registration step 2.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct RegistrierungAbschlussAnfrage {
    pub token: String,
    pub passwort: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct AnmeldeAnfrage {
    pub e_mail_adresse: String,
    pub passwort: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PasswortVergessenAnfrage {
    pub e_mail_adresse: String,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct PasswortWiederherstellenAnfrage {
    pub token: String,
    pub neues_passwort: String,
}

/// Deleting one's own Benutzer: the Passwort is re-entered, and the intent is explicit.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct BenutzerLöschungAnfrage {
    pub passwort: String,
    /// Must be `true`; a missing flag is a refusal, never a default.
    pub bestätigt: bool,
}

impl RegistrierungStartAnfrage {
    pub fn in_use_case(
        self,
        client_key: ClientKey,
    ) -> Result<RegistrierungStarten, Report<RegistrierungStartenFehler>> {
        let email = EmailAdresse::normalisiert(&self.e_mail_adresse)
            .map_err(|_| Report::new(RegistrierungStartenFehler::UngültigeAnfrage))?;
        Ok(RegistrierungStarten { email, client_key })
    }
}

impl RegistrierungAbschlussAnfrage {
    pub fn in_use_case(
        self,
        client_key: ClientKey,
    ) -> Result<RegistrierungAbschließen, Report<RegistrierungAbschliessenFehler>> {
        let token = SecretToken::try_from(self.token)
            .map_err(|_| Report::new(RegistrierungAbschliessenFehler::UngültigeAnfrage))?;
        let passwort = KlartextPasswort::try_from(self.passwort)
            .map_err(|_| Report::new(RegistrierungAbschliessenFehler::UngültigeAnfrage))?;
        Ok(RegistrierungAbschließen {
            token,
            passwort,
            client_key,
        })
    }
}

impl AnmeldeAnfrage {
    pub fn in_use_case(self, client_key: ClientKey) -> Result<Anmeldung, Report<AnmeldungFehler>> {
        let email = EmailAdresse::normalisiert(&self.e_mail_adresse)
            .map_err(|_| Report::new(AnmeldungFehler::ZugangsdatenFalsch))?;
        // A Passwort that breaks the length rule cannot be right, but it is also not a
        // reason to answer differently from a wrong one.
        let passwort = KlartextPasswort::try_from(self.passwort)
            .map_err(|_| Report::new(AnmeldungFehler::ZugangsdatenFalsch))?;
        Ok(Anmeldung {
            email,
            passwort,
            client_key,
        })
    }
}

impl PasswortVergessenAnfrage {
    pub fn in_use_case(
        self,
        client_key: ClientKey,
    ) -> Result<PasswortVergessen, Report<PasswortVergessenFehler>> {
        // A malformed address is answered like a free one: the endpoint must not become an
        // address oracle, and `PasswortVergessenFehler` has no "bad request" variant.
        let email = EmailAdresse::normalisiert(&self.e_mail_adresse)
            .map_err(|_| Report::new(PasswortVergessenFehler::UngültigeAnfrage))?;
        Ok(PasswortVergessen { email, client_key })
    }
}

impl PasswortWiederherstellenAnfrage {
    /// No throttle key: a recovery token is already single-use and short-lived, and
    /// throttling it per client would let one shared IP lock out a whole household.
    pub fn in_use_case(
        self,
    ) -> Result<PasswortWiederherstellen, Report<PasswortWiederherstellenFehler>> {
        let token = SecretToken::try_from(self.token)
            .map_err(|_| Report::new(PasswortWiederherstellenFehler::UngültigeAnfrage))?;
        let neues_passwort = KlartextPasswort::try_from(self.neues_passwort)
            .map_err(|_| Report::new(PasswortWiederherstellenFehler::UngültigeAnfrage))?;
        Ok(PasswortWiederherstellen {
            token,
            neues_passwort,
        })
    }
}

impl BenutzerLöschungAnfrage {
    pub fn in_use_case(self) -> Result<BenutzerLöschen, Report<BenutzerLöschenFehler>> {
        let passwort = KlartextPasswort::try_from(self.passwort)
            .map_err(|_| Report::new(BenutzerLöschenFehler::UngültigeAnfrage))?;
        Ok(BenutzerLöschen {
            passwort,
            bestätigt: self.bestätigt,
        })
    }
}
