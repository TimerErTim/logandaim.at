//! Inbound requests of the `konto` subdomain.
//!
//! Pure transport DTOs: they carry strings and let the domain validate. Normalisation of an
//! address, the Passwort length rule, and token parsing all happen in core, so the rules
//! stay in one place and this file cannot drift from them.

use logandaim_core::{
    ports::KundeSchlüssel,
    service::{
        Anmeldung, BenutzerLöschen, PasswortVergessen, PasswortWiederherstellen,
        RegistrierungAbschließen, RegistrierungStarten,
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

/// The client identity a throttle keys on. Never travels in a body: it is read from the
/// connection and the request headers, so a caller cannot choose its own budget.
#[derive(Debug, Clone)]
pub struct Kundenkennung(pub KundeSchlüssel);

impl RegistrierungStartAnfrage {
    pub fn in_use_case(self, kunde: KundeSchlüssel) -> RegistrierungStarten {
        RegistrierungStarten {
            email: self.e_mail_adresse,
            kunde,
        }
    }
}

impl RegistrierungAbschlussAnfrage {
    pub fn in_use_case(self, kunde: KundeSchlüssel) -> RegistrierungAbschließen {
        RegistrierungAbschließen {
            token: self.token,
            passwort: self.passwort,
            kunde,
        }
    }
}

impl AnmeldeAnfrage {
    pub fn in_use_case(self, kunde: KundeSchlüssel) -> Anmeldung {
        Anmeldung {
            email: self.e_mail_adresse,
            passwort: self.passwort,
            kunde,
        }
    }
}

impl PasswortVergessenAnfrage {
    pub fn in_use_case(self, kunde: KundeSchlüssel) -> PasswortVergessen {
        PasswortVergessen {
            email: self.e_mail_adresse,
            kunde,
        }
    }
}

impl PasswortWiederherstellenAnfrage {
    /// No throttle key: a recovery token is already single-use and short-lived, and
    /// throttling it per client would let one shared IP lock out a whole household.
    pub fn in_use_case(self) -> PasswortWiederherstellen {
        PasswortWiederherstellen {
            token: self.token,
            neues_passwort: self.neues_passwort,
        }
    }
}

impl BenutzerLöschungAnfrage {
    pub fn in_use_case(self) -> BenutzerLöschen {
        BenutzerLöschen {
            passwort: self.passwort,
            bestätigt: self.bestätigt,
        }
    }
}
