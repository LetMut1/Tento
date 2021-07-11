use anyhow::Error as AnyhowError;
use argon2::Error as Argon2Error;
use base64::DecodeError as Base64DecodeError;
use diesel::result::Error as DieselError;
use dotenv::Error as DotenvError;
use lettre_email::error::Error as LettreEmailError;
use lettre::smtp::error::Error as LettreSmtpError;
use log::SetLoggerError;
use log4rs::config::runtime::ConfigErrors as Log4rsConfigErrors;
use r2d2::Error as R2d2Error;
use redis::RedisError;
use regex::Error as RegexError;
use serde_json::Error as SerdeJsonError;
use std::convert::From;
use std::env::VarError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io::Error as IOError;
use std::io::ErrorKind as IOErrorKind;
use super::_core::entity_error::entity_error::EntityError;
use super::_core::run_time_error::_core::other_error::OtherError;
use super::_core::run_time_error::_core::resource_error::_core::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;
use super::_core::run_time_error::_core::resource_error::resource_error::ResourceError;
use super::_core::run_time_error::run_time_error::RunTimeError;

#[derive(Debug)]
pub enum BaseError {
    EntityError(EntityError),
    InvalidArgumentError,
    LogicError(&'static str),
    RunTimeError(RunTimeError)
}

impl Display for BaseError {
    fn fmt<'this, 'outer_a>(&'this self, formatter: &'outer_a mut Formatter<'_>) -> Result {
        match self {
            Self::LogicError(message) => {
                write!(formatter, "BaseError-LogicError: {}", message)?;
            },
            Self::RunTimeError(run_time_error) => {
                match run_time_error {
                    RunTimeError::OtherError(other_error) => {
                        write!(formatter, "BaseError-RunTimeError-OtherError-{}: {}", other_error.get_description(), other_error.get_displaying())?;
                    },
                    RunTimeError::ResourceError(resource_error) => {
                        match resource_error {
                            ResourceError::ConnectionPoolError(r2d2_error) => {
                                write!(formatter, "BaseError-RunTimeError-ResourceError-ConnectionPoolError: {}", r2d2_error)?;
                            },
                            ResourceError::EmailServerError(email_server_error) => {
                                match email_server_error {
                                    EmailServerError::EmailError(email_error) => {
                                        write!(formatter, "BaseError-RunTimeError-ResourceError-EmailServerError-EmailError: {}", email_error)?;
                                    },
                                    EmailServerError::SmtpError(smtp_error) => {
                                        write!(formatter, "BaseError-RunTimeError-ResourceError-EmailServerError-SmtpError: {}", smtp_error)?;
                                    }
                                }
                            },
                            ResourceError::PostgresqlError(diesel_error) => {
                                write!(formatter, "BaseError-RunTimeError-ResourceError-PostgresqlError: {}", diesel_error)?;
                            },
                            ResourceError::RedisError(redis_error) => {
                                write!(formatter, "BaseError-RunTimeError-ResourceError-RedisError: {}", redis_error)?;
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

impl Error for BaseError {}

impl From<IOError> for BaseError {
    fn from(io_error: IOError) -> Self {
        return Self::RunTimeError(RunTimeError::OtherError(OtherError::new("IOError", io_error)));
    }
}

impl From<AnyhowError> for BaseError {
    fn from(anyhow_error: AnyhowError) -> Self {
        return Self::from(IOError::new(IOErrorKind::Other, format!("{}", anyhow_error)));
    }
}

impl From<VarError> for BaseError {
    fn from(var_error: VarError) -> Self {
        return Self::RunTimeError(RunTimeError::OtherError(OtherError::new("EnvironmentVariableError", var_error)));
    }
}

impl From<SerdeJsonError> for BaseError {
    fn from(serde_json_error: SerdeJsonError) -> Self {
        return Self::RunTimeError(RunTimeError::OtherError(OtherError::new("SerdeJsonError", serde_json_error)));
    }
}

impl From<Base64DecodeError> for BaseError {
    fn from(base64_decode_error: Base64DecodeError) -> Self {
        return Self::RunTimeError(RunTimeError::OtherError(OtherError::new("Base64DecodeError", base64_decode_error)));
    }
}

impl From<RegexError> for BaseError {
    fn from(regex_error: RegexError) -> Self {
        return Self::RunTimeError(RunTimeError::OtherError(OtherError::new("RegexError", regex_error)));
    }
}

impl From<Argon2Error> for BaseError {
    fn from(argon2_error: Argon2Error) -> Self {
        return Self::RunTimeError(RunTimeError::OtherError(OtherError::new("Argon2Error", argon2_error)));
    }
}

impl From<DotenvError> for BaseError {
    fn from(dotenv_error: DotenvError) -> Self {
        return Self::RunTimeError(RunTimeError::OtherError(OtherError::new("DotenvError", dotenv_error)));
    }
}

impl From<Log4rsConfigErrors> for BaseError {
    fn from(log4rs_config_errors: Log4rsConfigErrors) -> Self {
        return Self::RunTimeError(RunTimeError::OtherError(OtherError::new("Log4rsConfigErrors", log4rs_config_errors)));
    }
}

impl From<SetLoggerError> for BaseError {
    fn from(set_logger_error: SetLoggerError) -> Self {
        return Self::RunTimeError(RunTimeError::OtherError(OtherError::new("SetLoggerError", set_logger_error)));
    }
}

impl From<R2d2Error> for BaseError {
    fn from(r2d2_error: R2d2Error) -> Self {
        return Self::RunTimeError(RunTimeError::ResourceError(ResourceError::ConnectionPoolError(r2d2_error)));
    }
}

impl From<DieselError> for BaseError {
    fn from(diesel_error: DieselError) -> Self {
        return Self::RunTimeError(RunTimeError::ResourceError(ResourceError::PostgresqlError(diesel_error)));
    }
}

impl From<RedisError> for BaseError {
    fn from(redis_error: RedisError) -> Self {
        return Self::RunTimeError(RunTimeError::ResourceError(ResourceError::RedisError(redis_error)));
    }
}

impl From<LettreEmailError> for BaseError {
    fn from(lettre_email_error: LettreEmailError) -> Self {
        return Self::RunTimeError(RunTimeError::ResourceError(ResourceError::EmailServerError(EmailServerError::EmailError(lettre_email_error))));
    }
}

impl From<LettreSmtpError> for BaseError {
    fn from(lettre_smtp_error: LettreSmtpError) -> Self {
        return Self::RunTimeError(RunTimeError::ResourceError(ResourceError::EmailServerError(EmailServerError::SmtpError(lettre_smtp_error))));
    }
}