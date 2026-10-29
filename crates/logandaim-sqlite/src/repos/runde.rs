//! `Runde` rows, with the captured `ParcoursSnapshot` stored as an adapter-local JSON document.
//!
//! The snapshot is serialised here and nowhere else: the domain has no serde dependency, and a
//! snapshot's shape is a storage detail. The history index (`runden_referenzen`) is written in
//! the same statement batch, so "does a Runde still reference this master data?" never needs to
//! parse a snapshot.

use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::Report;
use libsql::{Row, Transaction};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use logandaim_core::{
    ErrorReportExt,
    application::uow::Versioned,
    domain::{
        AnlageId, NeueRunde, ParcoursId, ParcoursSnapshot, Runde, RundeId, RundeZustand, ZielId,
        Zuordnung, konto::BenutzerId,
    },
    ports::{RepositoryError, RepositoryResult, RundeRepository},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{
        delete_versioned, format_datetime, parse_datetime, parse_uuid, parse_version,
        update_versioned,
    },
};

// ---------------------------------------------------------------------------------------
// Snapshot DTO
// ---------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct SnapshotDto {
    parcours_id: Uuid,
    parcours_name: String,
    pflöcke: Vec<PflockDto>,
    ziele: Vec<ZielDto>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PflockDto {
    farbe: String,
    position: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct ZielDto {
    ziel_id: Uuid,
    tiername: String,
    zielgruppe: u8,
    abstände: Vec<(String, u32)>,
}

fn snapshot_to_json(snapshot: &ParcoursSnapshot) -> RepositoryResult<String> {
    let dto = SnapshotDto {
        parcours_id: snapshot.parcours_id().0,
        parcours_name: snapshot.parcours_name().to_owned(),
        pflöcke: snapshot
            .pflöcke()
            .iter()
            .map(|pflock| PflockDto {
                farbe: pflock.farbe().to_owned(),
                position: pflock.position(),
            })
            .collect(),
        ziele: snapshot
            .ziele()
            .iter()
            .map(|ziel| ZielDto {
                ziel_id: ziel.ziel_id().0,
                tiername: ziel.tiername().to_owned(),
                zielgruppe: ziel.zielgruppe(),
                abstände: ziel.abstände().to_vec(),
            })
            .collect(),
    };
    serde_json::to_string(&dto).map_err(|_| RepositoryError::Data.into())
}

fn snapshot_from_json(json: &str) -> RepositoryResult<ParcoursSnapshot> {
    let dto: SnapshotDto = serde_json::from_str(json).map_err(|_| RepositoryError::Data)?;
    ParcoursSnapshot::from_parts(
        dto.parcours_id,
        dto.parcours_name,
        dto.pflöcke
            .into_iter()
            .map(|pflock| (pflock.farbe, pflock.position))
            .collect(),
        dto.ziele
            .into_iter()
            .map(|ziel| (ziel.ziel_id, ziel.tiername, ziel.zielgruppe, ziel.abstände))
            .collect(),
    )
    .map_err(|_| RepositoryError::Data.into())
}

// ---------------------------------------------------------------------------------------
// Row mapping
// ---------------------------------------------------------------------------------------

const SELECT: &str = r#"SELECT id, anlage_id, parcours_id, snapshot, benutzer_id, anonymisiert_am,
            zustand, begonnen_am, abgeschlossen_am, _version
       FROM "runden""#;

fn zuordnung_aus_zeile(
    benutzer_id: Option<String>,
    anonymisiert_am: Option<String>,
) -> RepositoryResult<Zuordnung> {
    match (benutzer_id, anonymisiert_am) {
        (Some(benutzer_id), None) => Ok(Zuordnung::Benutzer(BenutzerId(parse_uuid(&benutzer_id)?))),
        (None, Some(am)) => Ok(Zuordnung::Anonymisiert {
            am: parse_datetime(&am)?,
        }),
        // Neither a live owner nor an anonymisation stamp: the retention rule cannot be read
        // back, so the row is corrupt rather than merely old.
        _ => Err(RepositoryError::Data.into()),
    }
}

fn runde_aus_zeile(row: &Row) -> RepositoryResult<Versioned<Runde>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let anlage_id_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let snapshot_raw: String = row.get(3).contextualize(RepositoryError::Data)?;
    let benutzer_id: Option<String> = row.get(4).contextualize(RepositoryError::Data)?;
    let anonymisiert_am: Option<String> = row.get(5).contextualize(RepositoryError::Data)?;
    let zustand_raw: String = row.get(6).contextualize(RepositoryError::Data)?;
    let begonnen_am_raw: String = row.get(7).contextualize(RepositoryError::Data)?;
    let abgeschlossen_am: Option<String> = row.get(8).contextualize(RepositoryError::Data)?;

    let runde = Runde::from_parts(
        RundeId(parse_uuid(&id_raw).contextualize(RepositoryError::Data)?),
        AnlageId(parse_uuid(&anlage_id_raw).contextualize(RepositoryError::Data)?),
        zuordnung_aus_zeile(benutzer_id, anonymisiert_am)?,
        snapshot_from_json(&snapshot_raw)?,
        zustand_raw
            .parse::<RundeZustand>()
            .map_err(|_| RepositoryError::Data)?,
        parse_datetime(&begonnen_am_raw).contextualize(RepositoryError::Data)?,
        abgeschlossen_am
            .map(|am| parse_datetime(&am).contextualize(RepositoryError::Data))
            .transpose()?,
    );

    Ok(Versioned::new(parse_version(row, 9)?, runde))
}

