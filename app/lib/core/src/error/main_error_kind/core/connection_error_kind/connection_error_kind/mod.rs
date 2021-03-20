use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use super::core::postgresql::postgresql_connection_error::PostgresqlConnectionError;

#[derive(Debug)]
pub enum ConnectionErrorKind {
    PostgresqlConnectionError(PostgresqlConnectionError)
}

impl Display for ConnectionErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for ConnectionErrorKind {}

impl From<PostgresqlConnectionError> for ConnectionErrorKind {
    fn from(postgresql_connection_error: PostgresqlConnectionError) -> Self {
        return Self::PostgresqlConnectionError(postgresql_connection_error);
    }
}