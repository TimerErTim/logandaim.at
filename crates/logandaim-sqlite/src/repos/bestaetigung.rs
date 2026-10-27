use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;

use logandaim_core::{
    ErrorReportExt,
    application::uow::Versioned,
    domain::{
        EmailAdresse,
        konto::{BestätigungId, EmailBestätigung, NeueBestätigung, SecretTokenHash},
    },
    ports::{EmailBestätigungRepository, RepositoryError, RepositoryResult},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{
        delete_versioned, format_datetime, parse_datetime, parse_uuid, parse_version,
        update_versioned,
    },
};

const SELECT: &str =
    r#"SELECT id, email, token_hash, expires_at, spent_at, _version FROM "email_bestätigungen""#;

fn bestätigung_from_row(row: &Row) -> RepositoryResult<Versioned<EmailBestätigung>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let email_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let token_hash_raw: String = row.get(2).contextualize(RepositoryError::Data)?;
    let expires_at_raw: String = row.get(3).contextualize(RepositoryError::Data)?;
    let spent_at_raw: Option<String> = row.get(4).contextualize(RepositoryError::Data)?;

    let id = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let email = EmailAdresse::normalisiert(email_raw).change_context(RepositoryError::Data)?;
    let token_hash =
        SecretTokenHash::from_hex(token_hash_raw).change_context(RepositoryError::Data)?;
    let expires_at = parse_datetime(&expires_at_raw).contextualize(RepositoryError::Data)?;
    let spent_at = spent_at_raw
        .map(|wert| parse_datetime(&wert).contextualize(RepositoryError::Data))
        .transpose()?;

    Ok(Versioned::new(
        parse_version(row, 5)?,
        EmailBestätigung::from_parts(BestätigungId(id), email, token_hash, expires_at, spent_at),
    ))
}

pub struct SQLiteEmailBestätigungRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl EmailBestätigungRepository for SQLiteEmailBestätigungRepository {
    async fn create(
        &self,
        bestätigung: NeueBestätigung,
    ) -> RepositoryResult<Versioned<EmailBestätigung>> {
        let id = BestätigungId(Uuid::new_v4());
        let domain = EmailBestätigung::neu(id, bestätigung.clone());

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "email_bestätigungen" (id, email, token_hash, expires_at, spent_at, _version)
               VALUES (?1, ?2, ?3, ?4, NULL, 0)"#,
            (
                id.0.to_string(),
                bestätigung.email().as_ref().to_string(),
                bestätigung.token_hash().as_str().to_string(),
                format_datetime(bestätigung.expires_at()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(Versioned::init(domain))
    }

    async fn find_by_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Option<Versioned<EmailBestätigung>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                &format!(r#"{SELECT} WHERE token_hash = ?1"#),
                [hash.as_str().to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        else {
            return Ok(None);
        };

        bestätigung_from_row(&row).map(Some)
    }

    async fn find_by_id(
        &self,
        id: BestätigungId,
    ) -> RepositoryResult<Versioned<EmailBestätigung>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(&format!(r#"{SELECT} WHERE id = ?1"#), [id.0.to_string()])
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        else {
            return Err(RepositoryError::NotFound.into());
        };

        bestätigung_from_row(&row)
    }

    async fn find_by_email(
        &self,
        email: &EmailAdresse,
    ) -> RepositoryResult<Vec<Versioned<EmailBestätigung>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                &format!(r#"{SELECT} WHERE email = ?1"#),
                [email.as_ref().to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut bestätigungen = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            bestätigungen.push(bestätigung_from_row(&row)?);
        }
        Ok(bestätigungen)
    }

    /// Full-row write-back with an in-place version bump. Spending a confirmation is a
    /// domain transition; this only persists it.
    async fn update(
        &self, bestätigung: &mut Versioned<EmailBestätigung>
    ) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        update_versioned(
            tx,
            r#"UPDATE "email_bestätigungen" SET email = ?2, token_hash = ?3,
                   expires_at = ?4, spent_at = ?5, _version = ?6 + 1
               WHERE id = ?1 AND _version = ?6"#,
            (
                bestätigung.id().0.to_string(),
                bestätigung.email().as_ref().to_string(),
                bestätigung.token_hash().as_str().to_string(),
                format_datetime(bestätigung.expires_at()),
                bestätigung.spent_at().map(format_datetime),
                bestätigung.v() as i64,
            ),
            bestätigung.v(),
        )
        .await?;

        bestätigung.increment();
        Ok(())
    }

    /// Delete guarded by the version the caller holds.
    async fn delete(&self, bestätigung: Versioned<EmailBestätigung>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        delete_versioned(
            tx,
            r#"DELETE FROM "email_bestätigungen" WHERE id = ?1 AND _version = ?2"#,
            (bestätigung.id().0.to_string(), bestätigung.v() as i64),
            bestätigung.v(),
        )
        .await
    }

    async fn delete_by_email(&self, email: &EmailAdresse) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"DELETE FROM "email_bestätigungen" WHERE email = ?1"#,
            [email.as_ref().to_string()],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
        Ok(())
    }
}
