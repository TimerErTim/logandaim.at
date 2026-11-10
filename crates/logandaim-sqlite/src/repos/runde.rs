//! `Runde` rows, with the captured `ParkourSnapshot` stored as an adapter-local JSON document.
//!
//! The snapshot is serialised here and nowhere else: the domain has no serde dependency, and a
//! snapshot's shape is a storage detail. The history index (`runden_referenzen`) is written in
//! the same statement batch, so "does a Runde still reference this master data?" never needs to
//! parse a snapshot.

use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use libsql::{Row, Transaction};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use logandaim_core::{
    ErrorReportExt,
    application::uow::Versioned,
    domain::{
        Abstand, AnlageId, Erfasser, Farbe, NeueRunde, NonEmptyString, ParkourId, ParkourSnapshot,
        PflockSnapshot, Runde, RundeId, RundeInBearbeitung, ZielId, ZielPflöckeSnapshot,
        ZielSnapshot, konto::BenutzerId,
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
    parkour_id: Uuid,
    parkour_name: String,
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
    zielnummer: u32,
    /// Absent in a snapshot written before per-Ziel sets existed; such a snapshot inherits
    /// the Parkour-wide order, which is exactly what `Default` says.
    #[serde(default)]
    pflöcke: ZielPflöckeDto,
    abstände: Vec<(String, u32)>,
}

/// The stored mirror of `ZielPflöckeSnapshot`, externally tagged by serde.
///
/// `Default` is `InheritFromParkour`, so an old snapshot without the field keeps working and
/// reads back as the Parkour-wide set.
#[derive(Debug, Serialize, Deserialize, Default)]
enum ZielPflöckeDto {
    #[default]
    InheritFromParkour,
    Explicit(Vec<PflockDto>),
}

impl ZielPflöckeDto {
    fn aus_snapshot(snapshot: &ZielPflöckeSnapshot) -> Self {
        match snapshot.eigene() {
            None => Self::InheritFromParkour,
            Some(eigene) => Self::Explicit(
                eigene
                    .iter()
                    .map(|pflock| PflockDto {
                        farbe: pflock.farbe().to_hex(),
                        position: pflock.position(),
                    })
                    .collect(),
            ),
        }
    }
}

fn snapshot_to_json(snapshot: &ParkourSnapshot) -> RepositoryResult<String> {
    let dto = SnapshotDto {
        parkour_id: snapshot.parkour_id().0,
        parkour_name: snapshot.parkour_name().to_string(),
        pflöcke: snapshot
            .pflöcke()
            .iter()
            .map(|pflock| PflockDto {
                farbe: pflock.farbe().to_hex(),
                position: pflock.position(),
            })
            .collect(),
        ziele: snapshot
            .ziele()
            .iter()
            .map(|ziel| ZielDto {
                ziel_id: ziel.ziel_id().0,
                tiername: ziel.tiername().to_string(),
                zielnummer: ziel.zielnummer().get(),
                pflöcke: ZielPflöckeDto::aus_snapshot(ziel.pflöcke()),
                abstände: ziel
                    .abstände()
                    .iter()
                    .map(|(farbe, abstand)| (farbe.to_hex(), abstand.meter()))
                    .collect(),
            })
            .collect(),
    };
    serde_json::to_string(&dto).contextualize(RepositoryError::Data)
}

fn snapshot_from_json(json: &str) -> RepositoryResult<ParkourSnapshot> {
    let dto: SnapshotDto = serde_json::from_str(json).contextualize(RepositoryError::Data)?;
    let pflöcke = dto
        .pflöcke
        .into_iter()
        .map(|pflock| {
            Ok(PflockSnapshot::neu(
                (Farbe::neu(&pflock.farbe)).contextualize(RepositoryError::Data)?,
                pflock.position,
            ))
        })
        .collect::<RepositoryResult<Vec<_>>>()?;
    let ziele = dto
        .ziele
        .into_iter()
        .map(|ziel| {
            let abstände = ziel
                .abstände
                .into_iter()
                .map(|(farbe, meter)| {
                    Ok((
                        (Farbe::neu(&farbe)).contextualize(RepositoryError::Data)?,
                        (Abstand::neu(meter)).contextualize(RepositoryError::Data)?,
                    ))
                })
                .collect::<RepositoryResult<Vec<_>>>()?;
            let pflöcke = match ziel.pflöcke {
                ZielPflöckeDto::InheritFromParkour => ZielPflöckeSnapshot::InheritFromParkour,
                ZielPflöckeDto::Explicit(eigene) => ZielPflöckeSnapshot::Explicit(
                    eigene
                        .into_iter()
                        .map(|pflock| {
                            Ok(PflockSnapshot::neu(
                                (Farbe::neu(&pflock.farbe)).contextualize(RepositoryError::Data)?,
                                pflock.position,
                            ))
                        })
                        .collect::<RepositoryResult<Vec<_>>>()?,
                ),
            };
            Ok(ZielSnapshot::neu(
                ZielId(ziel.ziel_id),
                std::num::NonZeroU32::new(ziel.zielnummer).ok_or(RepositoryError::Data)?,
                (NonEmptyString::neu(&ziel.tiername)).contextualize(RepositoryError::Data)?,
                pflöcke,
                abstände,
            ))
        })
        .collect::<RepositoryResult<Vec<_>>>()?;

    Ok(ParkourSnapshot::from_parts(
        ParkourId(dto.parkour_id),
        (NonEmptyString::neu(&dto.parkour_name)).contextualize(RepositoryError::Data)?,
        pflöcke,
        ziele,
    ))
}

// ---------------------------------------------------------------------------------------
// Row mapping
// ---------------------------------------------------------------------------------------

const SELECT: &str = r#"SELECT id, anlage_id, parkour_id, snapshot, benutzer_id, anonymisiert_am,
            zustand, begonnen_am, abgeschlossen_am, _version
       FROM "runden""#;

fn zuordnung_aus_zeile(
    benutzer_id: Option<String>,
    anonymisiert_am: Option<String>,
) -> RepositoryResult<Erfasser> {
    match (benutzer_id, anonymisiert_am) {
        (Some(benutzer_id), None) => Ok(Erfasser::Benutzer(BenutzerId(parse_uuid(&benutzer_id)?))),
        (None, Some(am)) => Ok(Erfasser::Anonymisiert {
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
    let begonnen_am_raw: String = row.get(7).contextualize(RepositoryError::Data)?;
    let abgeschlossen_am: Option<String> = row.get(8).contextualize(RepositoryError::Data)?;

    let runde = Runde::from_parts(
        RundeId(parse_uuid(&id_raw)?),
        AnlageId(parse_uuid(&anlage_id_raw)?),
        zuordnung_aus_zeile(benutzer_id, anonymisiert_am)?,
        snapshot_from_json(&snapshot_raw)?,
        parse_datetime(&begonnen_am_raw)?,
        abgeschlossen_am.map(|am| parse_datetime(&am)).transpose()?,
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
async fn ersetze_referenzen(tx: &Transaction, runde: &Runde) -> RepositoryResult<()> {
    let runde_id = runde.id().0.to_string();
    tx.execute(
        r#"DELETE FROM "runden_referenzen" WHERE runde_id = ?1"#,
        [runde_id.clone()],
    )
    .await
    .contextualize_with(libsql_error_to_persistence_error)?;

    let mut referenzen: Vec<(&'static str, String)> = vec![
        ("anlage", runde.anlage_id().0.to_string()),
        ("parkour", runde.parkour_id().0.to_string()),
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
        .contextualize_with(libsql_error_to_persistence_error)?;
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
        let (benutzer_id, anonymisiert_am) = match runde.erfasst_von() {
            Erfasser::Benutzer(benutzer_id) => (Some(benutzer_id.0.to_string()), None),
            Erfasser::Anonymisiert { am } => (None, Some(format_datetime(am))),
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
                runde.zustand(),
                runde.abgeschlossen_am().map(format_datetime),
                erwartete_version,
            ),
            u64::try_from(erwartete_version).contextualize(RepositoryError::Data)?,
        )
        .await
    }
}

#[async_trait]
impl RundeRepository for SQLiteRundeRepository {
    async fn create(&self, runde: NeueRunde) -> RepositoryResult<Versioned<Runde>> {
        let id = RundeId(Uuid::new_v4());
        let domain = Runde::InBearbeitung(RundeInBearbeitung::neu(id, runde));

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let (benutzer_id, anonymisiert_am) = match domain.erfasst_von() {
            Erfasser::Benutzer(benutzer_id) => (Some(benutzer_id.0.to_string()), None),
            Erfasser::Anonymisiert { am } => (None, Some(format_datetime(am))),
        };

        tx.execute(
            r#"INSERT INTO "runden" (id, anlage_id, parkour_id, snapshot, benutzer_id,
                                     anonymisiert_am, zustand, begonnen_am, abgeschlossen_am,
                                     _version)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, 0)"#,
            (
                id.0.to_string(),
                domain.anlage_id().0.to_string(),
                domain.parkour_id().0.to_string(),
                snapshot_to_json(domain.snapshot())?,
                benutzer_id,
                anonymisiert_am,
                domain.zustand(),
                format_datetime(domain.begonnen_am()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        ersetze_referenzen(tx, &domain).await?;

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

    async fn find_offen_by_benutzer(
        &self,
        benutzer_id: BenutzerId,
    ) -> RepositoryResult<Versioned<Runde>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;
        let mut rows = tx
            .query(
                &format!(
                    r#"{SELECT} WHERE benutzer_id = ?1 AND zustand = 'InBearbeitung' LIMIT 1"#
                ),
                [benutzer_id.0.to_string()],
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
        runde_aus_zeile(&row)
    }

    async fn update(&self, runde: &mut Versioned<Runde>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        Self::schreibe_zeile(tx, runde, runde.v() as i64).await?;
        ersetze_referenzen(tx, runde).await?;

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

    async fn exists_referencing_parkour(&self, id: ParkourId) -> RepositoryResult<bool> {
        self.hat_referenz("parkour", &id.0.to_string()).await
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
