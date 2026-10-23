//! SQLite repository adapters.
//!
//! One module per domain repository. Shared parsing/formatting helpers live in
//! [`common`].

pub mod common;
mod schuss;

pub use schuss::SQLiteSchussRepository;
