use error_stack::Report;
use futures::StreamExt;
use http::StatusCode;
use logandaim_core::{App, ThreadSafeError, ports::ObjectStream};
use poem::{Body, IntoResponse, Response};
use poem_openapi::{
    OpenApi, OpenApiService, ServerObject,
    payload::{Json, Payload, PlainText},
    registry::{MetaSchema, MetaSchemaRef},
    types::ToJSON,
};

use crate::{
    api::{
        LogandaimAppApi,
        requests::{SchussErstellung, WertungsAnfrage},
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
