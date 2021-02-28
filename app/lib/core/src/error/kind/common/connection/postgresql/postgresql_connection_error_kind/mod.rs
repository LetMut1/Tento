use crate::error::context::Context;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use diesel::result::ConnectionError;

#[derive(Debug)]
pub enum PostgresqlConnectionErrorKind {
    CanNotEstablish(Context<ConnectionError>)
}

impl Display for PostgresqlConnectionErrorKind{
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for PostgresqlConnectionErrorKind {}