use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use logandaim_core::{
    ErrorReportExt,
    domain::Menge,
    ports::{RepositoryError, RepositoryResult},
};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::errors::libsql_error_to_persistence_error;

/// Parse a stored UUID. A malformed string is retained as the report's source, so a corrupt
/// row names what it read rather than only that something was wrong.
pub fn parse_uuid(s: &str) -> RepositoryResult<Uuid> {
    Uuid::from_str(s).contextualize(RepositoryError::Data)
}

pub fn parse_naive_date(s: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

pub fn parse_decimal(s: &str) -> RepositoryResult<Decimal> {
    Decimal::from_str(s).contextualize(RepositoryError::Data)
}

pub fn parse_menge(s: &str) -> RepositoryResult<Menge> {
    Menge::new(parse_decimal(s)?).contextualize(RepositoryError::Data)
}

pub fn menge_to_str(menge: &Menge) -> String {
    menge.value().to_string()
}

pub fn format_naive_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn format_datetime(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

pub fn parse_datetime(s: &str) -> RepositoryResult<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .contextualize(RepositoryError::Data)
}

/// Read the `_version` column of a row.
pub fn parse_version(row: &libsql::Row, index: i32) -> RepositoryResult<u64> {
    let version: i64 = row.get(index).contextualize(RepositoryError::Data)?;
    u64::try_from(version).contextualize(RepositoryError::Data)
}

/// Conditional full-row update, so a stale `_version` is a `VersionMismatch` rather than a
/// silent overwrite.
///
/// `UPDATE ... SET ..., _version = ?expected + 1 WHERE id = ?id AND _version = ?expected`.
/// Zero affected rows means the row moved under the caller's feet.
pub async fn update_versioned(
    tx: &libsql::Transaction,
    sql: &str,
    params: impl libsql::params::IntoParams,
    expected: u64,
) -> RepositoryResult<()> {
    let affected = tx
        .execute(sql, params)
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
    if affected == 0 {
        return Err(RepositoryError::VersionMismatch {
            expected,
            actual: None,
        }
        .into());
    }
    Ok(())
}

/// Conditional delete guarded by `_version`, so removing an aggregate somebody else has
/// changed since it was loaded is a `VersionMismatch` rather than a vanished row.
///
/// `DELETE ... WHERE id = ?id AND _version = ?expected`.
pub async fn delete_versioned(
    tx: &libsql::Transaction,
    sql: &str,
    params: impl libsql::params::IntoParams,
    expected: u64,
) -> RepositoryResult<()> {
    let affected = tx
        .execute(sql, params)
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
    if affected == 0 {
        return Err(RepositoryError::VersionMismatch {
            expected,
            actual: None,
        }
        .into());
    }
    Ok(())
}
