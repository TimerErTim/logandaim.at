//! Request DTOs. Grouped by domain module (`stammdaten.rs`, `runde.rs`, `schuss.rs`), not
//! by the increment that happened to introduce them: later requests about the same concept
//! belong in the same file.

mod konto;
mod runde;
mod schuss;
mod stammdaten;

pub use konto::*;
pub use runde::*;
pub use schuss::*;
pub use stammdaten::*;
