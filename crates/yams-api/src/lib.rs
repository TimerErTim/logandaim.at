pub mod errors;

#[cfg(feature = "openapi")]
mod spec;
#[cfg(feature = "openapi")]
pub use spec::*;
