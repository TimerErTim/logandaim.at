//! Outbound schemas of the `konto` subdomain.
//!
//! API mirrors of domain values, kept separate so the domain never learns about OpenAPI or
//! serde. The access token lives here and only here: it travels in the response body,
//! never in a cookie, so no cross-site request can read it (SPEC-konto AC-7a).

use uuid::Uuid;

use logandaim_core::domain::konto::Benutzer;

use crate::schema::EmailAdresse;

/// One Benutzer. It carries no address: the `Konto` owns the address, and only the
/// caller's own `Konto` is ever disclosed (see [`BenutzerAntwort`]).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaBenutzer {
    pub id: Uuid,
}

impl From<Benutzer> for SchemaBenutzer {
    fn from(benutzer: Benutzer) -> Self {
        Self {
            id: benutzer.id().0,
        }
    }
}

/// The logged-in Benutzer as `GET /benutzer` answers it.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BenutzerAntwort {
    pub id: Uuid,
    pub e_mail_adresse: EmailAdresse,
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
pub struct AnmeldeAntwort {
    pub access_token: String,
    pub benutzer: SchemaBenutzer,
}
