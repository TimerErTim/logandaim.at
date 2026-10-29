use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;

use logandaim_core::{
    ErrorReportExt,
    application::uow::Versioned,
    domain::konto::{Benutzer, BenutzerId, NeuerBenutzer, Rolle},
    ports::{BenutzerRepository, RepositoryError, RepositoryResult},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{
        delete_versioned, format_datetime, parse_datetime, parse_uuid, parse_version,
        update_versioned,
    },
};

const SELECT: &str = r#"SELECT id, rolle, created_at, _version FROM "benutzer""#;

fn benutzer_from_row(row: &Row) -> RepositoryResult<Versioned<Benutzer>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let rolle_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let created_at_raw: String = row.get(2).contextualize(RepositoryError::Data)?;

    let id = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let rolle: Rolle = rolle_raw.parse().change_context(RepositoryError::Data)?;
    let created_at = parse_datetime(&created_at_raw).contextualize(RepositoryError::Data)?;

    Ok(Versioned::new(
        parse_version(row, 3)?,
        Benutzer::from_parts(BenutzerId(id), rolle, created_at),
    ))
}

pub struct SQLiteBenutzerRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl BenutzerRepository for SQLiteBenutzerRepository {
    async fn create(&self, benutzer: NeuerBenutzer) -> RepositoryResult<Versioned<Benutzer>> {
        let id = BenutzerId(Uuid::new_v4());
        let domain = Benutzer::neu(id, benutzer);

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "benutzer" (id, rolle, created_at, _version) VALUES (?1, ?2, ?3, 0)"#,
            (
                id.0.to_string(),
                domain.rolle().as_str(),
                format_datetime(domain.created_at()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(Versioned::init(domain))
    }

    async fn find_by_id(&self, id: BenutzerId) -> RepositoryResult<Versioned<Benutzer>> {
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

        benutzer_from_row(&row)
    }

    /// Full-row write-back with an in-place version bump; a stale version is a
    /// `VersionMismatch`, never a silent overwrite.
    async fn update(&self, benutzer: &mut Versioned<Benutzer>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        update_versioned(
            tx,
            r#"UPDATE "benutzer" SET rolle = ?2, created_at = ?3, _version = ?4 + 1
               WHERE id = ?1 AND _version = ?4"#,
            (
                benutzer.id().0.to_string(),
                benutzer.rolle().as_str(),
                format_datetime(benutzer.created_at()),
                benutzer.v() as i64,
            ),
            benutzer.v(),
        )
        .await?;

        benutzer.increment();
        Ok(())
    }

    /// Delete guarded by the version the caller holds.
    async fn delete(&self, benutzer: Versioned<Benutzer>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        delete_versioned(
            tx,
            r#"DELETE FROM "benutzer" WHERE id = ?1 AND _version = ?2"#,
            (benutzer.id().0.to_string(), benutzer.v() as i64),
            benutzer.v(),
        )
        .await
    }
}
