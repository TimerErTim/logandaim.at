//! SQLite repository adapters.
//!
//! One module per domain repository. Shared parsing/formatting helpers live in
//! [`common`].

mod anlage;
mod benutzer;
mod bestaetigung;
pub mod common;
mod konto;
mod passwort_wiederherstellung;
mod runde;
mod schuss;
mod session;

pub use anlage::SQLiteAnlageRepository;
pub use benutzer::SQLiteBenutzerRepository;
pub use bestaetigung::SQLiteEmailBestätigungRepository;
pub use konto::SQLiteKontoRepository;
pub use passwort_wiederherstellung::SQLitePasswortWiederherstellungRepository;
pub use runde::SQLiteRundeRepository;
pub use schuss::SQLiteSchussRepository;
pub use session::SQLiteSessionRepository;