// ---------------------------------------------------------------------------------------
// History index
// ---------------------------------------------------------------------------------------

/// Record the master-data identities one Runde leans on, so AC-11/AC-12 is an index read.
///
/// `art` names the kind of reference; the ids are the snapshot's own copies, which is exactly
/// what "may this master data be deleted?" has to ask.
async fn ersetze_referenzen(tx: &Transaction, runde: &Runde) -> Result<(), RepositoryError> {
    let runde_id = runde.id().0.to_string();
    tx.execute(
        r#"DELETE FROM "runden_referenzen" WHERE runde_id = ?1"#,
        [runde_id.clone()],
    )
    .await
    .map_err(|_| RepositoryError::OperationFailed)?;

    let mut referenzen: Vec<(&'static str, String)> = vec![
        ("anlage", runde.anlage_id().0.to_string()),
        ("parcours", runde.parcours_id().0.to_string()),
    ];
    for ziel in runde.snapshot().ziele() {
        referenzen.push(("ziel", ziel.ziel_id().0.to_string()));
    }

    for (art, ref_id) in referenzen {
        tx.execute(
            r#"INSERT OR IGNORE INTO "runden_referenzen" (runde_id, art, ref_id)
               VALUES (?1, ?2, ?3)"#,
            (runde_id.clone(), art, ref_id),
        )
        .await
        .map_err(|_| RepositoryError::OperationFailed)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------
// Repository
// ---------------------------------------------------------------------------------------

pub struct SQLiteRundeRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

impl SQLiteRundeRepository {
    /// Write the mutable columns of one row. Shared by `update` so the insert and the write-back
    /// can never disagree about a column.
    async fn schreibe_zeile(
        tx: &Transaction,
        runde: &Runde,
        erwartete_version: i64,
    ) -> RepositoryResult<()> {
        let (benutzer_id, anonymisiert_am) = match runde.zuordnung() {
            Zuordnung::Benutzer(benutzer_id) => (Some(benutzer_id.0.to_string()), None),
            Zuordnung::Anonymisiert { am } => (None, Some(format_datetime(am))),
        };
        let snapshot = snapshot_to_json(runde.snapshot())?;
        update_versioned(
            tx,
            r#"UPDATE "runden" SET snapshot = ?2, benutzer_id = ?3, anonymisiert_am = ?4,
                   zustand = ?5, abgeschlossen_am = ?6, _version = ?7 + 1
               WHERE id = ?1 AND _version = ?7"#,
            (
                runde.id().0.to_string(),
                snapshot,
                benutzer_id,
                anonymisiert_am,
                runde.zustand().as_str(),
                runde.abgeschlossen_am().map(format_datetime),
                erwartete_version,
            ),
            u64::try_from(erwartete_version).map_err(|_| RepositoryError::Data)?,
        )
        .await
        .map_err(Report::new)
    }
}

#[async_trait]
impl RundeRepository for SQLiteRundeRepository {
    async fn create(&self, runde: NeueRunde) -> RepositoryResult<Versioned<Runde>> {
        let id = RundeId(Uuid::new_v4());
        let domain = Runde::neu(id, runde);

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let (benutzer_id, anonymisiert_am) = match domain.zuordnung() {
            Zuordnung::Benutzer(benutzer_id) => (Some(benutzer_id.0.to_string()), None),
            Zuordnung::Anonymisiert { am } => (None, Some(format_datetime(am))),
        };

        tx.execute(
            r#"INSERT INTO "runden" (id, anlage_id, parcours_id, snapshot, benutzer_id,
                                     anonymisiert_am, zustand, begonnen_am, abgeschlossen_am,
                                     _version)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, 0)"#,
            (
                id.0.to_string(),
                domain.anlage_id().0.to_string(),
                domain.parcours_id().0.to_string(),
                snapshot_to_json(domain.snapshot())?,
                benutzer_id,
                anonymisiert_am,
                domain.zustand().as_str(),
                format_datetime(domain.begonnen_am()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        ersetze_referenzen(tx, &domain).await.map_err(Report::new)?;

        Ok(Versioned::init(domain))
    }

    async fn find_by_id(&self, id: RundeId) -> RepositoryResult<Versioned<Runde>> {
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

        runde_aus_zeile(&row)
    }

    async fn find_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Runde>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                &format!(r#"{SELECT} WHERE benutzer_id = ?1 ORDER BY begonnen_am DESC"#),
                [benutzer_id.0.to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut runden = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            runden.push(runde_aus_zeile(&row)?);
        }
        Ok(runden)
    }

    async fn update(&self, runde: &mut Versioned<Runde>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        Self::schreibe_zeile(tx, runde, runde.v() as i64).await?;
        ersetze_referenzen(tx, runde).await.map_err(Report::new)?;

        runde.increment();
        Ok(())
    }

    async fn delete(&self, runde: Versioned<Runde>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        delete_versioned(
            tx,
            r#"DELETE FROM "runden" WHERE id = ?1 AND _version = ?2"#,
            (runde.id().0.to_string(), runde.v() as i64),
            runde.v(),
        )
        .await?;
        tx.execute(
            r#"DELETE FROM "runden_referenzen" WHERE runde_id = ?1"#,
            [runde.id().0.to_string()],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
        Ok(())
    }

    async fn exists_referencing_anlage(&self, id: AnlageId) -> RepositoryResult<bool> {
        self.hat_referenz("anlage", &id.0.to_string()).await
    }

    async fn exists_referencing_parcours(&self, id: ParcoursId) -> RepositoryResult<bool> {
        self.hat_referenz("parcours", &id.0.to_string()).await
    }

    async fn exists_referencing_ziel(&self, id: ZielId) -> RepositoryResult<bool> {
        self.hat_referenz("ziel", &id.0.to_string()).await
    }
}

impl SQLiteRundeRepository {
    async fn hat_referenz(&self, art: &str, ref_id: &str) -> RepositoryResult<bool> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                r#"SELECT 1 FROM "runden_referenzen" WHERE art = ?1 AND ref_id = ?2 LIMIT 1"#,
                (art, ref_id),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
            .is_some())
    }
}
