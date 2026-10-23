//! SQLite repository adapters.
//!
//! One module per domain repository, added as feature slices land. Shared
//! parsing/formatting helpers live in [`common`]; `docs/vertical-slice.md` walks
//! through adding one.

pub mod common;
mod schuss;

pub use schuss::SQLiteSchussRepository;
