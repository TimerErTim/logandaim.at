use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use libsql::Transaction;
use logandaim_core::{
    ErrorReportExt, ResultReport,
    ports::{
        AnlageRepository, BenutzerRepository, EmailBestätigungRepository, KontoRepository,
        PasswortWiederherstellungRepository, RepositoryError, RepositoryResult, RundeRepository,
        SchussRepository, SessionRepository,
    },
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

use crate::{
    SQLiteConnection, SQLiteInstance,
    errors::libsql_error_to_persistence_error,
    repos::{
        SQLiteAnlageRepository, SQLiteBenutzerRepository, SQLiteEmailBestätigungRepository,
        SQLiteKontoRepository, SQLitePasswortWiederherstellungRepository, SQLiteRundeRepository,
        SQLiteSchussRepository, SQLiteSessionRepository,
    },
};

pub struct SQLiteUnitOfWork {
    /// Held for the whole UoW so the connection outlives the transaction.
    /// Dropped after repos/tx (field order). Unused after `checkpoint` was removed.
    #[allow(dead_code)]
    connection: SQLiteConnection,
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
    anlage_repo: SQLiteAnlageRepository,
    runde_repo: SQLiteRundeRepository,
    schuss_repo: SQLiteSchussRepository,
    benutzer_repo: SQLiteBenutzerRepository,
    konto_repo: SQLiteKontoRepository,
    session_repo: SQLiteSessionRepository,
    bestätigung_repo: SQLiteEmailBestätigungRepository,
    wiederherstellung_repo: SQLitePasswortWiederherstellungRepository,
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
        Ok(Box::new(SQLiteUnitOfWork {
            connection,
            anlage_repo: SQLiteAnlageRepository { tx: tx.clone() },
            runde_repo: SQLiteRundeRepository { tx: tx.clone() },
            schuss_repo: SQLiteSchussRepository { tx: tx.clone() },
            benutzer_repo: SQLiteBenutzerRepository { tx: tx.clone() },
            konto_repo: SQLiteKontoRepository { tx: tx.clone() },
            session_repo: SQLiteSessionRepository { tx: tx.clone() },
            bestätigung_repo: SQLiteEmailBestätigungRepository { tx: tx.clone() },
            wiederherstellung_repo: SQLitePasswortWiederherstellungRepository { tx: tx.clone() },
            tx,
        }))
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

    fn anlagen(&self) -> &dyn AnlageRepository {
        &self.anlage_repo
    }

    fn runden(&self) -> &dyn RundeRepository {
        &self.runde_repo
    }

    fn schüsse(&self) -> &dyn SchussRepository {
        &self.schuss_repo
    }

    fn benutzer(&self) -> &dyn BenutzerRepository {
        &self.benutzer_repo
    }

    fn konten(&self) -> &dyn KontoRepository {
        &self.konto_repo
    }

    fn sessions(&self) -> &dyn SessionRepository {
        &self.session_repo
    }

    fn bestätigungen(&self) -> &dyn EmailBestätigungRepository {
        &self.bestätigung_repo
    }

    fn wiederherstellungen(&self) -> &dyn PasswortWiederherstellungRepository {
        &self.wiederherstellung_repo
    }
}
