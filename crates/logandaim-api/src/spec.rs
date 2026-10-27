//! The HTTP edge: one thin `#[OpenApi]` wrapper over [`LogandaimAppApi`].
//!
//! Everything HTTP-specific lives here and only here — response envelopes with their
//! headers, the cookie, the client key — so the facade in `api/` stays free of `poem` and
//! the library compiles without the `openapi` feature. Handlers take native `poem_openapi`
//! extractors, never a raw request.
//!
//! Three things every handler enforces and a caller cannot opt out of:
//!
//! - **The refresh token never appears in a body.** It is written to the `__Host-` cookie
//!   by the response builder; the access token travels in the body only (AC-7a).
//! - **Every auth response is `Cache-Control: no-store`.**
//! - **Protected handlers answer 401 instead of running with `Anonymous`.**

use std::net::{IpAddr, Ipv4Addr};

use error_stack::Report;
use futures::StreamExt;
use http::StatusCode;
use logandaim_core::{
    App, ResultReport, ThreadSafeError,
    ports::{ClientKey, ObjectStream},
    service::{
        AbmeldungFehler, BenutzerAnzeigenFehler, BenutzerLöschenFehler, ErneuerungFehler,
        PasswortWiederherstellenFehler,
    },
};
use poem::{Body, IntoResponse, Response, web::RealIp};
use poem_openapi::{
    ApiResponse, OpenApi, OpenApiService, ServerObject,
    param::{Cookie, Header, Query},
    payload::{Json, Payload, PlainText},
    registry::{MetaSchema, MetaSchemaRef},
    types::ToJSON,
};
use uuid::Uuid;

use crate::{
    api::{
        AusgestellteSession, LogandaimAppApi,
        requests::{
            AnmeldungRequest, BenutzerLöschungRequest, PasswortVergessenRequest,
            PasswortWiederherstellenRequest, RegistrierungAbschlussRequest,
            RegistrierungStartRequest, SchussErstellungRequest, WertungRequest,
        },
    },
    auth::{cookie, kunde},
    errors::{HttpStatusMapping, InternalServerError, StructuredError, status_from_report},
    schema::{AnmeldeAntwort, BenutzerAntwort, OkAntwort, Schuss, Wertung},
};

/// Never store an auth response.
const NO_STORE: &str = "no-store";

pub struct LogandaimApiSpec {
    app_api: LogandaimAppApi,
}

impl LogandaimApiSpec {
    pub fn new(app_api: LogandaimAppApi) -> Self {
        Self { app_api }
    }
}

impl From<LogandaimAppApi> for LogandaimApiSpec {
    fn from(app_api: LogandaimAppApi) -> Self {
        Self::new(app_api)
    }
}

impl From<App> for LogandaimApiSpec {
    fn from(app: App) -> Self {
        Self::new(LogandaimAppApi::new(app))
    }
}

// ------------------------------------------------------------------ responses

#[derive(ApiResponse)]
pub enum TypicalJsonResponse<T: ToJSON> {
    #[oai(status = 200)]
    Ok(Json<T>),
    #[oai(status_range = "4XX")]
    ClientError(StatusCode, Json<StructuredError>),
    #[oai(status = 500)]
    InternalError(PlainText<InternalServerError>),
}

impl<T: ToJSON, C: ThreadSafeError + HttpStatusMapping> From<Result<T, Report<C>>>
    for TypicalJsonResponse<T>
{
    fn from(result: Result<T, Report<C>>) -> Self {
        match result {
            Ok(value) => TypicalJsonResponse::Ok(Json(value)),
            Err(error) => {
                let status = status_from_report(&error);
                if status.is_server_error() {
                    return TypicalJsonResponse::InternalError(PlainText(InternalServerError));
                }
                TypicalJsonResponse::ClientError(status, Json(error.into()))
            }
        }
    }
}

