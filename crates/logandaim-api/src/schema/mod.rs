//! OpenAPI schemas. Grouped by domain module (`schuss.rs`), not by the increment that
//! happened to introduce them: later schemas about the same concept belong in the same
//! file.

mod general;
mod kontakt;
mod konto;
mod schuss;

pub use general::*;
pub use kontakt::*;
pub use konto::*;
pub use schuss::*;
