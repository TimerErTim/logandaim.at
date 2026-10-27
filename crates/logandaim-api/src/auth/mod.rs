//! The auth edge: the JOSE envelope, the refresh cookie, and client identity.
//!
//! Everything here is transport. The claims' *meaning* (is the window open, is the session
//! live, is the key active) is decided by `SessionPrüfen` in core, so no rule is
//! duplicated at this layer.

pub mod cookie;
pub mod jwt;
pub mod kunde;
