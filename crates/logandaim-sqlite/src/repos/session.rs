use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;

use logandaim_core::{
    ErrorReportExt,
    domain::konto::{BenutzerId, NeueSession, Session, SessionId, SigningKeyId, TokenHash},
    ports::{RepositoryError, RepositoryResult, SessionRepository},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{format_datetime, parse_datetime, parse_uuid},
};

const SELECT: &str = r#"SELECT id, benutzer_id, refresh_token_hash, signing_key_id, erstellt_am,
        expires_at, spent_at, revoked_at FROM "sessions""#;

fn option_datetime(raw: Option<String>) -> RepositoryResult<Option<chrono::DateTime<chrono::Utc>>> {
    raw.map(|wert| parse_datetime(&wert).contextualize(RepositoryError::Data))
        .transpose()
}

fn session_from_row(row: &Row) -> RepositoryResult<Session> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let benutzer_id_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let refresh_token_hash_raw: String = row.get(2).contextualize(RepositoryError::Data)?;
    let signing_key_id_raw: String = row.get(3).contextualize(RepositoryError::Data)?;
    let erstellt_am_raw: String = row.get(4).contextualize(RepositoryError::Data)?;
    let expires_at_raw: String = row.get(5).contextualize(RepositoryError::Data)?;
    let spent_at_raw: Option<String> = row.get(6).contextualize(RepositoryError::Data)?;
    let revoked_at_raw: Option<String> = row.get(7).contextualize(RepositoryError::Data)?;

    let id = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let benutzer_id = parse_uuid(&benutzer_id_raw).contextualize(RepositoryError::Data)?;
    let refresh_token_hash =
        TokenHash::from_hex(refresh_token_hash_raw).change_context(RepositoryError::Data)?;
    let signing_key_id =
        SigningKeyId::new(signing_key_id_raw).change_context(RepositoryError::Data)?;
    let erstellt_am = parse_datetime(&erstellt_am_raw).contextualize(RepositoryError::Data)?;
    let expires_at = parse_datetime(&expires_at_raw).contextualize(RepositoryError::Data)?;

    Ok(Session::from_parts(
        SessionId(id),
        BenutzerId(benutzer_id),
        refresh_token_hash,
        signing_key_id,
        erstellt_am,
        expires_at,
        option_datetime(spent_at_raw)?,
        option_datetime(revoked_at_raw)?,
    ))
}

pub struct SQLiteSessionRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl SessionRepository for SQLiteSessionRepository {
    async fn create(&self, session: NeueSession) -> RepositoryResult<Session> {
        let id = SessionId(Uuid::new_v4());
        let domain = Session::neu(id, session.clone());

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "sessions"
               (id, benutzer_id, refresh_token_hash, signing_key_id, erstellt_am, expires_at, spent_at, revoked_at, _version)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL, NULL, 0)"#,
            (
                id.0.to_string(),
                session.benutzer_id().0.to_string(),
                session.refresh_token_hash().as_str().to_string(),
                session.signing_key_id().as_str().to_string(),
                format_datetime(session.erstellt_am()),
                format_datetime(session.expires_at()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(domain)
    }

    async fn find_by_id(&self, id: SessionId) -> RepositoryResult<Session> {
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

    async fn find_by_refresh_hash(&self, hash: &TokenHash) -> RepositoryResult<Option<Session>> {
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

    async fn markiere_verbraucht(
        &self,
        id: SessionId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        // `COALESCE` keeps the first spend: spending twice must not move the timestamp.
        self.mutate(
            r#"UPDATE "sessions" SET spent_at = COALESCE(spent_at, ?2), _version = _version + 1 WHERE id = ?1"#,
            (id.0.to_string(), format_datetime(jetzt)),
        )
        .await
    }

    async fn widerrufe(
        &self,
        id: SessionId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        self.mutate(
            r#"UPDATE "sessions" SET revoked_at = COALESCE(revoked_at, ?2), _version = _version + 1 WHERE id = ?1"#,
            (id.0.to_string(), format_datetime(jetzt)),
        )
        .await
    }

    async fn widerrufe_alle_fuer_benutzer(
        &self,
        benutzer_id: BenutzerId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"UPDATE "sessions" SET revoked_at = COALESCE(revoked_at, ?2), _version = _version + 1
               WHERE benutzer_id = ?1"#,
            (benutzer_id.0.to_string(), format_datetime(jetzt)),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
        Ok(())
    }

    async fn delete_alle_fuer_benutzer(&self, benutzer_id: BenutzerId) -> RepositoryResult<()> {
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

impl SQLiteSessionRepository {
    async fn mutate(&self, sql: &str, params: (String, String)) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let betroffen = tx
            .execute(sql, params)
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        if betroffen == 0 {
            return Err(RepositoryError::NotFound.into());
        }
        Ok(())
    }
}
