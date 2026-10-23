use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;

use logandaim_core::{
    ErrorReportExt,
    domain::{Schuss, SchussId, Wertungszone, ZielId},
    ports::{NeuerSchuss, RepositoryError, RepositoryResult, SchussRepository},
};

use crate::{errors::libsql_error_to_persistence_error, repos::common::parse_uuid};

/// Single source of truth for the stored spelling of a Wertungszone.
fn zone_to_str(zone: Wertungszone) -> &'static str {
    match zone {
        Wertungszone::Spot => "Spot",
        Wertungszone::Kill => "Kill",
        Wertungszone::Body => "Body",
        Wertungszone::Miss => "Miss",
    }
}

fn zone_from_str(value: &str) -> Result<Wertungszone, RepositoryError> {
    match value {
        "Spot" => Ok(Wertungszone::Spot),
        "Kill" => Ok(Wertungszone::Kill),
        "Body" => Ok(Wertungszone::Body),
        "Miss" => Ok(Wertungszone::Miss),
        _ => Err(RepositoryError::Data),
    }
}

fn schuss_from_row(row: &Row) -> RepositoryResult<Schuss> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let ziel_id_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let pfeil: i64 = row.get(2).contextualize(RepositoryError::Data)?;
    let zone_raw: String = row.get(3).contextualize(RepositoryError::Data)?;

    let id = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let ziel_id = parse_uuid(&ziel_id_raw).contextualize(RepositoryError::Data)?;
    let zone = zone_from_str(&zone_raw).contextualize(RepositoryError::Data)?;
    let pfeil = usize::try_from(pfeil).contextualize(RepositoryError::Data)?;

    Schuss::from_parts(SchussId(id), ZielId(ziel_id), pfeil, zone)
        .change_context(RepositoryError::Data)
}

pub struct SQLiteSchussRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl SchussRepository for SQLiteSchussRepository {
    async fn create(&self, schuss: NeuerSchuss) -> RepositoryResult<Schuss> {
        let id = SchussId(Uuid::new_v4());
        let domain = Schuss::neu(id, schuss.ziel_id, schuss.pfeil, schuss.wertungszone)
            .change_context(RepositoryError::Data)?;

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "schüsse" (id, ziel_id, pfeil, "wertungszone", _version) VALUES (?1, ?2, ?3, ?4, 0)"#,
            (
                id.0.to_string(),
                schuss.ziel_id.0.to_string(),
                schuss.pfeil as i64,
                zone_to_str(schuss.wertungszone),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(domain)
    }

    async fn find_by_id(&self, id: SchussId) -> RepositoryResult<Schuss> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                r#"SELECT id, ziel_id, pfeil, "wertungszone" FROM "schüsse" WHERE id = ?1"#,
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

        schuss_from_row(&row)
    }

    async fn find_by_ids(&self, ids: &[SchussId]) -> RepositoryResult<Vec<Schuss>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let placeholders = (1..=ids.len())
            .map(|index| format!("?{index}"))
            .collect::<Vec<_>>()
            .join(", ");
        let sql = format!(
            r#"SELECT id, ziel_id, pfeil, "wertungszone" FROM "schüsse" WHERE id IN ({placeholders})"#
        );

        let mut rows = tx
            .query(
                &sql,
                libsql::params_from_iter(ids.iter().map(|id| id.0.to_string())),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut schüsse = Vec::with_capacity(ids.len());
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            schüsse.push(schuss_from_row(&row)?);
        }

        Ok(schüsse)
    }
}
