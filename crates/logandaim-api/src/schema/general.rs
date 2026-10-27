//! Schemas that belong to no domain module.

/// Acknowledgement of a request that has nothing to return.
///
/// A bare `204` would be indistinguishable from a proxy's empty answer, so the explicit
/// body is worth the two bytes.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OkAntwort {
    pub ok: bool,
}
