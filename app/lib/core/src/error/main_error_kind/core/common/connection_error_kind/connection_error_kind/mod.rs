use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use super::core::postgresql::postgresql_connection_error_kind::PostgresqlConnectionErrorKind;

#[derive(Debug)]
pub enum ConnectionErrorKind {
    Postgresql(PostgresqlConnectionErrorKind)
}

impl Display for ConnectionErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for ConnectionErrorKind {}
