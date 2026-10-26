use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;

use logandaim_core::{
    ErrorReportExt,
    application::uow::Versioned,
    domain::konto::{
        BenutzerId, NeueSession, SecretTokenHash, Session, SessionId, SessionIn, SigningKeyId,
    },
    ports::{RepositoryError, RepositoryResult, SessionRepository},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{format_datetime, parse_datetime, parse_uuid, parse_version},
};

const SELECT: &str = r#"SELECT id, benutzer_id, refresh_token_hash, signing_key_id, public_key,
    created_at, expires_at, spent_at, revoked_at, _version FROM "sessions""#;

fn option_datetime(raw: Option<String>) -> RepositoryResult<Option<chrono::DateTime<chrono::Utc>>> {
    raw.map(|wert| parse_datetime(&wert).contextualize(RepositoryError::Data))
        .transpose()
}

fn session_from_row(row: &Row) -> RepositoryResult<Versioned<Session>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let benutzer_id_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let refresh_token_hash_raw: String = row.get(2).contextualize(RepositoryError::Data)?;
    let signing_key_id_raw: String = row.get(3).contextualize(RepositoryError::Data)?;
    let public_key: String = row.get(4).contextualize(RepositoryError::Data)?;
    let created_at_raw: String = row.get(5).contextualize(RepositoryError::Data)?;
    let expires_at_raw: String = row.get(6).contextualize(RepositoryError::Data)?;
    let spent_at_raw: Option<String> = row.get(7).contextualize(RepositoryError::Data)?;
    let revoked_at_raw: Option<String> = row.get(8).contextualize(RepositoryError::Data)?;

    let id = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let benutzer_id = parse_uuid(&benutzer_id_raw).contextualize(RepositoryError::Data)?;
    let refresh_token_hash =
        SecretTokenHash::from_hex(refresh_token_hash_raw).change_context(RepositoryError::Data)?;
    let signing_key_id =
        SigningKeyId::new(signing_key_id_raw).change_context(RepositoryError::Data)?;
    let created_at = parse_datetime(&created_at_raw).contextualize(RepositoryError::Data)?;
    let expires_at = parse_datetime(&expires_at_raw).contextualize(RepositoryError::Data)?;

    Ok(Versioned::new(
        parse_version(row, 9)?,
        Session::from_parts(
            SessionId(id),
            BenutzerId(benutzer_id),
            refresh_token_hash,
            signing_key_id,
            public_key,
            created_at,
            expires_at,
            option_datetime(spent_at_raw)?,
            option_datetime(revoked_at_raw)?,
        ),
    ))
}

pub struct SQLiteSessionRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl SessionRepository for SQLiteSessionRepository {
    async fn create(&self, session: NeueSession) -> RepositoryResult<Versioned<Session>> {
        let id = SessionId(Uuid::new_v4());
        let domain = Session::Ok(SessionIn::aus(id, session.clone()));

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "sessions"
               (id, benutzer_id, refresh_token_hash, signing_key_id, public_key, created_at,
                expires_at, spent_at, revoked_at, _version)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL, NULL, 0)"#,
            (
                id.0.to_string(),
                session.benutzer_id().0.to_string(),
                session.refresh_token_hash().as_str().to_string(),
                session.signing_key_id().as_str().to_string(),
                session.public_key_hex().to_string(),
                format_datetime(session.created_at()),
                format_datetime(session.expires_at()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(Versioned::init(domain))
    }

    async fn find_by_id(&self, id: SessionId) -> RepositoryResult<Versioned<Session>> {
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

        session_from_row(&row)
    }

    async fn find_by_refresh_hash(
        &self,
        hash: &SecretTokenHash,
    ) -> RepositoryResult<Option<Versioned<Session>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                &format!(r#"{SELECT} WHERE refresh_token_hash = ?1"#),
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

        session_from_row(&row).map(Some)
    }

    async fn update(&self, session: &Versioned<Session>) -> RepositoryResult<Versioned<Session>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let betroffen = tx
            .execute(
                r#"UPDATE "sessions" SET benutzer_id = ?2, refresh_token_hash = ?3,
                       signing_key_id = ?4, public_key = ?5, created_at = ?6, expires_at = ?7,
                       spent_at = ?8, revoked_at = ?9, _version = ?10 + 1
                   WHERE id = ?1 AND _version = ?10"#,
                (
                    session.id().0.to_string(),
                    session.benutzer_id().0.to_string(),
                    session.refresh_token_hash().as_str().to_string(),
                    session.signing_key_id().as_str().to_string(),
                    session.public_key_hex().to_string(),
                    format_datetime(session.created_at()),
                    format_datetime(session.expires_at()),
                    session.spent_at().map(format_datetime),
                    session.revoked_at().map(format_datetime),
                    session.v() as i64,
                ),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if betroffen == 0 {
            return Err(RepositoryError::VersionMismatch {
                expected: session.v(),
                actual: None,
            }
            .into());
        }

        Ok(session.clone().incremented())
    }

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Session>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                &format!(r#"{SELECT} WHERE benutzer_id = ?1"#),
                [benutzer_id.0.to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut sessions = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            sessions.push(session_from_row(&row)?);
        }
        Ok(sessions)
    }

    async fn delete(&self, id: SessionId) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let betroffen = tx
            .execute(
                r#"DELETE FROM "sessions" WHERE id = ?1"#,
                [id.0.to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if betroffen == 0 {
            return Err(RepositoryError::NotFound.into());
        }
        Ok(())
    }

    async fn delete_by_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"DELETE FROM "sessions" WHERE benutzer_id = ?1"#,
            [benutzer_id.0.to_string()],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
        Ok(())
    }
}
