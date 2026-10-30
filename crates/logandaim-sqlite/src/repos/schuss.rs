//! `Schuss` rows: one logged arrow, its Runde, and its attribution.
//!
//! A Schuss is a standalone aggregate root, so it stores the links it needs (`runde_id`,
//! `ziel_id`) and its `Erfasser` as two nullable columns rather than joining to either. An
//! anonymised row keeps scoring but loses its owner, which is what the retention rule asks for.

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
        Erfasser, NeuerSchuss, RundeId, Schuss, SchussId, Wertungszone, ZielId, konto::BenutzerId,
    },
    ports::{RepositoryError, RepositoryResult, SchussRepository},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{
        delete_versioned, format_datetime, parse_datetime, parse_uuid, parse_version,
        update_versioned,
    },
};

const SELECT: &str = r#"SELECT id, runde_id, benutzer_id, anonymisiert_am, ziel_id, pfeil,
            "wertungszone", _version
       FROM "schüsse""#;

fn zone_from_str(value: &str) -> Result<Wertungszone, RepositoryError> {
    value.parse().map_err(|_| RepositoryError::Data)
}

fn schuss_aus_zeile(row: &Row) -> RepositoryResult<Versioned<Schuss>> {
    let id: String = row.get(0).contextualize(RepositoryError::Data)?;
    let runde_id: String = row.get(1).contextualize(RepositoryError::Data)?;
    let benutzer_id: Option<String> = row.get(2).contextualize(RepositoryError::Data)?;
    let anonymisiert_am: Option<String> = row.get(3).contextualize(RepositoryError::Data)?;
    let ziel_id: String = row.get(4).contextualize(RepositoryError::Data)?;
    let pfeil: i64 = row.get(5).contextualize(RepositoryError::Data)?;
    let zone: String = row.get(6).contextualize(RepositoryError::Data)?;

    let zuordnung = match (benutzer_id, anonymisiert_am) {
        (Some(benutzer_id), None) => Erfasser::Benutzer(BenutzerId(
            parse_uuid(&benutzer_id).contextualize(RepositoryError::Data)?,
        )),
        (None, Some(am)) => Erfasser::Anonymisiert {
            am: parse_datetime(&am).contextualize(RepositoryError::Data)?,
        },
        _ => return Err(RepositoryError::Data.into()),
    };

    Ok(Versioned::new(
        parse_version(row, 7)?,
        Schuss::from_parts(
            SchussId(parse_uuid(&id).contextualize(RepositoryError::Data)?),
            RundeId(parse_uuid(&runde_id).contextualize(RepositoryError::Data)?),
            zuordnung,
            ZielId(parse_uuid(&ziel_id).contextualize(RepositoryError::Data)?),
            usize::try_from(pfeil).contextualize(RepositoryError::Data)?,
            zone_from_str(&zone)?,
        )
        .change_context(RepositoryError::Data)?,
    ))
}

fn zuordnungsspalten(zuordnung: Erfasser) -> (Option<String>, Option<String>) {
    match zuordnung {
        Erfasser::Benutzer(benutzer_id) => (Some(benutzer_id.0.to_string()), None),
        Erfasser::Anonymisiert { am } => (None, Some(format_datetime(am))),
    }
}

pub struct SQLiteSchussRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

impl SQLiteSchussRepository {
    /// Load every row matching one `WHERE` clause, sharing the row mapping.
    async fn finde(
        &self,
        where_clause: &str,
        params: impl libsql::params::IntoParams,
    ) -> RepositoryResult<Vec<Versioned<Schuss>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(&format!(r#"{SELECT} {where_clause}"#), params)
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut schüsse = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            schüsse.push(schuss_aus_zeile(&row)?);
        }
        Ok(schüsse)
    }
}

#[async_trait]
impl SchussRepository for SQLiteSchussRepository {
    async fn create(&self, schuss: NeuerSchuss) -> RepositoryResult<Versioned<Schuss>> {
        let id = SchussId(Uuid::new_v4());
        let domain = Schuss::neu(id, schuss);
        let (benutzer_id, anonymisiert_am) = zuordnungsspalten(domain.erfasser());

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "schüsse" (id, runde_id, benutzer_id, anonymisiert_am, ziel_id, pfeil,
                                      "wertungszone", _version)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0)"#,
            (
                id.0.to_string(),
                domain.runde_id().0.to_string(),
                benutzer_id,
                anonymisiert_am,
                domain.ziel_id().0.to_string(),
                domain.pfeil().get() as i64,
                domain.wertungszone().as_str(),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(Versioned::init(domain))
    }

    async fn find_by_id(&self, id: SchussId) -> RepositoryResult<Versioned<Schuss>> {
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

        schuss_aus_zeile(&row)
    }

    async fn find_by_runde(&self, runde_id: RundeId) -> RepositoryResult<Vec<Versioned<Schuss>>> {
        self.finde(
            r#"WHERE runde_id = ?1 ORDER BY pfeil"#,
            [runde_id.0.to_string()],
        )
        .await
    }

    async fn find_by_ziel(&self, ziel_id: ZielId) -> RepositoryResult<Vec<Versioned<Schuss>>> {
        self.finde(
            r#"WHERE ziel_id = ?1 ORDER BY runde_id, pfeil"#,
            [ziel_id.0.to_string()],
        )
        .await
    }

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Schuss>>> {
        self.finde(
            r#"WHERE benutzer_id = ?1 ORDER BY runde_id, pfeil"#,
            [benutzer_id.0.to_string()],
        )
        .await
    }

    async fn update(&self, schuss: &mut Versioned<Schuss>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;
        let (benutzer_id, anonymisiert_am) = zuordnungsspalten(schuss.erfasser());

        update_versioned(
            tx,
            r#"UPDATE "schüsse" SET runde_id = ?2, benutzer_id = ?3, anonymisiert_am = ?4,
                   ziel_id = ?5, pfeil = ?6, "wertungszone" = ?7, _version = ?8 + 1
               WHERE id = ?1 AND _version = ?8"#,
            (
                schuss.id().0.to_string(),
                schuss.runde_id().0.to_string(),
                benutzer_id,
                anonymisiert_am,
                schuss.ziel_id().0.to_string(),
                schuss.pfeil().get() as i64,
                schuss.wertungszone().as_str(),
                schuss.v() as i64,
            ),
            schuss.v(),
        )
        .await?;

        schuss.increment();
        Ok(())
    }

    async fn delete(&self, schuss: Versioned<Schuss>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        delete_versioned(
            tx,
            r#"DELETE FROM "schüsse" WHERE id = ?1 AND _version = ?2"#,
            (schuss.id().0.to_string(), schuss.v() as i64),
            schuss.v(),
        )
        .await
    }
}
