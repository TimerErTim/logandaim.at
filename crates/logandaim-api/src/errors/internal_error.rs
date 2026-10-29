use thiserror::Error;

#[derive(Debug, Error)]
#[error("Internal server error")]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalServerError;

impl From<InternalServerError> for String {
    fn from(value: InternalServerError) -> Self {
        format!("{}", value)
    }
}
