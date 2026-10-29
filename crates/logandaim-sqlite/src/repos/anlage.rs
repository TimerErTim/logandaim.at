//! The `Anlage` aggregate with its `Ziel`, `Parcours`, Pflock, and placement children.
//!
//! One aggregate, one repository: `Ziel` and `Parcours` are children, so a child write is a
//! replace inside the root's transaction, guarded by the root `_version`. That is deliberate —
//! two concurrent master-data edits to the same Anlage conflict rather than silently overwrite
//! each other's children.

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
        Anlage, AnlageAdresse, AnlageAktiv, AnlageArchiviert, AnlageId, AnlageKontakt, AnlageName,
        AnlageVerwaltung, AnlageVerwaltungRolle, Ländercode, NeueAnlage, Parcours, ParcoursId,
        ParcoursName, Pflock, PflockFarbe, PflockId, Tiername, Ziel, ZielId, Zielgruppe,
        Zielplatzierung, Zustand,
    },
    ports::{AnlageRepository, RepositoryError, RepositoryResult},
};

use crate::{
    errors::libsql_error_to_persistence_error,
    repos::common::{
        delete_versioned, format_datetime, parse_datetime, parse_uuid, parse_version,
        update_versioned,
    },
};

const SELECT: &str = r#"SELECT id, zustand, name, strasse, postleitzahl, ort, laendercode,
            kontakt_name, kontakt_email, kontakt_telefon, kontakt_website, deaktiviert_am, _version
       FROM "anlagen""#;

fn zustand_to_str(zustand: Zustand) -> &'static str {
    zustand.as_str()
}

fn zustand_from_str(value: &str) -> Result<Zustand, RepositoryError> {
    value.parse().map_err(|_| RepositoryError::Data)
}

fn rolle_from_str(value: &str) -> Result<AnlageVerwaltungRolle, RepositoryError> {
    value.parse().map_err(|_| RepositoryError::Data)
}

/// Load the children of one Anlage. Each query is scoped by the root id, so the aggregate is
/// assembled from four small reads rather than a join that would duplicate every child.
async fn lade_verwaltungen(
    tx: &Transaction,
    anlage_id: AnlageId,
) -> Result<Vec<AnlageVerwaltung>, RepositoryError> {
    let mut rows = tx
        .query(
            r#"SELECT benutzer_id, rolle FROM "anlage_verwaltungen" WHERE anlage_id = ?1
               ORDER BY rolle, benutzer_id"#,
            [anlage_id.0.to_string()],
        )
        .await
        .map_err(|_| RepositoryError::OperationFailed)?;

    let mut verwaltungen = Vec::new();
    while let Some(row) = rows
        .next()
        .await
        .map_err(|_| RepositoryError::OperationFailed)?
    {
        let benutzer_id: String = row.get(0).map_err(|_| RepositoryError::Data)?;
        let rolle: String = row.get(1).map_err(|_| RepositoryError::Data)?;
        verwaltungen.push(AnlageVerwaltung::neu(
            logandaim_core::domain::konto::BenutzerId(parse_uuid(&benutzer_id)?),
            rolle_from_str(&rolle)?,
        ));
    }
    Ok(verwaltungen)
}

