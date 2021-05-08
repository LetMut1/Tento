use diesel::result::ConnectionError;
use diesel::result::Error as DieselError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum PostgresqlErrorKind {
    ConnectionError(ConnectionError),
    RuntimeError(DieselError)
}

impl Display for PostgresqlErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for PostgresqlErrorKind {}