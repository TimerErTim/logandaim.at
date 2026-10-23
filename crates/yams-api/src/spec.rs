use error_stack::Report;
use http::StatusCode;
use poem_openapi::{
    OpenApi, OpenApiService, ServerObject,
    payload::{Json, PlainText},
    types::ToJSON,
};
use yams_core::{App, ThreadSafeError};

use crate::errors::{HttpStatusMapping, InternalServerError, StructuredError, status_from_report};

pub struct YamsApiSpec {
    app: App,
}

impl YamsApiSpec {
    pub fn new(app: App) -> Self {
        Self { app }
    }
}

impl From<App> for YamsApiSpec {
    fn from(app: App) -> Self {
        Self::new(app)
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

#[OpenApi]
impl YamsApiSpec {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<String> {
        let _ = &self.app;
        Json("OK".to_string())
    }
}

pub fn openapi_service(
    app: App,
    server_urls: impl IntoIterator<Item = impl Into<ServerObject>>,
) -> OpenApiService<YamsApiSpec, ()> {
    let mut service = OpenApiService::new(
        YamsApiSpec::from(app),
        "YAMS API",
        env!("CARGO_PKG_VERSION"),
    )
    .description(env!("CARGO_PKG_DESCRIPTION"));
    for server_url in server_urls {
        service = service.server(server_url);
    }
    service
}
