//! Repository tracing wrappers.
//!
//! Each wrapper holds a pointer to the parent [`UnitOfWorkImpl`] and delegates through the
//! matching accessor on every call.

use std::pin::Pin;

use async_trait::async_trait;
use tracing::instrument;

use crate::application::uow::UnitOfWorkImpl;
use crate::domain::{NeuerSchuss, Schuss, SchussId};
use crate::ports::{RepositoryResult, SchussRepository};

macro_rules! impl_instrumented_repo {
    (
        $wrapper:ident,
        $trait:ident,
        $accessor:ident,
        methods: [
            $(#[$meta:meta] fn $name:ident(&self $(, $arg:ident : $arg_ty:ty)*) -> $ret:ty;)*
        ]
    ) => {
        pub(crate) struct $wrapper {
            uow: Option<*const dyn UnitOfWorkImpl>,
        }

        impl $wrapper {
            pub(crate) fn new(uow: *const dyn UnitOfWorkImpl) -> Self {
                Self { uow: Some(uow) }
            }

            pub(crate) fn disabled() -> Self {
                Self { uow: None }
            }

            fn uow(&self) -> &dyn UnitOfWorkImpl {
                let uow = self
                    .uow
                    .expect("instrumented repository without unit of work");
                unsafe { &*uow }
            }
        }

        // SAFETY: `uow` points into a pinned `RepoStorage`; the address is stable until
        // the storage is consumed on commit/rollback.
        unsafe impl Send for $wrapper {}
        unsafe impl Sync for $wrapper {}

        #[async_trait]
        impl $trait for $wrapper {
            $(#[$meta]
            async fn $name(&self $(, $arg : $arg_ty)*) -> $ret {
                self.uow().$accessor().$name($($arg),*).await
            })*
        }
    };
}

impl_instrumented_repo!(
    InstrumentedSchussRepository,
    SchussRepository,
    schüsse,
    methods: [
        #[instrument(skip(self, schuss), level = "debug", err(Debug))]
        fn create(&self, schuss: NeuerSchuss) -> RepositoryResult<Schuss>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: SchussId) -> RepositoryResult<Schuss>;
        #[instrument(skip(self, ids), fields(anzahl = ids.len()), level = "trace", err(Debug))]
        fn find_by_ids(&self, ids: &[SchussId]) -> RepositoryResult<Vec<Schuss>>;
    ]
);

/// Pinned bundle: instrumented repos hold pointers into `uow` (field order keeps drop safe).
pub(crate) struct RepoStorage {
    pub(crate) schüsse: InstrumentedSchussRepository,
    pub(crate) uow: Box<dyn UnitOfWorkImpl>,
}

impl RepoStorage {
    pub(crate) fn new(uow: Box<dyn UnitOfWorkImpl>) -> Pin<Box<Self>> {
        let mut pinned = Box::pin(RepoStorage {
            schüsse: InstrumentedSchussRepository::disabled(),
            uow,
        });

        // SAFETY: `pinned` is boxed, so `uow`'s address is stable for the lifetime of the
        // storage. The wrappers are never moved out; they are dropped in place with it.
        unsafe {
            let this = pinned.as_mut().get_unchecked_mut();
            let uow_ptr = std::ptr::from_ref(this.uow.as_ref());
            this.schüsse = InstrumentedSchussRepository::new(uow_ptr);
        }

        pinned
    }
}
