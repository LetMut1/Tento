use diesel::result::Error as DieselError;
use lettre_email::error::Error as LettreEmailError;
use lettre::smtp::error::Error as LettreSmtpError;
use r2d2::Error as R2d2Error;
use redis::RedisError;
use serde_json::Error as SerdeJsonError;
use std::convert::From;
use std::env::VarError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_core::entity_error::entity_error::EntityError;
use super::_core::run_time_error::_core::resource_error::_core::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;
use super::_core::run_time_error::_core::resource_error::resource_error::ResourceError;
use super::_core::run_time_error::run_time_error::RunTimeError;


#[derive(Debug)]
pub enum MainError {
    EntityError(EntityError),
    InvalidArgumentError,
    LogicError(&'static str),
    RunTimeError(RunTimeError)
}

impl Display for MainError {
    fn fmt<'this, 'outer_a>(&'this self, formatter: &'outer_a mut Formatter<'_>) -> Result {
        match self {
            Self::LogicError(message) => {
                write!(formatter, "MainError-LogicError: {}", message)?;
            },
            Self::RunTimeError(run_time_error) => {
                match run_time_error {
                    RunTimeError::EnvironmentVariableError(var_error) => {
                        write!(formatter, "MainError-RunTimeError-EnvironmentVariableError: {}", var_error)?;
                    },
                    RunTimeError::ResourceError(resource_error) => {
                        match resource_error {
                            ResourceError::ConnectionPoolError(r2d2_error) => {
                                write!(formatter, "MainError-RunTimeError-ResourceError-ConnectionPoolError: {}", r2d2_error)?;
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
                            ResourceError::PostgresqlError(diesel_error) => {
                                write!(formatter, "MainError-RunTimeError-ResourceError-PostgresqlError: {}", diesel_error)?;
                            },
                            ResourceError::RedisError(redis_error) => {
                                write!(formatter, "MainError-RunTimeError-ResourceError-RedisError: {}", redis_error)?;
                            }
                        }
                    },
                    RunTimeError::SerializationDeserializationError(serde_json_error) => {
                        write!(formatter, "MainError-RunTimeError-SerializationDeserializationError: {}", serde_json_error)?;
                    }
                }
            },
            _ => {}
        }

        return Ok(());
    }
}

impl Error for MainError {}

impl From<VarError> for MainError {
    fn from(var_error: VarError) -> Self {
        return Self::RunTimeError(RunTimeError::EnvironmentVariableError(var_error));
    }
}

impl From<R2d2Error> for MainError {
    fn from(r2d2_error: R2d2Error) -> Self {
        return Self::RunTimeError(RunTimeError::ResourceError(ResourceError::ConnectionPoolError(r2d2_error)));
    }
}

impl From<DieselError> for MainError {
    fn from(diesel_error: DieselError) -> Self {
        return Self::RunTimeError(RunTimeError::ResourceError(ResourceError::PostgresqlError(diesel_error)));
    }
}

impl From<RedisError> for MainError {
    fn from(redis_error: RedisError) -> Self {
        return Self::RunTimeError(RunTimeError::ResourceError(ResourceError::RedisError(redis_error)));
    }
}

impl From<LettreEmailError> for MainError {
    fn from(lettre_email_error: LettreEmailError) -> Self {
        return Self::RunTimeError(RunTimeError::ResourceError(ResourceError::EmailServerError(EmailServerError::EmailError(lettre_email_error))));
    }
}

impl From<LettreSmtpError> for MainError {
    fn from(lettre_smtp_error: LettreSmtpError) -> Self {
        return Self::RunTimeError(RunTimeError::ResourceError(ResourceError::EmailServerError(EmailServerError::SmtpError(lettre_smtp_error))));
    }
}

impl From<SerdeJsonError> for MainError {
    fn from(serde_json_error: SerdeJsonError) -> Self {
        return Self::RunTimeError(RunTimeError::SerializationDeserializationError(serde_json_error));
    }
}