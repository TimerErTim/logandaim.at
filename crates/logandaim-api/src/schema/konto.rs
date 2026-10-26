//! Outbound schemas of the `konto` subdomain.
//!
//! API mirrors of domain values, kept separate so the domain never learns about OpenAPI or
//! serde. The access token lives here and only here: it travels in the response body,
//! never in a cookie, so no cross-site request can read it (SPEC-konto AC-7a).

use uuid::Uuid;

use logandaim_core::domain::konto::{Benutzer, Rolle};

/// Authority of a Benutzer, as the API spells it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SchemaRolle {
    Benutzer,
    Systemadministrator,
}

impl From<Rolle> for SchemaRolle {
    fn from(rolle: Rolle) -> Self {
        match rolle {
            Rolle::Benutzer => Self::Benutzer,
            Rolle::Systemadministrator => Self::Systemadministrator,
        }
    }
}

/// One Benutzer. It carries no address: the `Konto` owns the address, and only the
/// caller's own `Konto` is ever disclosed (see [`BenutzerAntwort`]).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SchemaBenutzer {
    pub id: Uuid,
    pub rolle: SchemaRolle,
}

impl From<Benutzer> for SchemaBenutzer {
    fn from(benutzer: Benutzer) -> Self {
        Self {
            id: benutzer.id().0,
            rolle: benutzer.rolle().into(),
        }
    }
}

/// The logged-in Benutzer as `GET /benutzer` answers it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct BenutzerAntwort {
    pub id: Uuid,
    pub e_mail_adresse: String,
    pub rolle: SchemaRolle,
}

/// What registration step 2, login, and renewal hand back.
///
/// The access token is in the body; the refresh token is *not* — it leaves as the
/// `__Host-` cookie, so a script that can read a JSON response still cannot read it, and
/// nothing in the body is ever stored by a proxy (`Cache-Control: no-store`).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct AnmeldeAntwort {
    pub access_token: String,
    pub benutzer: SchemaBenutzer,
}

/// Acknowledgement of a request that has nothing to return.
///
/// A bare `204` would be indistinguishable from a proxy's empty answer, and a caller
/// cannot tell success from a dropped connection; an explicit body is worth the two bytes.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct OkAntwort {
    pub ok: bool,
}
