use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::{domain::Menge, ports::RepositoryError};

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
