use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use super::core::_in_context_for::postgresql::_new_for_context::postgresql_connection_error::PostgresqlConnectionError;
use super::core::_in_context_for::redis::_new_for_context::redis_connection_error::RedisConnectionError;

#[derive(Debug)]
pub enum ConnectionErrorKind {
    PostgresqlConnectionError(PostgresqlConnectionError),
    RedisConnectionError(RedisConnectionError)
}

impl Display for ConnectionErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for ConnectionErrorKind {}