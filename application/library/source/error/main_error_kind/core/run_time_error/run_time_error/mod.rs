use diesel::result::Error as DieselError;
use lettre_email::error::Error as LettreEmailError;
use lettre::smtp::error::Error as LettreSmtpError;
use r2d2::Error as R2d2Error;
use redis::RedisError;
use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::core::resource_error_kind::core::_in_context_for::_resource::_new_for_context::connection_pool_error_kind::ConnectionPoolErrorKind;
use super::core::resource_error_kind::core::_in_context_for::_resource::email_server::_new_for_context::email_server_error_kind::EmailServerErrorKind;
use super::core::resource_error_kind::core::_in_context_for::_resource::postgresql::_new_for_context::postgresql_error_kind::PostgresqlErrorKind;
use super::core::resource_error_kind::core::_in_context_for::_resource::redis::_new_for_context::redis_error_kind::RedisErrorKind;
use super::core::resource_error_kind::resource_error_kind::ResourceErrorKind;

#[derive(Debug)]
pub enum RunTimeError {
    ResourceErrorKind(ResourceErrorKind)
}

impl Display for RunTimeError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for RunTimeError {}

impl From<ConnectionPoolErrorKind> for RunTimeError {
    fn from(connection_pool_error_kind: ConnectionPoolErrorKind) -> Self {
        return Self::ResourceErrorKind(ResourceErrorKind::ConnectionPoolErrorKind(connection_pool_error_kind));
    }
}

impl From<R2d2Error> for RunTimeError {
    fn from(r2d2_error: R2d2Error) -> Self {
        return Self::ResourceErrorKind(ResourceErrorKind::ConnectionPoolErrorKind(ConnectionPoolErrorKind::CommonError(r2d2_error)));
    }
}

impl From<EmailServerErrorKind> for RunTimeError {
    fn from(email_server_error_kind: EmailServerErrorKind) -> Self {
        return Self::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(email_server_error_kind));
    }
}

impl From<PostgresqlErrorKind> for RunTimeError {
    fn from(postgresql_error_kind: PostgresqlErrorKind) -> Self {
        return Self::ResourceErrorKind(ResourceErrorKind::PostgresqlErrorKind(postgresql_error_kind));
    }
}

impl From<RedisErrorKind> for RunTimeError {
    fn from(redis_error_kind: RedisErrorKind) -> Self {
        return Self::ResourceErrorKind(ResourceErrorKind::RedisErrorKind(redis_error_kind));
    }
}

impl From<DieselError> for RunTimeError {
    fn from(diesel_error: DieselError) -> Self {
        return Self::ResourceErrorKind(ResourceErrorKind::PostgresqlErrorKind(PostgresqlErrorKind::RuntimeError(diesel_error)));
    }
}

impl From<RedisError> for RunTimeError {
    fn from(redis_error: RedisError) -> Self {
        return Self::ResourceErrorKind(ResourceErrorKind::RedisErrorKind(RedisErrorKind::RuntimeError(redis_error)));
    }
}

impl From<LettreEmailError> for RunTimeError {
    fn from(lettre_email_error: LettreEmailError) -> Self {
        return Self::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::EmailError(lettre_email_error)));
    }
}

impl From<LettreSmtpError> for RunTimeError {
    fn from(lettre_smtp_error: LettreSmtpError) -> Self {
        return Self::ResourceErrorKind(ResourceErrorKind::EmailServerErrorKind(EmailServerErrorKind::SmtpError(lettre_smtp_error)));
    }
}