use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::core::_in_context_for::_resource::_new_for_context::connection_pool_error_kind::ConnectionPoolErrorKind;
use super::core::_in_context_for::_resource::email_server::_new_for_context::email_server_error_kind::EmailServerErrorKind;
use super::core::_in_context_for::_resource::postgresql::_new_for_context::postgresql_error_kind::PostgresqlErrorKind;
use super::core::_in_context_for::_resource::redis::_new_for_context::redis_error_kind::RedisErrorKind;

#[derive(Debug)]
pub enum ResourceErrorKind {
    ConnectionPoolErrorKind(ConnectionPoolErrorKind),
    EmailServerErrorKind(EmailServerErrorKind),
    PostgresqlErrorKind(PostgresqlErrorKind),
    RedisErrorKind(RedisErrorKind)
}

impl Display for ResourceErrorKind {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for ResourceErrorKind {}