//! SQLite repository adapters.
//!
//! One module per domain repository. Shared parsing/formatting helpers live in
//! [`common`].

mod benutzer;
mod bestaetigung;
pub mod common;
mod konto;
mod schuss;
mod session;
mod signing_key;
mod wiederherstellung;

pub use benutzer::SQLiteBenutzerRepository;
pub use bestaetigung::SQLiteEmailBestaetigungRepository;
pub use konto::SQLiteKontoRepository;
pub use schuss::SQLiteSchussRepository;
pub use session::SQLiteSessionRepository;
pub use signing_key::SQLiteSigningKeyRepository;
pub use wiederherstellung::SQLitePasswortWiederherstellungRepository;
