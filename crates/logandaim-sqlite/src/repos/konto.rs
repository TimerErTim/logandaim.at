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
        konto::{Konto, KontoId, NeuesKonto, PasswordHash},
    },
    ports::{KontoRepository, RepositoryError, RepositoryResult},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{format_datetime, parse_datetime, parse_uuid},
};

fn konto_from_row(row: &Row) -> RepositoryResult<Konto> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let benutzer_id_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let email_raw: String = row.get(2).contextualize(RepositoryError::Data)?;
    let password_hash_raw: String = row.get(3).contextualize(RepositoryError::Data)?;
    let bestaetigt_am_raw: String = row.get(4).contextualize(RepositoryError::Data)?;
    let erstellt_am_raw: String = row.get(5).contextualize(RepositoryError::Data)?;

    let id = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let benutzer_id = parse_uuid(&benutzer_id_raw).contextualize(RepositoryError::Data)?;
    let email = EmailAdresse::new(email_raw).change_context(RepositoryError::Data)?;
    let password_hash =
        PasswordHash::from_phc(password_hash_raw).change_context(RepositoryError::Data)?;
    let bestaetigt_am = parse_datetime(&bestaetigt_am_raw).contextualize(RepositoryError::Data)?;
    let erstellt_am = parse_datetime(&erstellt_am_raw).contextualize(RepositoryError::Data)?;

    Ok(Konto::from_parts(
        KontoId(id),
        logandaim_core::domain::konto::BenutzerId(benutzer_id),
        email,
        password_hash,
        bestaetigt_am,
        erstellt_am,
    ))
}

const SELECT: &str =
    r#"SELECT id, benutzer_id, email, password_hash, bestaetigt_am, erstellt_am FROM "konten""#;

pub struct SQLiteKontoRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl KontoRepository for SQLiteKontoRepository {
    async fn create(&self, konto: NeuesKonto) -> RepositoryResult<Konto> {
        let id = KontoId(Uuid::new_v4());
        let domain = Konto::neu(id, konto.clone());

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "konten" (id, benutzer_id, email, password_hash, bestaetigt_am, erstellt_am, _version)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)"#,
            (
                id.0.to_string(),
                konto.benutzer_id().0.to_string(),
                konto.email().as_ref().to_string(),
                konto.password_hash().as_str().to_string(),
                format_datetime(konto.bestaetigt_am()),
                format_datetime(konto.erstellt_am()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(domain)
    }

    async fn find_by_id(&self, id: KontoId) -> RepositoryResult<Konto> {
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

    async fn find_by_email(&self, email: &EmailAdresse) -> RepositoryResult<Option<Konto>> {
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
        benutzer_id: logandaim_core::domain::konto::BenutzerId,
    ) -> RepositoryResult<Vec<Konto>> {
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

    async fn speichere(&self, konto: &Konto) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let betroffen = tx
            .execute(
                r#"UPDATE "konten" SET password_hash = ?2, _version = _version + 1 WHERE id = ?1"#,
                (
                    konto.id().0.to_string(),
                    konto.password_hash().as_str().to_string(),
                ),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if betroffen == 0 {
            return Err(RepositoryError::NotFound.into());
        }
        Ok(())
    }

    async fn delete(&self, id: KontoId) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let betroffen = tx
            .execute(r#"DELETE FROM "konten" WHERE id = ?1"#, [id.0.to_string()])
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if betroffen == 0 {
            return Err(RepositoryError::NotFound.into());
        }
        Ok(())
    }
}
