use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use logandaim_core::{domain::Menge, ports::RepositoryError};
use rust_decimal::Decimal;
use uuid::Uuid;

pub fn parse_naive_date(s: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

pub fn parse_uuid(s: &str) -> Result<Uuid, RepositoryError> {
    Uuid::from_str(s).map_err(|_| RepositoryError::Data)
}

pub fn parse_decimal(s: &str) -> Result<Decimal, RepositoryError> {
    Decimal::from_str(s).map_err(|_| RepositoryError::Data)
}

pub fn parse_menge(s: &str) -> Result<Menge, RepositoryError> {
    let decimal = parse_decimal(s)?;
    Menge::new(decimal).map_err(|_| RepositoryError::Data)
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

pub fn parse_datetime(s: &str) -> Result<DateTime<Utc>, RepositoryError> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| RepositoryError::Data)
}

/// Read the `_version` column of a row.
pub fn parse_version(row: &libsql::Row, index: i32) -> Result<u64, RepositoryError> {
    let version: i64 = row.get(index).map_err(|_| RepositoryError::Data)?;
    u64::try_from(version).map_err(|_| RepositoryError::Data)
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
) -> Result<(), RepositoryError> {
    let affected = tx
        .execute(sql, params)
        .await
        .map_err(|_| RepositoryError::OperationFailed)?;
    if affected == 0 {
        return Err(RepositoryError::VersionMismatch {
            expected,
            actual: None,
        });
    }
    Ok(())
}
