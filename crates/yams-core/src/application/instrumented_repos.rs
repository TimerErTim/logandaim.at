//! Repository tracing wrappers.
//!
//! Each wrapper holds a pointer to the parent [`UnitOfWorkImpl`] and delegates
//! through the matching accessor on every call. Domain repositories are added
//! to [`RepoStorage`] as feature slices land.

use std::pin::Pin;

use crate::application::uow::UnitOfWorkImpl;

/// Pinned bundle: instrumented repos hold pointers into `uow` (field order keeps drop safe).
pub(crate) struct RepoStorage {
    pub(crate) uow: Box<dyn UnitOfWorkImpl>,
}

impl RepoStorage {
    pub(crate) fn new(uow: Box<dyn UnitOfWorkImpl>) -> Pin<Box<Self>> {
        Box::pin(RepoStorage { uow })
    }
}
