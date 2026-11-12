//! `Runde` rows, with the captured setup stored as an adapter-local JSON document.
//!
//! The setup is serialised here and nowhere else: the domain has no serde dependency, and a
//! setup's shape is a storage detail. The document is an **untagged** enum, so a Parkour round
//! stores its Anlage and Parkour ids inside one object and a free round stores neither — the
//! JSON itself cannot hold "parkour without anlage", and the read path takes both ids from the
//! document, never from the denormalised columns (SPEC-runde rule 17).
//!
//! The `runden.anlage_id`/`parkour_id` columns are written **from** that document and are only
//! a read projection for ad-hoc SQL; a hand-edited column therefore cannot produce a pair the
//! application would believe. The history index (`runden_referenzen`) is written in the same
//! statement batch, so "does a Runde still reference this master data?" never parses a document.

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
        Abstand, AnlageId, AnlageZielId, Erfasser, Farbe, NeueRunde, NonEmptyString, ParkourId,
        PflockSnapshot, Runde, RundeId, RundenZiel, RundenZielId, Rundenaufbau, Rundenherkunft,
        ZUSTAND_IN_BEARBEITUNG, ZielPflöckeSnapshot, konto::BenutzerId,
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
// Setup DTO
// ---------------------------------------------------------------------------------------

