//! OpenAPI schemas. Grouped by domain module (`stammdaten.rs`, `runde.rs`, `schuss.rs`),
//! not by the increment that happened to introduce them: later schemas about the same
//! concept belong in the same file.

mod kontakt;
mod konto;
mod runde;
mod schuss;
mod stammdaten;

pub use kontakt::*;
pub use konto::*;
pub use runde::*;
pub use schuss::*;
pub use stammdaten::*;

/// Acknowledgement of a request that has nothing to return.
///
/// A bare `204` would be indistinguishable from a proxy's empty answer, so the explicit
/// body is worth the two bytes.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase", example))]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ok {
    pub ok: bool,
}

impl Default for Ok {
    fn default() -> Self {
        Self { ok: true }
    }
}

#[cfg(feature = "openapi")]
impl poem_openapi::types::Example for Ok {
    fn example() -> Self {
        Self::default()
    }
}
