use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use libsql::Transaction;
use logandaim_core::{
    ErrorReportExt, ResultReport,
    ports::{RepositoryError, RepositoryResult},
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

use crate::{SQLiteConnection, SQLiteInstance, errors::libsql_error_to_persistence_error};

pub struct SQLiteUnitOfWork {
    /// Held for the whole UoW so the connection outlives the transaction.
    /// Dropped after repos/tx (field order). Unused after `checkpoint` was removed.
    #[allow(dead_code)]
    connection: SQLiteConnection,
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl UnitOfWorkProvider for SQLiteInstance {
    async fn begin(&self) -> ResultReport<Box<dyn UnitOfWorkImpl>, RepositoryError> {
        let connection: SQLiteConnection = self.create_connection().await?;
        let tx = connection
            .transaction_with_behavior(libsql::TransactionBehavior::Deferred)
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        let tx = Arc::new(Mutex::new(Some(tx)));
        Ok(Box::new(SQLiteUnitOfWork { connection, tx }))
    }
}

#[async_trait]
impl UnitOfWorkImpl for SQLiteUnitOfWork {
    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.take().ok_or(RepositoryError::Conflict)?;
        // No incremental_vacuum here: it needs an exclusive lock and races the
        // next connection open. auto_vacuum=INCREMENTAL still tracks free pages;
        // vacuum can run idle/shutdown if freelist reclaim is needed later.
        tx.commit()
            .await
            .contextualize_with(libsql_error_to_persistence_error)
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.take().ok_or(RepositoryError::Conflict)?;
        tx.rollback()
            .await
            .contextualize_with(libsql_error_to_persistence_error)
    }
}
