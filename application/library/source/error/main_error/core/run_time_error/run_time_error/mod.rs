use diesel::result::Error as DieselError;
use lettre_email::error::Error as LettreEmailError;
use lettre::smtp::error::Error as LettreSmtpError;
use r2d2::Error as R2d2Error;
use redis::RedisError as RedisCrateError;
use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::core::resource_error::core::_in_context_for::_resource::_new_for_context::connection_pool_error::ConnectionPoolError;
use super::core::resource_error::core::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;
use super::core::resource_error::core::_in_context_for::_resource::postgresql::_new_for_context::postgresql_error::PostgresqlError;
use super::core::resource_error::core::_in_context_for::_resource::redis::_new_for_context::redis_error::RedisError;
use super::core::resource_error::resource_error::ResourceError;

#[derive(Debug)]
pub enum RunTimeError {
    ResourceErrorKind(ResourceError)
}

impl Display for RunTimeError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for RunTimeError {}

impl From<ConnectionPoolError> for RunTimeError {
    fn from(connection_pool_error_kind: ConnectionPoolError) -> Self {
        return Self::ResourceErrorKind(ResourceError::ConnectionPoolError(connection_pool_error_kind));
    }
}

impl From<R2d2Error> for RunTimeError {
    fn from(r2d2_error: R2d2Error) -> Self {
        return Self::ResourceErrorKind(ResourceError::ConnectionPoolError(ConnectionPoolError::CommonError(r2d2_error)));
    }
}

impl From<EmailServerError> for RunTimeError {
    fn from(email_server_error_kind: EmailServerError) -> Self {
        return Self::ResourceErrorKind(ResourceError::EmailServerError(email_server_error_kind));
    }
}

impl From<PostgresqlError> for RunTimeError {
    fn from(postgresql_error_kind: PostgresqlError) -> Self {
        return Self::ResourceErrorKind(ResourceError::PostgresqlError(postgresql_error_kind));
    }
}

impl From<RedisError> for RunTimeError {
    fn from(redis_error_kind: RedisError) -> Self {
        return Self::ResourceErrorKind(ResourceError::RedisError(redis_error_kind));
    }
}

impl From<DieselError> for RunTimeError {
    fn from(diesel_error: DieselError) -> Self {
        return Self::ResourceErrorKind(ResourceError::PostgresqlError(PostgresqlError::RuntimeError(diesel_error)));
    }
}

impl From<RedisCrateError> for RunTimeError {
    fn from(redis_crate_error: RedisCrateError) -> Self {
        return Self::ResourceErrorKind(ResourceError::RedisError(RedisError::RuntimeError(redis_crate_error)));
    }
}

impl From<LettreEmailError> for RunTimeError {
    fn from(lettre_email_error: LettreEmailError) -> Self {
        return Self::ResourceErrorKind(ResourceError::EmailServerError(EmailServerError::EmailError(lettre_email_error)));
    }
}

impl From<LettreSmtpError> for RunTimeError {
    fn from(lettre_smtp_error: LettreSmtpError) -> Self {
        return Self::ResourceErrorKind(ResourceError::EmailServerError(EmailServerError::SmtpError(lettre_smtp_error)));
    }
}