/// The stored setup: one object for a Parkour round, one for a free round.
///
/// Untagged, so the document itself is the discriminant: a Parkour round's JSON carries the
/// Anlage and Parkour pair in a single object (`parkour_id` present implies `anlage_id`), and a
/// free round's JSON carries neither. No field can disagree with another.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum RundenaufbauDto {
    Parkour {
        anlage_id: Uuid,
        parkour_id: Uuid,
        parkour_name: String,
        pflöcke: Vec<PflockDto>,
        ziele: Vec<ZielDto>,
    },
    Freie {
        name: String,
        ziele: Vec<ZielDto>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct PflockDto {
    farbe: String,
    position: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct ZielDto {
    ziel_id: Uuid,
    #[serde(default)]
    anlage_ziel_id: Option<Uuid>,
    #[serde(alias = "name")]
    tiername: String,
    #[serde(alias = "ordinal")]
    zielnummer: u32,
    #[serde(default)]
    pflöcke: ZielPflöckeDto,
    #[serde(default)]
    abstände: Vec<(String, u32)>,
}

/// The stored mirror of `ZielPflöckeSnapshot`, externally tagged by serde.
///
/// `Default` is `InheritFromParkour` for constructing new DTOs where applicable.
#[derive(Debug, Serialize, Deserialize, Default)]
enum ZielPflöckeDto {
    #[default]
    InheritFromParkour,
    Explicit(Vec<PflockDto>),
}

impl ZielPflöckeDto {
    fn aus_snapshot(snapshot: &ZielPflöckeSnapshot) -> Self {
        match snapshot {
            ZielPflöckeSnapshot::InheritFromParkour => Self::InheritFromParkour,
            ZielPflöckeSnapshot::Explicit(eigene) => Self::Explicit(
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

fn parkour_to_dto(
    anlage_id: AnlageId,
    parkour_id: ParkourId,
    parkour_name: &NonEmptyString,
    aufbau: &Rundenaufbau,
) -> RundenaufbauDto {
    RundenaufbauDto::Parkour {
        anlage_id: anlage_id.0,
        parkour_id: parkour_id.0,
        parkour_name: parkour_name.to_string(),
        pflöcke: aufbau
            .pflöcke()
            .iter()
            .map(|pflock| PflockDto {
                farbe: pflock.farbe().to_hex(),
                position: pflock.position(),
            })
            .collect(),
        ziele: aufbau
            .ziele()
            .iter()
            .map(|ziel| ZielDto {
                ziel_id: ziel.id().0,
                anlage_ziel_id: ziel.anlage_ziel_id().map(|id| id.0),
                tiername: ziel.name().to_string(),
                zielnummer: ziel.ordinal().get(),
                pflöcke: ZielPflöckeDto::aus_snapshot(ziel.pflöcke()),
                abstände: ziel
                    .abstände()
                    .iter()
                    .map(|(farbe, abstand)| (farbe.to_hex(), abstand.meter()))
                    .collect(),
            })
            .collect(),
    }
}

/// Serialise one Runde's document: its provenance and its route, in one object.
fn aufbau_to_json(herkunft: &Rundenherkunft, aufbau: &Rundenaufbau) -> RepositoryResult<String> {
    let dto = match herkunft {
        Rundenherkunft::Parkour {
            anlage_id,
            parkour_id,
            parkour_name,
        } => parkour_to_dto(*anlage_id, *parkour_id, parkour_name, aufbau),
        Rundenherkunft::Freie { name } => RundenaufbauDto::Freie {
            name: name.to_string(),
            ziele: aufbau
                .ziele()
                .iter()
                .map(|ziel| ZielDto {
                    ziel_id: ziel.id().0,
                    // A free target has no master-data identity to carry: the round minted
                    // its own. Writing one here would let a free target be read back wearing
                    // Anlage provenance that `ersetze_referenzen` never indexes.
                    anlage_ziel_id: None,
                    tiername: ziel.name().to_string(),
                    zielnummer: ziel.ordinal().get(),
                    pflöcke: ZielPflöckeDto::aus_snapshot(ziel.pflöcke()),
                    abstände: ziel
                        .abstände()
                        .iter()
                        .map(|(farbe, abstand)| (farbe.to_hex(), abstand.meter()))
                        .collect(),
                })
                .collect(),
        },
    };
    serde_json::to_string(&dto).contextualize(RepositoryError::Data)
}

/// The two denormalised projection columns, derived from the round's provenance.
///
/// `None` for both on a free round: SQL that filters by Anlage must not see one.
fn projizierte_ids(herkunft: &Rundenherkunft) -> (Option<String>, Option<String>) {
    match herkunft {
        Rundenherkunft::Parkour {
            anlage_id,
            parkour_id,
            ..
        } => (
            Some(anlage_id.0.to_string()),
            Some(parkour_id.0.to_string()),
        ),
        Rundenherkunft::Freie { .. } => (None, None),
    }
}

/// Read one stored document back into the provenance and the route it captured.
fn aufbau_from_json(json: &str) -> RepositoryResult<(Rundenherkunft, Rundenaufbau)> {
    let dto: RundenaufbauDto = serde_json::from_str(json).contextualize(RepositoryError::Data)?;
    match dto {
        RundenaufbauDto::Parkour {
            anlage_id,
            parkour_id,
            parkour_name,
            pflöcke,
            ziele,
        } => {
            let pflöcke = pflöcke
                .into_iter()
                .map(|pflock| {
                    Ok(PflockSnapshot::neu(
                        (Farbe::neu(&pflock.farbe)).contextualize(RepositoryError::Data)?,
                        pflock.position,
                    ))
                })
                .collect::<RepositoryResult<Vec<_>>>()?;
            let ziele = ziele
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
                        ZielPflöckeDto::InheritFromParkour => {
                            ZielPflöckeSnapshot::InheritFromParkour
                        }
                        ZielPflöckeDto::Explicit(eigene) => ZielPflöckeSnapshot::Explicit(
                            eigene
                                .into_iter()
                                .map(|pflock| {
                                    Ok(PflockSnapshot::neu(
                                        (Farbe::neu(&pflock.farbe))
                                            .contextualize(RepositoryError::Data)?,
                                        pflock.position,
                                    ))
                                })
                                .collect::<RepositoryResult<Vec<_>>>()?,
                        ),
                    };
                    let id = RundenZielId(ziel.ziel_id);
                    let name = (NonEmptyString::neu(&ziel.tiername))
                        .contextualize(RepositoryError::Data)?;
                    let ordinal =
                        std::num::NonZeroU32::new(ziel.zielnummer).ok_or(RepositoryError::Data)?;
                    Ok(RundenZiel::from_parts(
                        id,
                        ziel.anlage_ziel_id.map(AnlageZielId),
                        name,
                        ordinal,
                        pflöcke,
                        abstände,
                    ))
                })
                .collect::<RepositoryResult<Vec<_>>>()?;

            Ok((
                Rundenherkunft::Parkour {
                    anlage_id: AnlageId(anlage_id),
                    parkour_id: ParkourId(parkour_id),
                    parkour_name: (NonEmptyString::neu(&parkour_name))
                        .contextualize(RepositoryError::Data)?,
                },
                Rundenaufbau::from_parts(pflöcke, ziele),
            ))
        }
        RundenaufbauDto::Freie { name, ziele } => {
            let ziele = ziele
                .into_iter()
                .map(|ziel| {
                    let pflöcke = match ziel.pflöcke {
                        ZielPflöckeDto::InheritFromParkour => {
                            ZielPflöckeSnapshot::InheritFromParkour
                        }
                        ZielPflöckeDto::Explicit(pflöcke) => ZielPflöckeSnapshot::Explicit(
                            pflöcke
                                .into_iter()
                                .map(|pflock| {
                                    Ok(PflockSnapshot::neu(
                                        (Farbe::neu(&pflock.farbe))
                                            .contextualize(RepositoryError::Data)?,
                                        pflock.position,
                                    ))
                                })
                                .collect::<RepositoryResult<Vec<_>>>()?,
                        ),
                    };
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
                    // The Freie DTO carries no Anlage identity, so a stored one is corrupt
                    // rather than provenance: refuse it instead of restoring it.
                    if ziel.anlage_ziel_id.is_some() {
                        return Err(RepositoryError::Data.into());
                    }
                    Ok(RundenZiel::from_parts(
                        RundenZielId(ziel.ziel_id),
                        None,
                        (NonEmptyString::neu(&ziel.tiername))
                            .contextualize(RepositoryError::Data)?,
                        std::num::NonZeroU32::new(ziel.zielnummer).ok_or(RepositoryError::Data)?,
                        pflöcke,
                        abstände,
                    ))
                })
                .collect::<RepositoryResult<Vec<_>>>()?;
            Ok((
                Rundenherkunft::Freie {
                    name: (NonEmptyString::neu(&name)).contextualize(RepositoryError::Data)?,
                },
                Rundenaufbau::from_parts(Vec::new(), ziele),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ac21_snapshot_mit_anlage_und_parkour_ist_lesbar() {
        let snapshot = r#"{
            "anlage_id":"00000000-0000-0000-0000-000000000001",
            "parkour_id":"00000000-0000-0000-0000-000000000002",
            "parkour_name":"Wald",
            "pflöcke":[],
            "ziele":[]
        }"#;

        let (herkunft, aufbau) = aufbau_from_json(snapshot).unwrap();
        let Rundenherkunft::Parkour {
            anlage_id,
            parkour_id,
            ..
        } = &herkunft
        else {
            panic!("parkour setup");
        };
        assert_eq!(
            anlage_id.0.to_string(),
            "00000000-0000-0000-0000-000000000001"
        );
        assert_eq!(
            parkour_id.0.to_string(),
            "00000000-0000-0000-0000-000000000002"
        );
        assert!(aufbau.ziele().is_empty());
    }

    #[test]
    fn ac21_legacy_snapshot_without_embedded_anlage_id_is_rejected() {
        let legacy_snapshot = r#"{
            "parkour_id":"00000000-0000-0000-0000-000000000002",
            "parkour_name":"Wald",
            "pflöcke":[],
            "ziele":[]
        }"#;

        assert!(aufbau_from_json(legacy_snapshot).is_err());
    }
}

// ---------------------------------------------------------------------------------------
// Row mapping
// ---------------------------------------------------------------------------------------

const SELECT: &str = r#"SELECT id, snapshot, benutzer_id, anonymisiert_am, zustand,
            begonnen_am, abgeschlossen_am, _version
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
    let snapshot_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let benutzer_id: Option<String> = row.get(2).contextualize(RepositoryError::Data)?;
    let anonymisiert_am: Option<String> = row.get(3).contextualize(RepositoryError::Data)?;
    let zustand_raw: String = row.get(4).contextualize(RepositoryError::Data)?;
    let begonnen_am_raw: String = row.get(5).contextualize(RepositoryError::Data)?;
    let abgeschlossen_am: Option<String> = row.get(6).contextualize(RepositoryError::Data)?;

    let (herkunft, aufbau) = aufbau_from_json(&snapshot_raw)?;
    let runde = Runde::from_parts(
        RundeId(parse_uuid(&id_raw)?),
        zuordnung_aus_zeile(benutzer_id, anonymisiert_am)?,
        herkunft,
        aufbau,
        parse_datetime(&begonnen_am_raw)?,
        abgeschlossen_am.map(|am| parse_datetime(&am)).transpose()?,
    );

    // `zustand` is a stored projection of the snapshot's own typestate, used by the
    // "open round" query. A row where the two disagree cannot be read back into a
    // coherent aggregate, so it is corrupt data rather than a round that looks open.
    if zustand_raw != runde.zustand() {
        return Err(RepositoryError::Data.into());
    }

    Ok(Versioned::new(parse_version(row, 7)?, runde))
}

// ---------------------------------------------------------------------------------------
// History index
// ---------------------------------------------------------------------------------------

/// Record the master-data identities one Runde leans on, so AC-11/AC-12 is an index read.
///
/// `art` names the kind of reference; the ids are the snapshot's own copies, which is exactly
/// what "may this master data be deleted?" has to ask. A free round references no master data
/// at all — its Ziele are round-local identities the round minted itself, not `AnlageZielId`s
/// — so it records no rows here. That is why deleting an Anlage never reaches a free round.
async fn ersetze_referenzen(tx: &Transaction, runde: &Runde) -> RepositoryResult<()> {
    let runde_id = runde.id().0.to_string();
    tx.execute(
        r#"DELETE FROM "runden_referenzen" WHERE runde_id = ?1"#,
        [runde_id.clone()],
    )
    .await
    .contextualize_with(libsql_error_to_persistence_error)?;

    let mut referenzen: Vec<(&'static str, String)> = Vec::new();
    if let Rundenherkunft::Parkour {
        anlage_id,
        parkour_id,
        ..
    } = runde.herkunft()
    {
        referenzen.push(("anlage", anlage_id.0.to_string()));
        referenzen.push(("parkour", parkour_id.0.to_string()));
        for ziel in runde.aufbau().ziele() {
            if let Some(anlage_ziel_id) = ziel.anlage_ziel_id() {
                referenzen.push(("ziel", anlage_ziel_id.0.to_string()));
            }
        }
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
        let snapshot = aufbau_to_json(runde.herkunft(), runde.aufbau())?;
        let (anlage_id, parkour_id) = projizierte_ids(runde.herkunft());
        update_versioned(
            tx,
            r#"UPDATE "runden" SET anlage_id = ?2, parkour_id = ?3, snapshot = ?4,
                   benutzer_id = ?5, anonymisiert_am = ?6, zustand = ?7, abgeschlossen_am = ?8,
                   _version = ?9 + 1
               WHERE id = ?1 AND _version = ?9"#,
            (
                runde.id().0.to_string(),
                anlage_id,
                parkour_id,
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
        let domain = Runde::aus_neu(id, runde);

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let (benutzer_id, anonymisiert_am) = match domain.erfasst_von() {
            Erfasser::Benutzer(benutzer_id) => (Some(benutzer_id.0.to_string()), None),
            Erfasser::Anonymisiert { am } => (None, Some(format_datetime(am))),
        };
        let (anlage_id, parkour_id) = projizierte_ids(domain.herkunft());

        tx.execute(
            r#"INSERT INTO "runden" (id, anlage_id, parkour_id, snapshot, benutzer_id,
                                     anonymisiert_am, zustand, begonnen_am, abgeschlossen_am,
                                     _version)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL, 0)"#,
            (
                id.0.to_string(),
                anlage_id,
                parkour_id,
                aufbau_to_json(domain.herkunft(), domain.aufbau())?,
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
        // One spelling, shared with the domain markers: the partial unique index
        // `idx_runden_offen_pro_benutzer` filters on the same literal.
        let mut rows = tx
            .query(
                &format!(r#"{SELECT} WHERE benutzer_id = ?1 AND zustand = ?2 LIMIT 1"#),
                (benutzer_id.0.to_string(), ZUSTAND_IN_BEARBEITUNG),
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

    async fn exists_referencing_ziel(&self, id: AnlageZielId) -> RepositoryResult<bool> {
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
