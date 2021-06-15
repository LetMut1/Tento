use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::core::entity_error_kind::entity_error_kind::EntityErrorKind;
use super::core::invalid_argument_error::InvalidArgumentError;
use super::core::logic_error::LogicError;
use super::core::run_time_error::core::resource_error_kind::core::_in_context_for::_resource::_new_for_context::connection_pool_error_kind::ConnectionPoolErrorKind;
use super::core::run_time_error::core::resource_error_kind::core::_in_context_for::_resource::email_server::_new_for_context::email_server_error_kind::EmailServerErrorKind;
use super::core::run_time_error::core::resource_error_kind::core::_in_context_for::_resource::postgresql::_new_for_context::postgresql_error_kind::PostgresqlErrorKind;
use super::core::run_time_error::core::resource_error_kind::core::_in_context_for::_resource::redis::_new_for_context::redis_error_kind::RedisErrorKind;
use super::core::run_time_error::core::resource_error_kind::resource_error_kind::ResourceErrorKind;
use super::core::run_time_error::run_time_error_kind::RunTimeErrorKind;

#[derive(Debug)]
pub enum MainErrorKind {
    EntityErrorKind(EntityErrorKind),
    InvalidArgumentError,
    LogicError(LogicError),
    RunTimeErrorKind(RunTimeErrorKind)
}

impl Display for MainErrorKind {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        match self {
            Self::LogicError(logic_error) => {
                write!(formatter, "MainErrorKind-LogicError: {}", logic_error.get_message())?;
            },
            Self::RunTimeErrorKind(run_time_error_kind) => {
                match run_time_error_kind {
                    RunTimeErrorKind::ResourceErrorKind(resource_error_kind) => {
                        match resource_error_kind {
                            ResourceErrorKind::ConnectionPoolErrorKind(connection_pool_error_kind) => {
                                match connection_pool_error_kind {
                                    ConnectionPoolErrorKind::CommonError(r2d2_error) => {
                                        write!(formatter, "MainErrorKind-RunTimeError-ResourceErrorKind-ConnectionPoolErrorKind-CommonError: {}", r2d2_error)?;
                                    }
                                }
                            },
                            ResourceErrorKind::EmailServerErrorKind(email_server_error_kind) => {
                                match email_server_error_kind {
                                    EmailServerErrorKind::EmailError(email_error) => {
                                        write!(formatter, "MainErrorKind-RunTimeError-ResourceErrorKind-EmailServerErrorKind-EmailError: {}", email_error)?;
                                    },
                                    EmailServerErrorKind::SmtpError(smtp_error) => {
                                        write!(formatter, "MainErrorKind-RunTimeError-ResourceErrorKind-EmailServerErrorKind-SmtpError: {}", smtp_error)?;
                                    }
                                }
                            },
                            ResourceErrorKind::PostgresqlErrorKind(postgresql_error_kind) => {
                                match postgresql_error_kind {
                                    PostgresqlErrorKind::ConnectionError(connection_error) => {
                                        write!(formatter, "MainErrorKind-RunTimeError-ResourceErrorKind-PostgresqlErrorKind-ConnectionError: {}", connection_error)?;
                                    },
                                    PostgresqlErrorKind::RuntimeError(run_time_error) => {
                                        write!(formatter, "MainErrorKind-RunTimeError-ResourceErrorKind-PostgresqlErrorKind-RuntimeError: {}", run_time_error)?;
                                    }
                                }
                            },
                            ResourceErrorKind::RedisErrorKind(redis_error_kind) => {
                                match redis_error_kind {
                                    RedisErrorKind::ConnectionError(connection_error) => {
                                        write!(formatter, "MainErrorKind-RunTimeError-ResourceErrorKind-RedisErrorKind-ConnectionError: {}", connection_error)?;
                                    },
                                    RedisErrorKind::RuntimeError(run_time_error) => {
                                        write!(formatter, "MainErrorKind-RunTimeError-ResourceErrorKind-RedisErrorKind-RuntimeError: {}", run_time_error)?;
                                    }
                                }
                            }
                        }
                    }
                }
            },
            _ => {}
        }

        return Ok(());
    }
}

impl Error for MainErrorKind {}

impl From<EntityErrorKind> for MainErrorKind {
    fn from(entity_error_kind: EntityErrorKind) -> Self {
        return Self::EntityErrorKind(entity_error_kind);
    }
}

impl From<InvalidArgumentError> for MainErrorKind {
    fn from(_: InvalidArgumentError) -> Self {
        return Self::InvalidArgumentError;
    }
}

impl From<LogicError> for MainErrorKind {
    fn from(logic_error: LogicError) -> Self {
        return Self::LogicError(logic_error);
    }
}

impl From<RunTimeErrorKind> for MainErrorKind {
    fn from(runt_time_error: RunTimeErrorKind) -> Self {
        return Self::RunTimeErrorKind(runt_time_error);
    }
}