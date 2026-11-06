use error_stack::Report;
use futures::StreamExt;
use http::StatusCode;
use logandaim_core::{ThreadSafeError, ports::ObjectStream};
use poem::{Body, IntoResponse, Response};
use poem_openapi::{
    ApiResponse,
    payload::{Json, Payload, PlainText},
    registry::{MetaSchema, MetaSchemaRef},
    types::ToJSON,
};

use crate::errors::{HttpStatusMapping, InternalServerError, StructuredError, status_from_report};

mod session;
pub use session::*;

/// Never store an auth response.
const NO_STORE: &str = "no-store";

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

/// A plain answer that requires no caching
#[derive(ApiResponse)]
pub enum UncachedJsonResponse<T: ToJSON> {
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

impl<T, E> From<Result<T, Report<E>>> for UncachedJsonResponse<T>
where
    T: ToJSON,
    E: ThreadSafeError + HttpStatusMapping,
{
    fn from(ergebnis: Result<T, Report<E>>) -> Self {
        match TypicalJsonResponse::from(ergebnis) {
            TypicalJsonResponse::Ok(value) => Self::Ok(value, NO_STORE.to_owned()),
            TypicalJsonResponse::ClientError(status, error) => {
                Self::ClientError(status, error, NO_STORE.to_owned())
            }
            TypicalJsonResponse::InternalError(error) => {
                Self::InternalError(error, NO_STORE.to_owned())
            }
        }
    }
}
