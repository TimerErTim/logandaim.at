use error_stack::Report;
use futures::StreamExt;
use http::StatusCode;
use logandaim_core::{App, ThreadSafeError, ports::ObjectStream};
use poem::{Body, IntoResponse, Request, Response};
use poem_openapi::{
    OpenApi, OpenApiService, ServerObject,
    param::Query,
    payload::{Json, Payload, PlainText},
    registry::{MetaSchema, MetaSchemaRef},
    types::ToJSON,
};
use uuid::Uuid;

use crate::{
    api::{
        LogandaimAppApi,
        requests::{
            AnmeldeAnfrage, BenutzerLoeschungAnfrage, PasswortVergessenAnfrage,
            PasswortWiederherstellenAnfrage, RegistrierungAbschlussAnfrage,
            RegistrierungStartAnfrage, SchussErstellung, WertungsAnfrage,
        },
    },
    errors::{HttpStatusMapping, InternalServerError, StructuredError, status_from_report},
    schema::{Schuss, Wertung},
};

pub struct LogandaimApiSpec {
    app_api: LogandaimAppApi,
}

impl LogandaimApiSpec {
    pub fn new(app_api: LogandaimAppApi) -> Self {
        Self { app_api }
    }

    /// The facade, for the route impls that live in their own module.
    pub(crate) fn api(&self) -> &LogandaimAppApi {
        &self.app_api
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

#[derive(poem_openapi::ApiResponse)]
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

#[derive(poem_openapi::ApiResponse)]
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

use crate::api::routes::konto as konto_routes;
use crate::api::session::{BeendendeAntwort, KontoAntwort, SitzungAntwort};
use crate::schema::{BenutzerAntwort, OkAntwort};

#[OpenApi]
impl LogandaimApiSpec {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<String> {
        let _ = &self.app_api;
        Json("OK".to_string())
    }

    #[oai(path = "/schuss", method = "post")]
    async fn schuss_registrieren(
        &self,
        body: Json<SchussErstellung>,
    ) -> TypicalJsonResponse<Schuss> {
        self.app_api.schuss_registrieren(body.0).await.into()
    }

    #[oai(path = "/wertung", method = "post")]
    async fn wertung_berechnen(&self, body: Json<WertungsAnfrage>) -> TypicalJsonResponse<Wertung> {
        self.app_api.wertung_berechnen(body.0).await.into()
    }

    // ------------------------------------------------------------------ konto

    /// Step 1 of registration. Answers `200` whether the address is free or taken, so the
    /// endpoint is not an address oracle (AC-2).
    #[oai(path = "/registrierung/start", method = "post")]
    async fn registrierung_starten(
        &self,
        req: &Request,
        body: Json<RegistrierungStartAnfrage>,
    ) -> KontoAntwort<OkAntwort> {
        konto_routes::registrierung_starten(self, req, body).await
    }

    /// Step 2: creates Benutzer + Konto and answers the first token pair.
    #[oai(path = "/registrierung/abschluss", method = "post")]
    async fn registrierung_abschliessen(
        &self,
        req: &Request,
        body: Json<RegistrierungAbschlussAnfrage>,
    ) -> SitzungAntwort {
        konto_routes::registrierung_abschliessen(self, req, body).await
    }

    #[oai(path = "/anmeldung", method = "post")]
    async fn anmelden(&self, req: &Request, body: Json<AnmeldeAnfrage>) -> SitzungAntwort {
        konto_routes::anmelden(self, req, body).await
    }

    /// Rotate the refresh token from the cookie.
    #[oai(path = "/anmeldung/erneuerung", method = "post")]
    async fn erneuerung(&self, req: &Request) -> SitzungAntwort {
        konto_routes::erneuerung(self, req).await
    }

    #[oai(path = "/anmeldung/abmeldung", method = "post")]
    async fn abmelden(&self, req: &Request) -> BeendendeAntwort<OkAntwort> {
        konto_routes::abmelden(self, req).await
    }

    #[oai(path = "/passwort-vergessen", method = "post")]
    async fn passwort_vergessen(
        &self,
        req: &Request,
        body: Json<PasswortVergessenAnfrage>,
    ) -> KontoAntwort<OkAntwort> {
        konto_routes::passwort_vergessen(self, req, body).await
    }

    /// Single use, and ends every session: the cookie is cleared with them.
    #[oai(path = "/passwort-wiederherstellen", method = "post")]
    async fn passwort_wiederherstellen(
        &self,
        body: Json<PasswortWiederherstellenAnfrage>,
    ) -> BeendendeAntwort<OkAntwort> {
        konto_routes::passwort_wiederherstellen(self, body).await
    }

    /// The caller's own Benutzer. `?benutzerId=` names another one and needs
    /// `Systemadministrator`.
    #[oai(path = "/benutzer", method = "get")]
    async fn benutzer_anzeigen(
        &self,
        req: &Request,
        #[oai(name = "benutzerId")] benutzer_id: Query<Option<Uuid>>,
    ) -> KontoAntwort<BenutzerAntwort> {
        konto_routes::benutzer_anzeigen(self, req, benutzer_id.0).await
    }

    /// Deletes the caller's own Benutzer. The cookie is cleared with the identity.
    #[oai(path = "/benutzer/loeschung", method = "post")]
    async fn benutzer_loeschen(
        &self,
        req: &Request,
        body: Json<BenutzerLoeschungAnfrage>,
    ) -> BeendendeAntwort<OkAntwort> {
        konto_routes::benutzer_loeschen(self, req, body).await
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
