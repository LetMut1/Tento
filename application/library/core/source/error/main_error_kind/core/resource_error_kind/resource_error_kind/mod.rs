use diesel::result::Error as DieselError;
use lettre_email::error::Error as LettreEmailError;
use lettre::smtp::error::Error as LettreSmtpError;
use redis::RedisError;
use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use super::core::_in_context_for::resource::email_server::_new_for_context::email_server_error_kind::EmailServerErrorKind;
use super::core::_in_context_for::resource::postgresql::_new_for_context::postgresql_error_kind::PostgresqlErrorKind;
use super::core::_in_context_for::resource::redis::_new_for_context::redis_error_kind::RedisErrorKind;

#[derive(Debug)]
pub enum ResourceErrorKind {
    EmailServerErrorKind(EmailServerErrorKind),
    PostgresqlErrorKind(PostgresqlErrorKind),
    RedisErrorKind(RedisErrorKind)
}

impl Display for ResourceErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for ResourceErrorKind {}

impl From<EmailServerErrorKind> for ResourceErrorKind {
    fn from(email_server_error_kind: EmailServerErrorKind) -> Self {
        return Self::EmailServerErrorKind(email_server_error_kind);
    }
}

impl From<PostgresqlErrorKind> for ResourceErrorKind {
    fn from(postgresql_error_kind: PostgresqlErrorKind) -> Self {
        return Self::PostgresqlErrorKind(postgresql_error_kind);
    }
}

impl From<RedisErrorKind> for ResourceErrorKind {
    fn from(redis_error_kind: RedisErrorKind) -> Self {
        return Self::RedisErrorKind(redis_error_kind);
    }
}

impl From<DieselError> for ResourceErrorKind {
    fn from(diesel_error: DieselError) -> Self {
        return Self::PostgresqlErrorKind(PostgresqlErrorKind::RuntimeError(diesel_error));
    }
}

impl From<RedisError> for ResourceErrorKind {
    fn from(redis_error: RedisError) -> Self {
        return Self::RedisErrorKind(RedisErrorKind::RuntimeError(redis_error));
    }
}

impl From<LettreEmailError> for ResourceErrorKind {
    fn from(lettre_email_error: LettreEmailError) -> Self {
        return Self::EmailServerErrorKind(EmailServerErrorKind::EmailError(lettre_email_error));
    }
}

impl From<LettreSmtpError> for ResourceErrorKind {
    fn from(lettre_smtp_error: LettreSmtpError) -> Self {
        return Self::EmailServerErrorKind(EmailServerErrorKind::SmtpError(lettre_smtp_error));
    }
}