/// Streaming binary HTTP response for OpenAPI (poem `ApiResponse` needs `Payload`).
pub struct StreamBody(Response);

impl Payload for StreamBody {
    const CONTENT_TYPE: &'static str = "application/pdf";

    fn check_content_type(_content_type: &str) -> bool {
        true
    }

    fn schema_ref() -> MetaSchemaRef {
        MetaSchemaRef::Inline(Box::new(MetaSchema {
            format: Some("binary"),
            ..MetaSchema::new("string")
        }))
    }
}

impl IntoResponse for StreamBody {
    fn into_response(self) -> Response {
        self.0
    }
}

#[derive(ApiResponse)]
pub enum StreamBinaryResponse {
    #[oai(status = 200, content_type = "application/pdf")]
    Ok(StreamBody),
    #[oai(status = 404)]
    NotFound(Json<StructuredError>),
    #[oai(status_range = "4XX")]
    ClientError(StatusCode, Json<StructuredError>),
    #[oai(status = 500)]
    InternalError(PlainText<InternalServerError>),
}

impl<C: ThreadSafeError + HttpStatusMapping> From<Result<ObjectStream, Report<C>>>
    for StreamBinaryResponse
{
    fn from(result: Result<ObjectStream, Report<C>>) -> Self {
        match result {
            Ok(stream) => {
                let body = Body::from_bytes_stream(
                    stream.map(|chunk| chunk.map_err(|err| std::io::Error::other(err.to_string()))),
                );
                StreamBinaryResponse::Ok(StreamBody(
                    Response::builder()
                        .content_type("application/pdf")
                        .body(body),
                ))
            }
            Err(error) => {
                let status = status_from_report(&error);
                if status == StatusCode::NOT_FOUND {
                    return StreamBinaryResponse::NotFound(Json(error.into()));
                }
                if status.is_server_error() {
                    return StreamBinaryResponse::InternalError(PlainText(InternalServerError));
                }
                StreamBinaryResponse::ClientError(status, Json(error.into()))
            }
        }
    }
}

/// A session response: the access token in the body, the refresh token in the cookie.
///
/// Every variant touches the cookie and carries `Cache-Control: no-store`. A failure
/// clears it, because a failed renewal must not leave the previous token usable.
#[derive(ApiResponse)]
pub enum SessionAntwort {
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

impl SessionAntwort {
    pub fn neu<E>(ergebnis: ResultReport<AusgestellteSession, E>) -> Self
    where
        E: ThreadSafeError + HttpStatusMapping,
    {
        match ergebnis {
            Ok(ausgestellt) => Self::Ok(
                Json(ausgestellt.antwort),
                cookie::setzen_raw(&ausgestellt.refresh_token),
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
                    cookie::löschen(),
                    NO_STORE.to_owned(),
                )
            }
        }
    }
}

