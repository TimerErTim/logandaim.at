use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;

use logandaim_core::{
    ErrorReportExt,
    domain::konto::{Benutzer, BenutzerId, NeuerBenutzer, Rolle},
    ports::{BenutzerRepository, RepositoryError, RepositoryResult},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{format_datetime, parse_datetime, parse_uuid},
};

fn benutzer_from_row(row: &Row) -> RepositoryResult<Benutzer> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let rolle_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let erstellt_am_raw: String = row.get(2).contextualize(RepositoryError::Data)?;

    let id = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let rolle: Rolle = rolle_raw.parse().change_context(RepositoryError::Data)?;
    let erstellt_am = parse_datetime(&erstellt_am_raw).contextualize(RepositoryError::Data)?;

    Ok(Benutzer::from_parts(BenutzerId(id), rolle, erstellt_am))
}

pub struct SQLiteBenutzerRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl BenutzerRepository for SQLiteBenutzerRepository {
    async fn create(&self, benutzer: NeuerBenutzer) -> RepositoryResult<Benutzer> {
        let id = BenutzerId(Uuid::new_v4());
        let domain = Benutzer::neu(id, benutzer);

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "benutzer" (id, rolle, erstellt_am, _version) VALUES (?1, ?2, ?3, 0)"#,
            (
                id.0.to_string(),
                domain.rolle().as_str(),
                format_datetime(domain.erstellt_am()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(domain)
    }

    async fn find_by_id(&self, id: BenutzerId) -> RepositoryResult<Benutzer> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                r#"SELECT id, rolle, erstellt_am FROM "benutzer" WHERE id = ?1"#,
                [id.0.to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        else {
            return Err(RepositoryError::NotFound.into());
        };

        benutzer_from_row(&row)
    }

    async fn setze_rolle(&self, id: BenutzerId, rolle: Rolle) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let betroffen = tx
            .execute(
                r#"UPDATE "benutzer" SET rolle = ?2, _version = _version + 1 WHERE id = ?1"#,
                (id.0.to_string(), rolle.as_str()),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if betroffen == 0 {
            return Err(RepositoryError::NotFound.into());
        }
        Ok(())
    }

    async fn delete(&self, id: BenutzerId) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let betroffen = tx
            .execute(
                r#"DELETE FROM "benutzer" WHERE id = ?1"#,
                [id.0.to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if betroffen == 0 {
            return Err(RepositoryError::NotFound.into());
        }
        Ok(())
    }
}
