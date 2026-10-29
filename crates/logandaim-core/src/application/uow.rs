use std::ops::{ControlFlow, Deref, DerefMut};
use std::sync::Arc;

use async_trait::async_trait;
use error_stack::{FrameKind, Report};
use std::cmp::Ordering;
use std::fmt::Debug;

use crate::application::ThreadSafeError;
use crate::ports::{
    BenutzerRepository, EmailBestätigungRepository, KontoRepository,
    PasswortWiederherstellungRepository, RepositoryError, RepositoryResult, SchussRepository,
    SessionRepository,
};

/// Active unit of work. `commit` / `rollback` consume it.
///
/// Drop panics if neither `commit` nor `rollback` ran (forgotten conclusion).
/// Does **not** call [`UnitOfWorkImpl::rollback`] — that is async and fallible.
#[must_use = "UnitOfWork must be committed or rolled back"]
pub struct UnitOfWork<'a> {
    implementation: Option<Box<dyn UnitOfWorkImpl + 'a>>,
}

impl<'a> UnitOfWork<'a> {
    pub(crate) fn owned(implementation: Box<dyn UnitOfWorkImpl + 'a>) -> Self {
        Self {
            implementation: Some(implementation),
        }
    }

    pub(crate) fn locked(inner: &'a dyn UnitOfWorkImpl) -> Self {
        Self {
            implementation: Some(Box::new(LockedUnitOfWorkImpl { inner })),
        }
    }

    pub(crate) fn as_impl(&self) -> &dyn UnitOfWorkImpl {
        self.implementation
            .as_deref()
            .expect("UnitOfWork already committed or rolled back")
    }
}

impl UnitOfWork<'_> {
    pub async fn commit(mut self) -> RepositoryResult<()> {
        self.take_impl().commit().await
    }

    pub async fn rollback(mut self) -> RepositoryResult<()> {
        self.take_impl().rollback().await
    }

    /// Commit on `Ok`, rollback on `Err`. Pass a function to translate a `Report<RepositoryError>` from commit into a `Report<C>`.
    #[inline]
    pub async fn finish_with<T, C, F>(
        self,
        result: Result<T, Report<C>>,
        context_from_commit: F,
    ) -> Result<T, Report<C>>
    where
        C: ThreadSafeError,
        F: FnOnce(&Report<RepositoryError>) -> C,
    {
        match result {
            Ok(value) => {
                self.commit().await.map_err(|e| {
                    let new_context = context_from_commit(&e);
                    e.change_context(new_context)
                })?;
                Ok(value)
            }
            Err(mut error) => {
                if let Err(_rollback_error) = self.rollback().await {
                    // Atach the rollback report to the outer error
                    let _ = error.frames_mut(|frame| {
                        let FrameKind::Context(_) = frame.kind() else {
                            return ControlFlow::Continue(());
                        };

                        // Cannot attach right now because report does not support
                        // TODO: Somehow fix this later (upstream feat)

                        ControlFlow::Break(())
                    });
                }
                Err(error)
            }
        }
    }

    /// Commit on `Ok`, rollback on `Err`. Pass a function to translate a `Report<RepositoryError>` from commit into a `Report<C>`.
    #[inline]
    pub fn finish<T, C: ThreadSafeError>(
        self,
        result: Result<T, Report<C>>,
        context: C,
    ) -> impl Future<Output = Result<T, Report<C>>> {
        self.finish_with(result, |_| context)
    }

    /// Repository accessor.
    pub fn schüsse(&self) -> &dyn SchussRepository {
        self.as_impl().schüsse()
    }

    pub fn benutzer(&self) -> &dyn BenutzerRepository {
        self.as_impl().benutzer()
    }

    pub fn konten(&self) -> &dyn KontoRepository {
        self.as_impl().konten()
    }

    pub fn sessions(&self) -> &dyn SessionRepository {
        self.as_impl().sessions()
    }

    pub fn bestätigungen(&self) -> &dyn EmailBestätigungRepository {
        self.as_impl().bestätigungen()
    }

    pub fn wiederherstellungen(&self) -> &dyn PasswortWiederherstellungRepository {
        self.as_impl().wiederherstellungen()
    }

    fn take_impl(&mut self) -> Box<dyn UnitOfWorkImpl + '_> {
        self.implementation
            .take()
            .expect("UnitOfWork already committed or rolled back")
    }
}

impl Drop for UnitOfWork<'_> {
    fn drop(&mut self) {
        if self.implementation.is_some() && !std::thread::panicking() {
            panic!("UnitOfWork dropped without commit or rollback");
        }
    }
}

struct LockedUnitOfWorkImpl<'a> {
    inner: &'a dyn UnitOfWorkImpl,
}

#[async_trait]
impl UnitOfWorkImpl for LockedUnitOfWorkImpl<'_> {
    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        Ok(())
    }

    fn schüsse(&self) -> &dyn SchussRepository {
        self.inner.schüsse()
    }

    fn benutzer(&self) -> &dyn BenutzerRepository {
        self.inner.benutzer()
    }

    fn konten(&self) -> &dyn KontoRepository {
        self.inner.konten()
    }

    fn sessions(&self) -> &dyn SessionRepository {
        self.inner.sessions()
    }

    fn bestätigungen(&self) -> &dyn EmailBestätigungRepository {
        self.inner.bestätigungen()
    }

    fn wiederherstellungen(&self) -> &dyn PasswortWiederherstellungRepository {
        self.inner.wiederherstellungen()
    }
}

/// UoW Provider
#[async_trait]
pub trait UnitOfWorkImpl: Send + Sync {
    async fn commit(self: Box<Self>) -> RepositoryResult<()>;
    async fn rollback(self: Box<Self>) -> RepositoryResult<()>;

    /// One accessor per repository. Each feature slice adds exactly one.
    fn schüsse(&self) -> &dyn SchussRepository;

    fn benutzer(&self) -> &dyn BenutzerRepository;

    fn konten(&self) -> &dyn KontoRepository;

    fn sessions(&self) -> &dyn SessionRepository;

    fn bestätigungen(&self) -> &dyn EmailBestätigungRepository;

    fn wiederherstellungen(&self) -> &dyn PasswortWiederherstellungRepository;
}

#[async_trait]
pub trait UnitOfWorkProvider: Send + Sync {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>>;
}

#[async_trait]
impl<T: UnitOfWorkProvider + ?Sized> UnitOfWorkProvider for Arc<T> {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>> {
        (**self).begin().await
    }
}

pub struct Versioned<T> {
    version: u64,
    data: T,
}

impl<T> Deref for Versioned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Versioned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> PartialEq for Versioned<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl<T> PartialOrd for Versioned<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl<T> Versioned<T> {
    pub fn new(version: u64, data: T) -> Self {
        Self { version, data }
    }

    pub fn init(data: T) -> Self {
        Self { version: 0, data }
    }

    pub fn v(&self) -> u64 {
        self.version
    }

    pub fn increment(&mut self) -> u64 {
        self.version += 1;
        self.version
    }

    pub fn incremented(mut self) -> Self {
        self.increment();
        self
    }

    pub fn into_data(self) -> T {
        self.data
    }

    pub fn cloned_data(&self) -> T
    where
        T: Clone,
    {
        self.data.clone()
    }

    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }

    pub fn map_data(&mut self, f: impl FnOnce(&T) -> T) {
        self.data = f(&self.data);
    }
}

impl<T> Clone for Versioned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            version: self.version,
            data: self.cloned_data(),
        }
    }
}

impl<T> Debug for Versioned<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Versioned")
            .field("version", &self.version)
            .field("data", &self.data)
            .finish()
    }
}