/// A plain answer that does not touch the cookie (registration step 1, recovery request).
#[derive(ApiResponse)]
pub enum KontoAntwort<T: ToJSON> {
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

impl<T, E> From<Result<T, Report<E>>> for KontoAntwort<T>
where
    T: ToJSON,
    E: ThreadSafeError + HttpStatusMapping,
{
    fn from(ergebnis: Result<T, Report<E>>) -> Self {
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
#[derive(ApiResponse)]
pub enum BeendendeAntwort<T: ToJSON> {
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

impl<T: ToJSON> BeendendeAntwort<T> {
    pub fn neu<E>(ergebnis: ResultReport<T, E>) -> Self
    where
        E: ThreadSafeError + HttpStatusMapping,
    {
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

/// The throttle key of the client behind one request: real IP plus a header fingerprint,
/// never anything from the body, so a caller cannot choose its own budget.
fn client_key(
    real_ip: RealIp,
    user_agent: &Option<String>,
    accept_language: &Option<String>,
) -> ClientKey {
    // No socket address at all (a test client, a unix socket): loopback is the honest
    // placeholder, and it keeps the budget shared rather than unlimited.
    let ip = real_ip.0.unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST));
    kunde::client_key(ip, user_agent.as_deref(), accept_language.as_deref())
}

// ------------------------------------------------------------------ endpoints

#[OpenApi]
impl LogandaimApiSpec {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<String> {
        Json("OK".to_string())
    }

    #[oai(path = "/schuss", method = "post")]
    async fn schuss_registrieren(
        &self,
        body: Json<SchussErstellungRequest>,
    ) -> TypicalJsonResponse<Schuss> {
        self.app_api.schuss_registrieren(body.0).await.into()
    }

    #[oai(path = "/wertung", method = "post")]
    async fn wertung_berechnen(&self, body: Json<WertungRequest>) -> TypicalJsonResponse<Wertung> {
        self.app_api.wertung_berechnen(body.0).await.into()
    }

    // ------------------------------------------------------------------ konto

    /// Step 1. Answers `200` whether the address is free or taken (AC-2).
    #[oai(path = "/registrierung/start", method = "post")]
    async fn registrierung_starten(
        &self,
        real_ip: RealIp,
        #[oai(name = "User-Agent")] user_agent: Header<Option<String>>,
        #[oai(name = "Accept-Language")] accept_language: Header<Option<String>>,
        body: Json<RegistrierungStartRequest>,
    ) -> KontoAntwort<OkAntwort> {
        let client_key = client_key(real_ip, &user_agent.0, &accept_language.0);
        KontoAntwort::from(
            self.app_api
                .registrierung_starten(body.0, client_key)
                .await
                .map(|()| OkAntwort { ok: true }),
        )
    }

    /// Step 2: creates Benutzer + Konto and answers the first token pair.
    #[oai(path = "/registrierung/abschluss", method = "post")]
    async fn registrierung_abschließen(
        &self,
        real_ip: RealIp,
        #[oai(name = "User-Agent")] user_agent: Header<Option<String>>,
        #[oai(name = "Accept-Language")] accept_language: Header<Option<String>>,
        body: Json<RegistrierungAbschlussRequest>,
    ) -> SessionAntwort {
        let client_key = client_key(real_ip, &user_agent.0, &accept_language.0);
        SessionAntwort::neu(
            self.app_api
                .registrierung_abschließen(body.0, client_key)
                .await,
        )
    }

    #[oai(path = "/anmeldung", method = "post")]
    async fn anmelden(
        &self,
        real_ip: RealIp,
        #[oai(name = "User-Agent")] user_agent: Header<Option<String>>,
        #[oai(name = "Accept-Language")] accept_language: Header<Option<String>>,
        body: Json<AnmeldungRequest>,
    ) -> SessionAntwort {
        let client_key = client_key(real_ip, &user_agent.0, &accept_language.0);
        SessionAntwort::neu(self.app_api.anmelden(body.0, client_key).await)
    }

    /// Rotate the refresh token from the cookie.
    #[oai(path = "/anmeldung/erneuerung", method = "post")]
    async fn erneuerung(
        &self,
        real_ip: RealIp,
        #[oai(name = "User-Agent")] user_agent: Header<Option<String>>,
        #[oai(name = "Accept-Language")] accept_language: Header<Option<String>>,
        #[oai(name = "__Host-logandaim_refresh")] refresh: Cookie<Option<String>>,
    ) -> SessionAntwort {
        let client_key = client_key(real_ip, &user_agent.0, &accept_language.0);
        let ergebnis = match refresh.0.filter(|token| !token.is_empty()) {
            Some(token) => self.app_api.erneuerung(token, client_key).await,
            None => Err(Report::new(ErneuerungFehler::SessionUngültig)),
        };
        SessionAntwort::neu(ergebnis)
    }

    #[oai(path = "/anmeldung/abmeldung", method = "post")]
    async fn abmelden(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
    ) -> BeendendeAntwort<OkAntwort> {
        let ergebnis: ResultReport<(), AbmeldungFehler> = match self
            .app_api
            .require_authenticated(authorization.0.as_deref())
            .await
        {
            Ok(auth) => self.app_api.abmelden(auth).await,
            Err(_) => Err(Report::new(AbmeldungFehler::NichtAngemeldet)),
        };
        BeendendeAntwort::neu(ergebnis.map(|()| OkAntwort { ok: true }))
    }

    #[oai(path = "/passwort-vergessen", method = "post")]
    async fn passwort_vergessen(
        &self,
        real_ip: RealIp,
        #[oai(name = "User-Agent")] user_agent: Header<Option<String>>,
        #[oai(name = "Accept-Language")] accept_language: Header<Option<String>>,
        body: Json<PasswortVergessenRequest>,
    ) -> KontoAntwort<OkAntwort> {
        let client_key = client_key(real_ip, &user_agent.0, &accept_language.0);
        KontoAntwort::from(
            self.app_api
                .passwort_vergessen(body.0, client_key)
                .await
                .map(|()| OkAntwort { ok: true }),
        )
    }

    /// Single use, and ends every session: the cookie is cleared with them.
    #[oai(path = "/passwort-wiederherstellen", method = "post")]
    async fn passwort_wiederherstellen(
        &self,
        body: Json<PasswortWiederherstellenRequest>,
    ) -> BeendendeAntwort<OkAntwort> {
        let ergebnis: ResultReport<(), PasswortWiederherstellenFehler> =
            self.app_api.passwort_wiederherstellen(body.0).await;
        BeendendeAntwort::neu(ergebnis.map(|()| OkAntwort { ok: true }))
    }

    /// The caller's own Benutzer. `?benutzerId=` names another one and needs
    /// `Systemadministrator`.
    #[oai(path = "/benutzer", method = "get")]
    async fn benutzer_anzeigen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
        #[oai(name = "benutzerId")] benutzer_id: Query<Option<Uuid>>,
    ) -> KontoAntwort<BenutzerAntwort> {
        let ergebnis: ResultReport<BenutzerAntwort, BenutzerAnzeigenFehler> = match self
            .app_api
            .require_authenticated(authorization.0.as_deref())
            .await
        {
            Ok(auth) => self.app_api.benutzer_anzeigen(auth, benutzer_id.0).await,
            Err(_) => Err(Report::new(BenutzerAnzeigenFehler::NichtAngemeldet)),
        };
        KontoAntwort::from(ergebnis)
    }

    /// Deletes the caller's own Benutzer. The cookie is cleared with the identity.
    #[oai(path = "/benutzer/löschung", method = "post")]
    async fn benutzer_löschen(
        &self,
        #[oai(name = "Authorization")] authorization: Header<Option<String>>,
        body: Json<BenutzerLöschungRequest>,
    ) -> BeendendeAntwort<OkAntwort> {
        let ergebnis: ResultReport<(), BenutzerLöschenFehler> = match self
            .app_api
            .require_authenticated(authorization.0.as_deref())
            .await
        {
            Ok(auth) => self.app_api.benutzer_löschen(auth, body.0).await,
            Err(_) => Err(Report::new(BenutzerLöschenFehler::NichtAngemeldet)),
        };
        BeendendeAntwort::neu(ergebnis.map(|()| OkAntwort { ok: true }))
    }
}

pub fn openapi_service(
    app: App,
    server_urls: impl IntoIterator<Item = impl Into<ServerObject>>,
) -> OpenApiService<LogandaimApiSpec, ()> {
    let mut service = OpenApiService::new(
        LogandaimApiSpec::from(app),
        "logandaim.at API",
        env!("CARGO_PKG_VERSION"),
    )
    .description(env!("CARGO_PKG_DESCRIPTION"));
    for server_url in server_urls {
        service = service.server(server_url);
    }
    service
}
