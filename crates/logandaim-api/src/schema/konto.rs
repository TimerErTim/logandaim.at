//! Outbound schemas of the `konto` subdomain.
//!
//! API mirrors of domain values, kept separate so the domain never learns about OpenAPI or
//! serde. The access token lives here and only here: it travels in the response body,
//! never in a cookie, so no cross-site request can read it (SPEC-konto AC-7a).

use uuid::Uuid;

use logandaim_core::domain;

use logandaim_core::domain::konto::Rolle as DomainBenutzerRolle;

use crate::schema::EmailAdresse;

/// Account-level role of a Benutzer.
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

/// The logged-in Benutzer as `GET /benutzer` answers it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Benutzer {
    pub id: Uuid,
    pub e_mail_adresse: EmailAdresse,
    pub rolle: BenutzerRolle,
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

/// What registration step 2, login, and renewal hand back.
///
/// The access token is in the body; the refresh token is *not* — it leaves as the
/// `__Host-` cookie, so a script that can read a JSON response still cannot read it, and
/// nothing in the body is ever stored by a proxy (`Cache-Control: no-store`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NeueSession {
    pub access_token: String,
    pub refresh_token: String,
}
