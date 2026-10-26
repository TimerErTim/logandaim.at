use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;

use logandaim_core::{
    ErrorReportExt,
    domain::konto::{
        KontoId, NeueWiederherstellung, PasswortWiederherstellung, TokenHash, WiederherstellungId,
    },
    ports::{PasswortWiederherstellungRepository, RepositoryError, RepositoryResult},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{format_datetime, parse_datetime, parse_uuid},
};

const SELECT: &str =
    r#"SELECT id, konto_id, token_hash, expires_at, spent_at FROM "passwort_wiederherstellungen""#;

fn wiederherstellung_from_row(row: &Row) -> RepositoryResult<PasswortWiederherstellung> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let konto_id_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let token_hash_raw: String = row.get(2).contextualize(RepositoryError::Data)?;
    let expires_at_raw: String = row.get(3).contextualize(RepositoryError::Data)?;
    let spent_at_raw: Option<String> = row.get(4).contextualize(RepositoryError::Data)?;

    let id = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let konto_id = parse_uuid(&konto_id_raw).contextualize(RepositoryError::Data)?;
    let token_hash = TokenHash::from_hex(token_hash_raw).change_context(RepositoryError::Data)?;
    let expires_at = parse_datetime(&expires_at_raw).contextualize(RepositoryError::Data)?;
    let spent_at = spent_at_raw
        .map(|wert| parse_datetime(&wert).contextualize(RepositoryError::Data))
        .transpose()?;

    Ok(PasswortWiederherstellung::from_parts(
        WiederherstellungId(id),
        KontoId(konto_id),
        token_hash,
        expires_at,
        spent_at,
    ))
}

pub struct SQLitePasswortWiederherstellungRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl PasswortWiederherstellungRepository for SQLitePasswortWiederherstellungRepository {
    async fn create(
        &self,
        wiederherstellung: NeueWiederherstellung,
    ) -> RepositoryResult<PasswortWiederherstellung> {
        let id = WiederherstellungId(Uuid::new_v4());
        let domain = PasswortWiederherstellung::neu(id, wiederherstellung.clone());

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "passwort_wiederherstellungen" (id, konto_id, token_hash, expires_at, spent_at, _version)
               VALUES (?1, ?2, ?3, ?4, NULL, 0)"#,
            (
                id.0.to_string(),
                wiederherstellung.konto_id().0.to_string(),
                wiederherstellung.token_hash().as_str().to_string(),
                format_datetime(wiederherstellung.expires_at()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(domain)
    }

    async fn find_by_hash(
        &self,
        hash: &TokenHash,
    ) -> RepositoryResult<Option<PasswortWiederherstellung>> {
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

        wiederherstellung_from_row(&row).map(Some)
    }

    async fn consume(
        &self,
        id: WiederherstellungId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let betroffen = tx
            .execute(
                r#"UPDATE "passwort_wiederherstellungen" SET spent_at = ?2, _version = _version + 1
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

    async fn invalidate_open_for_konto(
        &self,
        konto_id: KontoId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"UPDATE "passwort_wiederherstellungen" SET spent_at = ?2, _version = _version + 1
               WHERE konto_id = ?1 AND spent_at IS NULL"#,
            (konto_id.0.to_string(), format_datetime(jetzt)),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
        Ok(())
    }

    async fn delete_alle_fuer_konto(&self, konto_id: KontoId) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"DELETE FROM "passwort_wiederherstellungen" WHERE konto_id = ?1"#,
            [konto_id.0.to_string()],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
        Ok(())
    }
}
