use crate::error::context::Context;
use diesel::result::ConnectionError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum PostgresqlConnectionErrorKind {
    CanNotEstablish(Context<ConnectionError>)
}

impl PostgresqlConnectionErrorKind {
    pub fn new_can_not_establish(connection_error: ConnectionError, message: Option<String>) -> Self {
        return Self::CanNotEstablish(Context::new(Some(connection_error), message));
    }
}

impl Display for PostgresqlConnectionErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for PostgresqlConnectionErrorKind {}