//! In-memory fakes for logandaim.at ports — usable in integration tests, scripts, and seed tooling.

mod clock;
mod uow;

pub use clock::*;
pub use uow::*;
