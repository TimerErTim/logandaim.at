//! Outbound schemas of the `konto` subdomain.
//!
//! API mirrors of domain values, kept separate so the domain never learns about OpenAPI or
//! serde. The access token lives here and only here: it travels in the response body,
//! never in a cookie, so no cross-site request can read it (SPEC-konto AC-7a).

use uuid::Uuid;

use logandaim_core::domain;

use logandaim_core::domain::konto::Rolle as DomainBenutzerRolle;

use crate::schema::EmailAdresse;

/// `Benutzer` or `Systemadministrator`: the role a Benutzer holds on the whole instance.
///
/// A Systemadministrator may create and administer any Anlage, independently of its
/// per-Anlage roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum BenutzerRolle {
    Benutzer,
    Systemadministrator,
}

impl From<DomainBenutzerRolle> for BenutzerRolle {
    fn from(rolle: DomainBenutzerRolle) -> Self {
        match rolle {
            DomainBenutzerRolle::Benutzer => Self::Benutzer,
            DomainBenutzerRolle::Systemadministrator => Self::Systemadministrator,
        }
    }
}

/// The authenticated Benutzer, as `GET /benutzer` answers it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Benutzer {
    /// Identity of the Benutzer.
    pub id: Uuid,
    /// Its e-mail address, which is also the sign-in name.
    pub e_mail_adresse: EmailAdresse,
    /// Its role on the whole instance.
    pub rolle: BenutzerRolle,
}

/// The single account found by an exact e-mail lookup, for granting an Anlage role.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenutzerSuchtreffer {
    /// Identity of the matching Benutzer.
    pub id: Uuid,
    /// The address that matched.
    pub e_mail_adresse: EmailAdresse,
}

impl From<(domain::konto::Benutzer, domain::EmailAdresse)> for BenutzerSuchtreffer {
    fn from((benutzer, email): (domain::konto::Benutzer, domain::EmailAdresse)) -> Self {
        Self {
            id: benutzer.id().0,
            e_mail_adresse: email.into(),
        }
    }
}

impl From<(domain::konto::Benutzer, domain::EmailAdresse)> for Benutzer {
    fn from((benutzer, email): (domain::konto::Benutzer, domain::EmailAdresse)) -> Self {
        Self {
            id: benutzer.id().0,
            e_mail_adresse: email.into(),
            rolle: benutzer.rolle().into(),
        }
    }
}

/// What registration step 2, sign-in, and session renewal hand back.
///
/// The access token is in the body. The refresh token is not: it leaves as the `__Host-`
/// cookie, so a script that can read a JSON response still cannot read it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeueSession {
    /// The token to send as `Authorization: Bearer <token>`.
    pub access_token: String,
    /// Kept for the response builder; it never leaves in a body.
    pub refresh_token: String,
}
