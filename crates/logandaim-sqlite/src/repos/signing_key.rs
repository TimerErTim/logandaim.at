use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};

use logandaim_core::{
    ErrorReportExt,
    domain::konto::{SigningKey, SigningKeyId},
    ports::{RepositoryError, RepositoryResult, SigningKeyRepository},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{format_datetime, parse_datetime},
};

const SELECT: &str = r#"SELECT kid, öffentlicher_schlüssel, aktiv, erstellt_am, außer_betrieb_am FROM "signing_keys""#;

fn signing_key_from_row(row: &Row) -> RepositoryResult<SigningKey> {
    let kid_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let schlüssel_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let aktiv_raw: i64 = row.get(2).contextualize(RepositoryError::Data)?;
    let erstellt_am_raw: String = row.get(3).contextualize(RepositoryError::Data)?;
    let außer_betrieb_am_raw: Option<String> = row.get(4).contextualize(RepositoryError::Data)?;

    let kid = SigningKeyId::new(kid_raw).change_context(RepositoryError::Data)?;
    let erstellt_am = parse_datetime(&erstellt_am_raw).contextualize(RepositoryError::Data)?;
    let außer_betrieb_am = außer_betrieb_am_raw
        .map(|wert| parse_datetime(&wert).contextualize(RepositoryError::Data))
        .transpose()?;

    Ok(SigningKey::from_parts(
        kid,
        schlüssel_raw,
        aktiv_raw != 0,
        erstellt_am,
        außer_betrieb_am,
    ))
}

pub struct SQLiteSigningKeyRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl SigningKeyRepository for SQLiteSigningKeyRepository {
    async fn find_by_kid(&self, kid: &SigningKeyId) -> RepositoryResult<Option<SigningKey>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                &format!(r#"{SELECT} WHERE kid = ?1"#),
                [kid.as_str().to_string()],
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

        signing_key_from_row(&row).map(Some)
    }

    async fn ensure(&self, schlüssel: &SigningKey) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        // Idempotent across restarts: an equal row is left alone, a row with the same kid
        // but different material is a deployment error, not something to overwrite —
        // silently replacing the public key would invalidate every token already issued.
        let mut rows = tx
            .query(
                &format!(r#"{SELECT} WHERE kid = ?1"#),
                [schlüssel.kid().as_str().to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            let vorhanden = signing_key_from_row(&row)?;
            return if vorhanden == *schlüssel {
                Ok(())
            } else {
                Err(RepositoryError::Conflict.into())
            };
        }

        tx.execute(
            r#"INSERT INTO "signing_keys" (kid, öffentlicher_schlüssel, aktiv, erstellt_am, außer_betrieb_am, _version)
               VALUES (?1, ?2, ?3, ?4, ?5, 0)"#,
            (
                schlüssel.kid().as_str().to_string(),
                schlüssel.öffentlicher_schlüssel().to_string(),
                if schlüssel.ist_aktiv() { 1i64 } else { 0i64 },
                format_datetime(schlüssel.erstellt_am()),
                schlüssel.außer_betrieb_am().map(format_datetime),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
        Ok(())
    }

    async fn außer_betrieb_nehmen(
        &self,
        kid: &SigningKeyId,
        jetzt: chrono::DateTime<chrono::Utc>,
    ) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let betroffen = tx
            .execute(
                r#"UPDATE "signing_keys"
                   SET aktiv = 0, außer_betrieb_am = COALESCE(außer_betrieb_am, ?2), _version = _version + 1
                   WHERE kid = ?1"#,
                (kid.as_str().to_string(), format_datetime(jetzt)),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if betroffen == 0 {
            return Err(RepositoryError::NotFound.into());
        }
        Ok(())
    }
}
