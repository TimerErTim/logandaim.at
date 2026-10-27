//! Default adapters that make an `App` usable without infrastructure wiring.
//!
//! `Signing`, `Throttle`, and `MailSender` are the three ports with a sane production default
//! for a single-process deployment; the rest are deployment choices and have none.

mod blank_pdf_renderer;
mod in_memory_object_store;
mod in_memory_throttle;
mod null_mail_sender;
mod signing;
mod system_clock;

pub use blank_pdf_renderer::{BLANK_PDF, BlankPdfRenderer};
pub use in_memory_object_store::InMemoryObjectStore;
pub use in_memory_throttle::InMemoryThrottle;
pub use null_mail_sender::NullMailSender;
pub use signing::{InMemorySigning, verify_with_public_key_hex};
pub use system_clock::SystemClock;
