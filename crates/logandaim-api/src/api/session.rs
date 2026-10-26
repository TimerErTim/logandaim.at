//! Response types that carry `Set-Cookie` and `Cache-Control`.
//!
//! OpenAPI has no first-class cookie payload, so the header is a variant field. That is
//! also the guarantee AC-7a needs: the refresh token is written by exactly one builder
//! (`auth::cookie`), and the success variant of a session response has no field that could
//! hold it accidentally.
//!
//! Two rules are encoded here rather than left to each route:
//!
//! - **`Set-Cookie` is present on every variant of a session response**, set or cleared. A
//!   response that fails to touch the cookie leaves the previous one in the jar.
//! - **`Cache-Control: no-store` on every variant**, including errors: an error body can
//!   echo a token, and a cache keyed on the URL would serve it to the next caller.

use http::StatusCode;
use logandaim_core::ResultReport;
use logandaim_core::service::KontoDienstFehler;
use poem_openapi::{
    ApiResponse,
    payload::{Json, PlainText},
};

use crate::{
    api::AusgestellteSitzung,
    auth::cookie,
    errors::{InternalServerError, StructuredError, status_from_report},
    schema::AnmeldeAntwort,
};

/// Never store an auth response.
const NO_STORE: &str = "no-store";

/// A session response: the access token in the body, the refresh token in the cookie.
///
/// A failure clears the cookie, because a failed renewal must not leave the previous token
/// usable in the jar; every variant carries `Cache-Control: no-store`.
#[derive(ApiResponse)]
pub enum SitzungAntwort {
    #[oai(status = 200)]
    Ok(
        Json<AnmeldeAntwort>,
        #[oai(header = "Set-Cookie")] String,
        #[oai(header = "Cache-Control")] String,
    ),
    #[oai(status_range = "4XX")]
    ClientError(
        StatusCode,
        Json<StructuredError>,
        #[oai(header = "Set-Cookie")] String,
        #[oai(header = "Cache-Control")] String,
    ),
    #[oai(status = 500)]
    InternalError(
        PlainText<InternalServerError>,
        #[oai(header = "Cache-Control")] String,
    ),
}

impl SitzungAntwort {
    pub fn neu(ergebnis: ResultReport<AusgestellteSitzung, KontoDienstFehler>) -> Self {
        match ergebnis {
            Ok(ausgestellt) => Self::Ok(
                Json(ausgestellt.antwort),
                cookie::setzen_raw(&ausgestellt.refresh_token),
                NO_STORE.to_owned(),
            ),
            Err(fehler) => {
                let status = status_from_report(&fehler);
                let gelöscht = cookie::löschen();
                if status.is_server_error() {
                    return Self::InternalError(
                        PlainText(InternalServerError),
                        NO_STORE.to_owned(),
                    );
                }
                Self::ClientError(status, Json(fehler.into()), gelöscht, NO_STORE.to_owned())
            }
        }
    }
}

/// A plain answer that does not touch the cookie (registration step 1, recovery request).
#[derive(ApiResponse)]
pub enum KontoAntwort<T: poem_openapi::types::ToJSON> {
    #[oai(status = 200)]
    Ok(Json<T>, #[oai(header = "Cache-Control")] String),
    #[oai(status_range = "4XX")]
    ClientError(
        StatusCode,
        Json<StructuredError>,
        #[oai(header = "Cache-Control")] String,
    ),
    #[oai(status = 500)]
    InternalError(
        PlainText<InternalServerError>,
        #[oai(header = "Cache-Control")] String,
    ),
}

impl<T: poem_openapi::types::ToJSON> From<ResultReport<T, KontoDienstFehler>> for KontoAntwort<T> {
    fn from(ergebnis: ResultReport<T, KontoDienstFehler>) -> Self {
        match ergebnis {
            Ok(wert) => Self::Ok(Json(wert), NO_STORE.to_owned()),
            Err(fehler) => {
                let status = status_from_report(&fehler);
                if status.is_server_error() {
                    return Self::InternalError(
                        PlainText(InternalServerError),
                        NO_STORE.to_owned(),
                    );
                }
                Self::ClientError(status, Json(fehler.into()), NO_STORE.to_owned())
            }
        }
    }
}

/// A response that also *ends* a session, so it clears the cookie beside its body.
///
/// Logout, failed renewal, recovery, and deletion all end a session: the cookie must not
/// survive any of them.
#[derive(ApiResponse)]
pub enum BeendendeAntwort<T: poem_openapi::types::ToJSON> {
    #[oai(status = 200)]
    Ok(
        Json<T>,
        #[oai(header = "Set-Cookie")] String,
        #[oai(header = "Cache-Control")] String,
    ),
    #[oai(status_range = "4XX")]
    ClientError(
        StatusCode,
        Json<StructuredError>,
        #[oai(header = "Set-Cookie")] String,
        #[oai(header = "Cache-Control")] String,
    ),
    #[oai(status = 500)]
    InternalError(
        PlainText<InternalServerError>,
        #[oai(header = "Cache-Control")] String,
    ),
}

impl<T: poem_openapi::types::ToJSON> BeendendeAntwort<T> {
    pub fn neu(ergebnis: ResultReport<T, KontoDienstFehler>) -> Self {
        let gelöscht = cookie::löschen();
        match ergebnis {
            Ok(wert) => Self::Ok(Json(wert), gelöscht, NO_STORE.to_owned()),
            Err(fehler) => {
                let status = status_from_report(&fehler);
                if status.is_server_error() {
                    return Self::InternalError(
                        PlainText(InternalServerError),
                        NO_STORE.to_owned(),
                    );
                }
                Self::ClientError(status, Json(fehler.into()), gelöscht, NO_STORE.to_owned())
            }
        }
    }
}
