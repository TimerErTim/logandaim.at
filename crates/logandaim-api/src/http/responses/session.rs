use http::StatusCode;
use logandaim_core::{ResultReport, ThreadSafeError, domain::konto::REFRESH_TOKEN_LIFETIME};
use poem_openapi::{
    ApiResponse,
    payload::{Json, PlainText},
    types::ToJSON,
};

use crate::{
    errors::{HttpStatusMapping, InternalServerError, StructuredError, status_from_report},
    http::{
        auth::REFRESH_COOKIE,
        cookie::{CookieConfig, SameSite, clear_cookie_header, set_cookie_header},
        responses::NO_STORE,
    },
    schema::NeueSession,
};

fn refresh_cookie_config() -> CookieConfig {
    CookieConfig {
        path: Some("/".to_owned()),
        secure: true,
        http_only: true,
        same_site: SameSite::Lax,
        max_age: Some(REFRESH_TOKEN_LIFETIME.to_std().unwrap()),
        ..Default::default()
    }
}

#[derive(Debug, Clone, poem_openapi::Object)]
#[oai(rename_all = "camelCase")]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionResponseContent {
    pub access_token: String,
}

/// A session response: the access token in the body, the refresh token in the cookie.
///
/// Every variant carries `Cache-Control: no-store`. A 401 clears the unusable refresh
/// cookie; other client errors leave it alone, including a throttled renewal.
#[derive(ApiResponse)]
pub enum NewSessionResponse {
    #[oai(status = 200)]
    Ok(
        Json<SessionResponseContent>,
        #[oai(header = "Set-Cookie")] String,
        #[oai(header = "Cache-Control")] String,
    ),
    #[oai(status_range = "4XX")]
    ClientError(
        StatusCode,
        Json<StructuredError>,
        #[oai(header = "Set-Cookie")] Option<String>,
        #[oai(header = "Cache-Control")] String,
    ),
    #[oai(status = 500)]
    InternalError(
        PlainText<InternalServerError>,
        #[oai(header = "Cache-Control")] String,
    ),
}

impl<E> From<ResultReport<NeueSession, E>> for NewSessionResponse
where
    E: ThreadSafeError + HttpStatusMapping,
{
    fn from(ergebnis: ResultReport<NeueSession, E>) -> Self {
        match ergebnis {
            Ok(session) => Self::Ok(
                Json(SessionResponseContent {
                    access_token: session.access_token,
                }),
                set_cookie_header(
                    REFRESH_COOKIE,
                    &session.refresh_token,
                    refresh_cookie_config(),
                ),
                NO_STORE.to_owned(),
            ),
            Err(fehler) => {
                let status = status_from_report(&fehler);
                if status.is_server_error() {
                    return Self::InternalError(
                        PlainText(InternalServerError),
                        NO_STORE.to_owned(),
                    );
                }
                let cookie = (status == StatusCode::UNAUTHORIZED)
                    .then(|| clear_cookie_header(REFRESH_COOKIE, refresh_cookie_config()));
                Self::ClientError(status, Json(fehler.into()), cookie, NO_STORE.to_owned())
            }
        }
    }
}

/// A response that also *ends* a session, so it clears the cookie beside its body.
#[derive(ApiResponse)]
pub enum EndSessionResponse<T: ToJSON> {
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

impl<T: ToJSON, E: ThreadSafeError + HttpStatusMapping> From<ResultReport<T, E>>
    for EndSessionResponse<T>
{
    fn from(value: ResultReport<T, E>) -> Self {
        match value {
            Ok(wert) => Self::Ok(
                Json(wert),
                clear_cookie_header(REFRESH_COOKIE, refresh_cookie_config()),
                NO_STORE.to_owned(),
            ),
            Err(fehler) => {
                let status = status_from_report(&fehler);
                if status.is_server_error() {
                    return Self::InternalError(
                        PlainText(InternalServerError),
                        NO_STORE.to_owned(),
                    );
                }
                Self::ClientError(
                    status,
                    Json(fehler.into()),
                    clear_cookie_header(REFRESH_COOKIE, refresh_cookie_config()),
                    NO_STORE.to_owned(),
                )
            }
        }
    }
}
