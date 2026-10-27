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
        konto::{BenutzerId, Konto, KontoId, NeuesKonto, PasswordHash},
    },
    ports::{KontoRepository, RepositoryError, RepositoryResult},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{
        delete_versioned, format_datetime, parse_datetime, parse_uuid, parse_version,
        update_versioned,
    },
};

const SELECT: &str = r#"SELECT id, benutzer_id, email, password_hash, bestätigt_am, created_at,
    _version FROM "konten""#;

fn konto_from_row(row: &Row) -> RepositoryResult<Versioned<Konto>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let benutzer_id_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let email_raw: String = row.get(2).contextualize(RepositoryError::Data)?;
    let password_hash_raw: String = row.get(3).contextualize(RepositoryError::Data)?;
    let bestätigt_am_raw: String = row.get(4).contextualize(RepositoryError::Data)?;
    let created_at_raw: String = row.get(5).contextualize(RepositoryError::Data)?;

    let id = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let benutzer_id = parse_uuid(&benutzer_id_raw).contextualize(RepositoryError::Data)?;
    // The one constructor: a stored address is normalised on the way in too.
    let email = EmailAdresse::normalisiert(email_raw).change_context(RepositoryError::Data)?;
    let password_hash =
        PasswordHash::from_phc(password_hash_raw).change_context(RepositoryError::Data)?;
    let bestätigt_am = parse_datetime(&bestätigt_am_raw).contextualize(RepositoryError::Data)?;
    let created_at = parse_datetime(&created_at_raw).contextualize(RepositoryError::Data)?;

    Ok(Versioned::new(
        parse_version(row, 6)?,
        Konto::from_parts(
            KontoId(id),
            BenutzerId(benutzer_id),
            email,
            password_hash,
            bestätigt_am,
            created_at,
        ),
    ))
}

pub struct SQLiteKontoRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl KontoRepository for SQLiteKontoRepository {
    async fn create(&self, konto: NeuesKonto) -> RepositoryResult<Versioned<Konto>> {
        let id = KontoId(Uuid::new_v4());
        let domain = Konto::neu(id, konto.clone());

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "konten" (id, benutzer_id, email, password_hash, bestätigt_am, created_at, _version)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)"#,
            (
                id.0.to_string(),
                konto.benutzer_id().0.to_string(),
                konto.email().as_ref().to_string(),
                konto.password_hash().as_str().to_string(),
                format_datetime(konto.bestätigt_am()),
                format_datetime(konto.created_at()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(Versioned::init(domain))
    }

    async fn find_by_id(&self, id: KontoId) -> RepositoryResult<Versioned<Konto>> {
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

        konto_from_row(&row)
    }

    async fn find_by_email(
        &self,
        email: &EmailAdresse,
    ) -> RepositoryResult<Option<Versioned<Konto>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                &format!(r#"{SELECT} WHERE email = ?1"#),
                [email.as_ref().to_string()],
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

        konto_from_row(&row).map(Some)
    }

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Konto>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                &format!(r#"{SELECT} WHERE benutzer_id = ?1"#),
                [benutzer_id.0.to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut konten = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            konten.push(konto_from_row(&row)?);
        }
        Ok(konten)
    }

    /// Full-row write-back with an in-place version bump; a stale version is a
    /// `VersionMismatch`, never a silent overwrite.
    async fn update(&self, konto: &mut Versioned<Konto>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        update_versioned(
            tx,
            r#"UPDATE "konten" SET benutzer_id = ?2, email = ?3, password_hash = ?4,
                   bestätigt_am = ?5, created_at = ?6, _version = ?7 + 1
               WHERE id = ?1 AND _version = ?7"#,
            (
                konto.id().0.to_string(),
                konto.benutzer_id().0.to_string(),
                konto.email().as_ref().to_string(),
                konto.password_hash().as_str().to_string(),
                format_datetime(konto.bestätigt_am()),
                format_datetime(konto.created_at()),
                konto.v() as i64,
            ),
            konto.v(),
        )
        .await?;

        konto.increment();
        Ok(())
    }

    /// Delete guarded by the version the caller holds.
    async fn delete(&self, konto: Versioned<Konto>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        delete_versioned(
            tx,
            r#"DELETE FROM "konten" WHERE id = ?1 AND _version = ?2"#,
            (konto.id().0.to_string(), konto.v() as i64),
            konto.v(),
        )
        .await
    }
}
