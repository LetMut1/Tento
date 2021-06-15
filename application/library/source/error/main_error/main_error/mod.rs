use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::core::entity_error::entity_error::EntityError;
use super::core::invalid_argument_error::InvalidArgumentError;
use super::core::logic_error::LogicError;
use super::core::run_time_error::core::resource_error::core::_in_context_for::_resource::_new_for_context::connection_pool_error::ConnectionPoolError;
use super::core::run_time_error::core::resource_error::core::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;
use super::core::run_time_error::core::resource_error::core::_in_context_for::_resource::postgresql::_new_for_context::postgresql_error::PostgresqlError;
use super::core::run_time_error::core::resource_error::core::_in_context_for::_resource::redis::_new_for_context::redis_error::RedisError;
use super::core::run_time_error::core::resource_error::resource_error::ResourceError;
use super::core::run_time_error::run_time_error::RunTimeError;

#[derive(Debug)]
pub enum MainError {
    EntityError(EntityError),
    InvalidArgumentError,
    LogicError(LogicError),
    RunTimeError(RunTimeError)
}

impl Display for MainError {
    fn fmt<'this, 'outer_a>(&'this self, formatter: &'outer_a mut Formatter<'_>) -> Result {
        match self {
            Self::LogicError(logic_error) => {
                write!(formatter, "MainError-LogicError: {}", logic_error.get_message())?;
            },
            Self::RunTimeError(run_time_error) => {
                match run_time_error {
                    RunTimeError::ResourceError(resource_error) => {
                        match resource_error {
                            ResourceError::ConnectionPoolError(connection_pool_error) => {
                                match connection_pool_error {
                                    ConnectionPoolError::CommonError(r2d2_error) => {
                                        write!(formatter, "MainError-RunTimeError-ResourceError-ConnectionPoolError-CommonError: {}", r2d2_error)?;
                                    }
                                }
                            },
                            ResourceError::EmailServerError(email_server_error) => {
                                match email_server_error {
                                    EmailServerError::EmailError(email_error) => {
                                        write!(formatter, "MainError-RunTimeError-ResourceError-EmailServerError-EmailError: {}", email_error)?;
                                    },
                                    EmailServerError::SmtpError(smtp_error) => {
                                        write!(formatter, "MainError-RunTimeError-ResourceError-EmailServerError-SmtpError: {}", smtp_error)?;
                                    }
                                }
                            },
                            ResourceError::PostgresqlError(postgresql_error) => {
                                match postgresql_error {
                                    PostgresqlError::ConnectionError(connection_error) => {
                                        write!(formatter, "MainError-RunTimeError-ResourceError-PostgresqlError-ConnectionError: {}", connection_error)?;
                                    },
                                    PostgresqlError::RuntimeError(run_time_error) => {
                                        write!(formatter, "MainError-RunTimeError-ResourceError-PostgresqlError-RuntimeError: {}", run_time_error)?;
                                    }
                                }
                            },
                            ResourceError::RedisError(redis_error) => {
                                match redis_error {
                                    RedisError::ConnectionError(connection_error) => {
                                        write!(formatter, "MainError-RunTimeError-ResourceError-RedisError-ConnectionError: {}", connection_error)?;
                                    },
                                    RedisError::RuntimeError(run_time_error) => {
                                        write!(formatter, "MainError-RunTimeError-ResourceError-RedisError-RuntimeError: {}", run_time_error)?;
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

impl Error for MainError {}

impl From<EntityError> for MainError {
    fn from(entity_error: EntityError) -> Self {
        return Self::EntityError(entity_error);
    }
}

impl From<InvalidArgumentError> for MainError {
    fn from(_: InvalidArgumentError) -> Self {
        return Self::InvalidArgumentError;
    }
}

impl From<LogicError> for MainError {
    fn from(logic_error: LogicError) -> Self {
        return Self::LogicError(logic_error);
    }
}

impl From<RunTimeError> for MainError {
    fn from(runt_time_error: RunTimeError) -> Self {
        return Self::RunTimeError(runt_time_error);
    }
}