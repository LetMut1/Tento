use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::core::_in_context_for::_resource::_new_for_context::connection_pool_error::ConnectionPoolError;
use super::core::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;
use super::core::_in_context_for::_resource::postgresql::_new_for_context::postgresql_error::PostgresqlError;
use super::core::_in_context_for::_resource::redis::_new_for_context::redis_error::RedisError;

#[derive(Debug)]
pub enum ResourceError {
    ConnectionPoolError(ConnectionPoolError),
    EmailServerError(EmailServerError),
    PostgresqlError(PostgresqlError),
    RedisError(RedisError)
}

impl Display for ResourceError {
    fn fmt<'this, 'outer_a>(&'this self, _: &'outer_a mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for ResourceError {}