use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;

use logandaim_core::{
    ErrorReportExt,
    domain::{
        EmailAdresse,
        konto::{BestaetigungId, EmailBestaetigung, NeueBestaetigung, TokenHash},
    },
    ports::{EmailBestaetigungRepository, RepositoryError, RepositoryResult},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{format_datetime, parse_datetime, parse_uuid},
};

const SELECT: &str =
    r#"SELECT id, email, token_hash, expires_at, spent_at FROM "email_bestaetigungen""#;

fn bestaetigung_from_row(row: &Row) -> RepositoryResult<EmailBestaetigung> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let email_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let token_hash_raw: String = row.get(2).contextualize(RepositoryError::Data)?;
    let expires_at_raw: String = row.get(3).contextualize(RepositoryError::Data)?;
    let spent_at_raw: Option<String> = row.get(4).contextualize(RepositoryError::Data)?;

    let id = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let email = EmailAdresse::new(email_raw).change_context(RepositoryError::Data)?;
    let token_hash = TokenHash::from_hex(token_hash_raw).change_context(RepositoryError::Data)?;
    let expires_at = parse_datetime(&expires_at_raw).contextualize(RepositoryError::Data)?;
    let spent_at = spent_at_raw
        .map(|wert| parse_datetime(&wert).contextualize(RepositoryError::Data))
        .transpose()?;

    Ok(EmailBestaetigung::from_parts(
        BestaetigungId(id),
        email,
        token_hash,
        expires_at,
        spent_at,
    ))
}

pub struct SQLiteEmailBestaetigungRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl EmailBestaetigungRepository for SQLiteEmailBestaetigungRepository {
    async fn create(&self, bestaetigung: NeueBestaetigung) -> RepositoryResult<EmailBestaetigung> {
        let id = BestaetigungId(Uuid::new_v4());
        let domain = EmailBestaetigung::neu(id, bestaetigung.clone());

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "email_bestaetigungen" (id, email, token_hash, expires_at, spent_at, _version)
               VALUES (?1, ?2, ?3, ?4, NULL, 0)"#,
            (
                id.0.to_string(),
                bestaetigung.email().as_ref().to_string(),
                bestaetigung.token_hash().as_str().to_string(),
                format_datetime(bestaetigung.expires_at()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(domain)
    }

    async fn find_by_hash(&self, hash: &TokenHash) -> RepositoryResult<Option<EmailBestaetigung>> {
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

        bestaetigung_from_row(&row).map(Some)
    }

    async fn consume(
        &self,
        id: BestaetigungId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        // Conditional update: the second spender matches no row, so `Conflict` is the
        // honest answer even when two requests interleave (AC-4, AC-13).
        let betroffen = tx
            .execute(
                r#"UPDATE "email_bestaetigungen" SET spent_at = ?2, _version = _version + 1
                   WHERE id = ?1 AND spent_at IS NULL"#,
                (id.0.to_string(), format_datetime(jetzt)),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if betroffen == 0 {
            return Err(RepositoryError::Conflict.into());
        }
        Ok(())
    }

    async fn invalidate_open_for_email(
        &self,
        email: &EmailAdresse,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"UPDATE "email_bestaetigungen" SET spent_at = ?2, _version = _version + 1
               WHERE email = ?1 AND spent_at IS NULL"#,
            (email.as_ref().to_string(), format_datetime(jetzt)),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
        Ok(())
    }

    async fn delete_alle_fuer_email(&self, email: &EmailAdresse) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"DELETE FROM "email_bestaetigungen" WHERE email = ?1"#,
            [email.as_ref().to_string()],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
        Ok(())
    }
}
