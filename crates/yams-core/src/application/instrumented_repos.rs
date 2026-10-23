//! Repository tracing wrappers.
//!
//! Each wrapper holds a pointer to the parent [`UnitOfWorkImpl`] and delegates
//! through the matching accessor on every call. Domain repositories are added
//! here as feature slices land.