async fn lade_ziele(tx: &Transaction, anlage_id: AnlageId) -> RepositoryResult<Vec<Ziel>> {
    let mut rows = tx
        .query(
            r#"SELECT id, tiername, zielgruppe, zustand FROM "ziele"
               WHERE anlage_id = ?1 ORDER BY ordnung"#,
            [anlage_id.0.to_string()],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

    let mut ziele = Vec::new();
    while let Some(row) = rows
        .next()
        .await
        .contextualize_with(libsql_error_to_persistence_error)?
    {
        let id: String = row.get(0).contextualize(RepositoryError::Data)?;
        let tiername: String = row.get(1).contextualize(RepositoryError::Data)?;
        let zielgruppe: i64 = row.get(2).contextualize(RepositoryError::Data)?;
        let zustand: String = row.get(3).contextualize(RepositoryError::Data)?;
        let zielgruppe = u8::try_from(zielgruppe).contextualize(RepositoryError::Data)?;
        ziele.push(Ziel::from_parts(
            ZielId(parse_uuid(&id).contextualize(RepositoryError::Data)?),
            Tiername::neu(&tiername).change_context(RepositoryError::Data)?,
            Zielgruppe::neu(zielgruppe).change_context(RepositoryError::Data)?,
            zustand_from_str(&zustand)?,
        ));
    }
    Ok(ziele)
}

async fn lade_parcours(tx: &Transaction, anlage_id: AnlageId) -> RepositoryResult<Vec<Parcours>> {
    let mut rows = tx
        .query(
            r#"SELECT id, name, zustand FROM "parcours" WHERE anlage_id = ?1 ORDER BY ordnung"#,
            [anlage_id.0.to_string()],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

    let mut parcours = Vec::new();
    while let Some(row) = rows
        .next()
        .await
        .contextualize_with(libsql_error_to_persistence_error)?
    {
        let id: String = row.get(0).contextualize(RepositoryError::Data)?;
        let name: String = row.get(1).contextualize(RepositoryError::Data)?;
        let zustand: String = row.get(2).contextualize(RepositoryError::Data)?;
        let parcours_id = ParcoursId(parse_uuid(&id).contextualize(RepositoryError::Data)?);
        let pflöcke = lade_pflöcke(tx, parcours_id).await?;
        let platzierungen = lade_platzierungen(tx, parcours_id).await?;
        parcours.push(
            Parcours::from_parts(
                parcours_id,
                ParcoursName::neu(&name).change_context(RepositoryError::Data)?,
                pflöcke,
                platzierungen,
                zustand_from_str(&zustand)?,
            )
            .change_context(RepositoryError::Data)?,
        );
    }
    Ok(parcours)
}

async fn lade_pflöcke(tx: &Transaction, parcours_id: ParcoursId) -> RepositoryResult<Vec<Pflock>> {
    let mut rows = tx
        .query(
            r#"SELECT id, farbe FROM "pflock_setups"
               WHERE parcours_id = ?1 ORDER BY ordnung"#,
            [parcours_id.0.to_string()],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

    let mut pflöcke = Vec::new();
    while let Some(row) = rows
        .next()
        .await
        .contextualize_with(libsql_error_to_persistence_error)?
    {
        let id: String = row.get(0).contextualize(RepositoryError::Data)?;
        let farbe: String = row.get(1).contextualize(RepositoryError::Data)?;
        pflöcke.push(Pflock::neu(
            PflockId(parse_uuid(&id).contextualize(RepositoryError::Data)?),
            PflockFarbe::neu(&farbe).change_context(RepositoryError::Data)?,
        ));
    }
    Ok(pflöcke)
}

async fn lade_platzierungen(
    tx: &Transaction,
    parcours_id: ParcoursId,
) -> RepositoryResult<Vec<Zielplatzierung>> {
    let mut rows = tx
        .query(
            r#"SELECT ziel_id FROM "zielplatzierungen" WHERE parcours_id = ?1 ORDER BY ordnung"#,
            [parcours_id.0.to_string()],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

    let mut ziel_ids = Vec::new();
    while let Some(row) = rows
        .next()
        .await
        .contextualize_with(libsql_error_to_persistence_error)?
    {
        let id: String = row.get(0).contextualize(RepositoryError::Data)?;
        ziel_ids.push(ZielId(
            parse_uuid(&id).contextualize(RepositoryError::Data)?,
        ));
    }

    let mut platzierungen = Vec::with_capacity(ziel_ids.len());
    for ziel_id in ziel_ids {
        let mut abstand_rows = tx
            .query(
                r#"SELECT pflock_id, meter FROM "zielplatzierung_abstaende"
                   WHERE parcours_id = ?1 AND ziel_id = ?2"#,
                (parcours_id.0.to_string(), ziel_id.0.to_string()),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        let mut abstände = Vec::new();
        while let Some(row) = abstand_rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            let pflock_id: String = row.get(0).contextualize(RepositoryError::Data)?;
            let meter: i64 = row.get(1).contextualize(RepositoryError::Data)?;
            let meter = u32::try_from(meter).contextualize(RepositoryError::Data)?;
            abstände.push((
                PflockId(parse_uuid(&pflock_id).contextualize(RepositoryError::Data)?),
                logandaim_core::domain::Abstand::neu(meter)
                    .change_context(RepositoryError::Data)?,
            ));
        }
        platzierungen
            .push(Zielplatzierung::neu(ziel_id, abstände).change_context(RepositoryError::Data)?);
    }
    Ok(platzierungen)
}

async fn anlage_aus_zeile(tx: &Transaction, row: &Row) -> RepositoryResult<Versioned<Anlage>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let zustand_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let name_raw: String = row.get(2).contextualize(RepositoryError::Data)?;
    let strasse: String = row.get(3).contextualize(RepositoryError::Data)?;
    let postleitzahl: String = row.get(4).contextualize(RepositoryError::Data)?;
    let ort: String = row.get(5).contextualize(RepositoryError::Data)?;
    let laendercode: String = row.get(6).contextualize(RepositoryError::Data)?;
    let kontakt_name: Option<String> = row.get(7).contextualize(RepositoryError::Data)?;
    let kontakt_email: Option<String> = row.get(8).contextualize(RepositoryError::Data)?;
    let kontakt_telefon: Option<String> = row.get(9).contextualize(RepositoryError::Data)?;
    let kontakt_website: Option<String> = row.get(10).contextualize(RepositoryError::Data)?;
    let deaktiviert_am: Option<String> = row.get(11).contextualize(RepositoryError::Data)?;

    let id = AnlageId(parse_uuid(&id_raw).contextualize(RepositoryError::Data)?);
    let name = AnlageName::neu(&name_raw).change_context(RepositoryError::Data)?;
    let adresse = AnlageAdresse::neu(
        &strasse,
        &postleitzahl,
        &ort,
        Ländercode::from_str(&laendercode).change_context(RepositoryError::Data)?,
    )
    .change_context(RepositoryError::Data)?;
    let kontakt = AnlageKontakt::neu(
        kontakt_name,
        kontakt_email
            .map(|email| {
                logandaim_core::domain::EmailAdresse::normalisiert(&email)
                    .change_context(RepositoryError::Data)
            })
            .transpose()?,
        kontakt_telefon
            .map(|telefon| {
                logandaim_core::domain::Mobilnummer::new(&telefon)
                    .change_context(RepositoryError::Data)
            })
            .transpose()?,
        kontakt_website
            .map(|website| {
                logandaim_core::domain::anlage::Website::neu(&website)
                    .change_context(RepositoryError::Data)
            })
            .transpose()?,
    );

    let verwaltungen = lade_verwaltungen(tx, id).await?;
    let ziele = lade_ziele(tx, id).await?;
    let parcours = lade_parcours(tx, id).await?;
    let version = parse_version(row, 12)?;

    let anlage = match zustand_from_str(&zustand_raw)? {
        Zustand::Aktiv => Anlage::Aktiv(AnlageAktiv::from_parts(
            id,
            name,
            adresse,
            kontakt,
            verwaltungen,
            ziele,
            parcours,
        )),
        Zustand::Archiviert => {
            let am = deaktiviert_am
                .ok_or(RepositoryError::Data)
                .and_then(|am| parse_datetime(&am))?;
            Anlage::Archiviert(AnlageArchiviert::from_parts(
                id,
                name,
                adresse,
                kontakt,
                verwaltungen,
                ziele,
                parcours,
                am,
            ))
        }
    };
    Ok(Versioned::new(version, anlage))
}

/// Replace every child row of one Anlage. Called inside the root's transaction, after the
/// root row itself was written successfully.
async fn ersetze_kinder(tx: &Transaction, anlage: &Anlage) -> Result<(), RepositoryError> {
    let anlage_id = anlage.id().0.to_string();

    tx.execute(
        r#"DELETE FROM "anlage_verwaltungen" WHERE anlage_id = ?1"#,
        [anlage_id.clone()],
    )
    .await
    .map_err(|_| RepositoryError::OperationFailed)?;
    for verwaltung in anlage.verwaltungen() {
        tx.execute(
            r#"INSERT INTO "anlage_verwaltungen" (anlage_id, benutzer_id, rolle)
               VALUES (?1, ?2, ?3)"#,
            (
                anlage_id.clone(),
                verwaltung.benutzer_id().0.to_string(),
                verwaltung.rolle().as_str(),
            ),
        )
        .await
        .map_err(|_| RepositoryError::OperationFailed)?;
    }

    // Children are removed and reinserted wholesale. The foreign keys cascade the grandchildren
    // (`pflock_setups`, `zielplatzierungen`, and their distances).
    for table in ["pflock_setups", "parcours", "ziele"] {
        let sql = match table {
            "pflock_setups" => {
                r#"DELETE FROM "pflock_setups" WHERE parcours_id IN
                     (SELECT id FROM "parcours" WHERE anlage_id = ?1)"#
            }
            _ => {
                if table == "parcours" {
                    r#"DELETE FROM "parcours" WHERE anlage_id = ?1"#
                } else {
                    r#"DELETE FROM "ziele" WHERE anlage_id = ?1"#
                }
            }
        };
        tx.execute(sql, [anlage_id.clone()])
            .await
            .map_err(|_| RepositoryError::OperationFailed)?;
    }

    for (ordnung, ziel) in anlage.ziele().iter().enumerate() {
        tx.execute(
            r#"INSERT INTO "ziele" (id, anlage_id, tiername, zielgruppe, zustand, ordnung)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
            (
                ziel.id().0.to_string(),
                anlage_id.clone(),
                ziel.tiername().as_str(),
                i64::from(ziel.zielgruppe().value()),
                zustand_to_str(ziel.zustand()),
                ordnung as i64,
            ),
        )
        .await
        .map_err(|_| RepositoryError::OperationFailed)?;
    }

    for (ordnung, parcours) in anlage.parcours().iter().enumerate() {
        let parcours_id = parcours.id().0.to_string();
        tx.execute(
            r#"INSERT INTO "parcours" (id, anlage_id, name, zustand, ordnung)
               VALUES (?1, ?2, ?3, ?4, ?5)"#,
            (
                parcours_id.clone(),
                anlage_id.clone(),
                parcours.name().as_str(),
                zustand_to_str(parcours.zustand()),
                ordnung as i64,
            ),
        )
        .await
        .map_err(|_| RepositoryError::OperationFailed)?;

        for (pflock_ordnung, pflock) in parcours.pflöcke().iter().enumerate() {
            tx.execute(
                r#"INSERT INTO "pflock_setups" (id, parcours_id, farbe, ordnung)
                   VALUES (?1, ?2, ?3, ?4)"#,
                (
                    pflock.id().0.to_string(),
                    parcours_id.clone(),
                    pflock.farbe().as_str(),
                    pflock_ordnung as i64,
                ),
            )
            .await
            .map_err(|_| RepositoryError::OperationFailed)?;
        }

        for (platzierung_ordnung, platzierung) in parcours.platzierungen().iter().enumerate() {
            let ziel_id = platzierung.ziel_id().0.to_string();
            tx.execute(
                r#"INSERT INTO "zielplatzierungen" (parcours_id, ziel_id, ordnung)
                   VALUES (?1, ?2, ?3)"#,
                (
                    parcours_id.clone(),
                    ziel_id.clone(),
                    platzierung_ordnung as i64,
                ),
            )
            .await
            .map_err(|_| RepositoryError::OperationFailed)?;

            for (pflock_id, abstand) in platzierung.abstände() {
                tx.execute(
                    r#"INSERT INTO "zielplatzierung_abstaende" (parcours_id, ziel_id, pflock_id, meter)
                       VALUES (?1, ?2, ?3, ?4)"#,
                    (
                        parcours_id.clone(),
                        ziel_id.clone(),
                        pflock_id.0.to_string(),
                        i64::from(abstand.meter()),
                    ),
                )
                .await
                .map_err(|_| RepositoryError::OperationFailed)?;
            }
        }
    }
    Ok(())
}

fn kinder_loeschen_sql(anlage_id: &str) -> Vec<(&'static str, String)> {
    vec![
        (
            r#"DELETE FROM "ziele" WHERE anlage_id = ?1"#,
            anlage_id.to_owned(),
        ),
        (
            r#"DELETE FROM "parcours" WHERE anlage_id = ?1"#,
            anlage_id.to_owned(),
        ),
        (
            r#"DELETE FROM "anlage_verwaltungen" WHERE anlage_id = ?1"#,
            anlage_id.to_owned(),
        ),
    ]
}

pub struct SQLiteAnlageRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl AnlageRepository for SQLiteAnlageRepository {
    async fn create(&self, anlage: NeueAnlage) -> RepositoryResult<Versioned<Anlage>> {
        let id = AnlageId(Uuid::new_v4());
        let domain = Anlage::Aktiv(AnlageAktiv::neu(id, anlage));

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            r#"INSERT INTO "anlagen" (id, zustand, name, strasse, postleitzahl, ort, laendercode,
                                      kontakt_name, kontakt_email, kontakt_telefon,
                                      kontakt_website, deaktiviert_am, _version)
               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, NULL, 0)"#,
            (
                id.0.to_string(),
                Zustand::Aktiv.as_str(),
                domain.name().as_str(),
                domain.adresse().straße(),
                domain.adresse().postleitzahl(),
                domain.adresse().ort(),
                domain.adresse().ländercode().as_str(),
                domain.kontakt().name().map(str::to_owned),
                domain
                    .kontakt()
                    .email()
                    .map(|email| email.as_ref().to_owned()),
                domain
                    .kontakt()
                    .telefon()
                    .map(|telefon| telefon.as_ref().to_owned()),
                domain
                    .kontakt()
                    .website()
                    .map(|website| website.as_str().to_owned()),
            ),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(Versioned::init(domain))
    }

    async fn find_by_id(&self, id: AnlageId) -> RepositoryResult<Versioned<Anlage>> {
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

        anlage_aus_zeile(tx, &row).await
    }

    async fn find_aktiv(&self) -> RepositoryResult<Vec<Versioned<Anlage>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                &format!(r#"{SELECT} WHERE zustand = ?1 ORDER BY name"#),
                [Zustand::Aktiv.as_str()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut anlagen = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            anlagen.push(anlage_aus_zeile(tx, &row).await?);
        }
        Ok(anlagen)
    }

    async fn find_by_verwaltung(
        &self,
        benutzer_id: logandaim_core::domain::konto::BenutzerId,
    ) -> RepositoryResult<Vec<Versioned<Anlage>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                &format!(
                    r#"{SELECT} WHERE id IN
                         (SELECT anlage_id FROM "anlage_verwaltungen" WHERE benutzer_id = ?1)
                       ORDER BY name"#
                ),
                [benutzer_id.0.to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut anlagen = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            anlagen.push(anlage_aus_zeile(tx, &row).await?);
        }
        Ok(anlagen)
    }

    async fn update(&self, anlage: &mut Versioned<Anlage>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let (zustand, deaktiviert_am) = match &**anlage {
            Anlage::Aktiv(_) => (Zustand::Aktiv, None),
            Anlage::Archiviert(archiviert) => {
                (Zustand::Archiviert, Some(archiviert.deaktiviert_am()))
            }
        };

        update_versioned(
            tx,
            r#"UPDATE "anlagen" SET zustand = ?2, name = ?3, strasse = ?4, postleitzahl = ?5,
                   ort = ?6, laendercode = ?7, kontakt_name = ?8, kontakt_email = ?9,
                   kontakt_telefon = ?10, kontakt_website = ?11, deaktiviert_am = ?12,
                   _version = ?13 + 1
               WHERE id = ?1 AND _version = ?13"#,
            (
                anlage.id().0.to_string(),
                zustand.as_str(),
                anlage.name().as_str(),
                anlage.adresse().straße(),
                anlage.adresse().postleitzahl(),
                anlage.adresse().ort(),
                anlage.adresse().ländercode().as_str(),
                anlage.kontakt().name().map(str::to_owned),
                anlage
                    .kontakt()
                    .email()
                    .map(|email| email.as_ref().to_owned()),
                anlage
                    .kontakt()
                    .telefon()
                    .map(|telefon| telefon.as_ref().to_owned()),
                anlage
                    .kontakt()
                    .website()
                    .map(|website| website.as_str().to_owned()),
                deaktiviert_am.map(format_datetime),
                anlage.v() as i64,
            ),
            anlage.v(),
        )
        .await?;

        ersetze_kinder(tx, anlage).await?;

        anlage.increment();
        Ok(())
    }

    async fn delete(&self, anlage: Versioned<Anlage>) -> RepositoryResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        for (sql, anlage_id) in kinder_loeschen_sql(&anlage.id().0.to_string()) {
            tx.execute(sql, [anlage_id])
                .await
                .contextualize_with(libsql_error_to_persistence_error)?;
        }

        delete_versioned(
            tx,
            r#"DELETE FROM "anlagen" WHERE id = ?1 AND _version = ?2"#,
            (anlage.id().0.to_string(), anlage.v() as i64),
            anlage.v(),
        )
        .await
    }
}
