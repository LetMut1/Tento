use diesel::result::ConnectionError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub struct PostgresqlConnectionError {
    connection_error: ConnectionError
}

impl PostgresqlConnectionError {
    pub fn new(connection_error: ConnectionError) -> Self {
        return Self {
            connection_error
        };
    }
}

impl Display for PostgresqlConnectionError {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for PostgresqlConnectionError {}