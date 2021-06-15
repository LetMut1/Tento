use diesel::result::ConnectionError;
use diesel::result::Error as DieselError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum PostgresqlError {
    ConnectionError(ConnectionError),
    RuntimeError(DieselError)
}

impl Display for PostgresqlError {
    fn fmt<'this, 'outer_a>(&'this self, _: &'outer_a mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for PostgresqlError {}