use actix_web::Error as ActixWebError;
use anyhow::Error as AnyhowError;
use argon2::Error as Argon2Error;
use base64::DecodeError as Base64DecodeError;
use chrono::ParseError as ChronoParseError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use crate::domain_layer::error::logic_error::LogicError;
use dotenv::Error as DotenvError;
use lettre_email::error::Error as LettreEmailError;
use lettre::smtp::error::Error as LettreSmtpError;
use log::SetLoggerError;
use log4rs::config::runtime::ConfigErrors as Log4rsConfigErrors;
use postgres::Error as PostgresqlError;
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
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::string::FromUtf8Error;
use super::_component::run_time_error::_component::other_error::OtherError;
use super::_component::run_time_error::_component::resource_error::_component::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;
use super::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use super::_component::run_time_error::run_time_error::RunTimeError;

#[derive(Debug)]
pub enum BaseError {                // TODO Как понять и отследить Бэктрейс ошибки? (Например, ЛогикЕррор). Нужно ли отслеживать? Или же покрыть все функциональными тестами?
    EntityError {
        entity_error: EntityError
    },
    InvalidArgumentError,
    LogicError {
        unreachable: bool,
        message: &'static str,
    },
    RunTimeError {
        run_time_error: RunTimeError
    }
}

impl Display for BaseError {
    fn fmt<'a, 'b>(
        &'a self,
        formatter: &'b mut Formatter<'_>
    ) -> Result {
        match self {
            Self::LogicError {unreachable, message} => {
                if *unreachable {
                    write!(formatter, "BaseError-LogicError: [Unreachable] {}", message)?;
                } else {
                    write!(formatter, "BaseError-LogicError: {}", message)?;
                }
            },
            Self::RunTimeError {run_time_error} => {
                match run_time_error {
                    RunTimeError::ActixWebError {actix_web_error} => {
                        write!(formatter, "BaseError-RunTimeError-ActixWebError: {}", actix_web_error)?;
                    },
                    RunTimeError::OtherError {other_error} => {
                        write!(formatter, "BaseError-RunTimeError-OtherError-{}: {}", other_error.get_error_kind_description(), other_error.get_message())?;
                    },
                    RunTimeError::ResourceError {resource_error} => {
                        match resource_error {
                            ResourceError::ConnectionPoolError {r2d2_error} => {
                                write!(formatter, "BaseError-RunTimeError-ResourceError-ConnectionPoolError: {}", r2d2_error)?;
                            },
                            ResourceError::EmailServerError {email_server_error} => {
                                match email_server_error {
                                    EmailServerError::EmailError {email_error} => {
                                        write!(formatter, "BaseError-RunTimeError-ResourceError-EmailServerError-EmailError: {}", email_error)?;
                                    },
                                    EmailServerError::SmtpError {smtp_error} => {
                                        write!(formatter, "BaseError-RunTimeError-ResourceError-EmailServerError-SmtpError: {}", smtp_error)?;
                                    }
                                }
                            },
                            ResourceError::PostgresqlError {postgresql_error} => {
                                write!(formatter, "BaseError-RunTimeError-ResourceError-PostgresqlError: {}", postgresql_error)?;
                            },
                            ResourceError::RedisError {redis_error} => {
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

impl From<EntityError> for BaseError {
    fn from(
        entity_error: EntityError
    ) -> Self {
        return Self::EntityError {entity_error};
    }
}

impl From<LogicError> for BaseError {
    fn from(
        logic_error: LogicError
    ) -> Self {
        return Self::LogicError {unreachable: false, message: logic_error.get_message()};
    }
}

impl From<ActixWebError> for BaseError {
    fn from(
        actix_web_error: ActixWebError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::ActixWebError {actix_web_error}};
    }
}

impl From<IoError> for BaseError {
    fn from(
        io_error: IoError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("IoError", io_error)}};
    }
}

impl From<AnyhowError> for BaseError {
    fn from(
        anyhow_error: AnyhowError
    ) -> Self {
        return Self::from(IoError::new(IoErrorKind::Other, format!("{}", anyhow_error)));
    }
}

impl From<VarError> for BaseError {
    fn from(
        var_error: VarError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("EnvironmentVariableError", var_error)}};
    }
}

impl From<SerdeJsonError> for BaseError {
    fn from(
        serde_json_error: SerdeJsonError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("SerdeJsonError", serde_json_error)}};
    }
}

impl From<Base64DecodeError> for BaseError {
    fn from(
        base64_decode_error: Base64DecodeError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("Base64DecodeError", base64_decode_error)}};
    }
}

impl From<RegexError> for BaseError {
    fn from(
        regex_error: RegexError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("RegexError", regex_error)}};
    }
}

impl From<Argon2Error> for BaseError {
    fn from(
        argon2_error: Argon2Error
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("Argon2Error", argon2_error)}};
    }
}

impl From<DotenvError> for BaseError {
    fn from(
        dotenv_error: DotenvError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("DotenvError", dotenv_error)}};
    }
}

impl From<Log4rsConfigErrors> for BaseError {
    fn from(
        log4rs_config_errors: Log4rsConfigErrors
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("Log4rsConfigErrors", log4rs_config_errors)}};
    }
}

impl From<SetLoggerError> for BaseError {
    fn from(
        set_logger_error: SetLoggerError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("SetLoggerError", set_logger_error)}};
    }
}

impl From<FromUtf8Error> for BaseError {
    fn from(
        from_utf8_error: FromUtf8Error
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("FromUtf8Error", from_utf8_error)}};
    }
}

impl From<ChronoParseError> for BaseError {
    fn from(
        chrono_parse_error: ChronoParseError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("ChronoParseError", chrono_parse_error)}};
    }
}

impl From<R2d2Error> for BaseError {
    fn from(
        r2d2_error: R2d2Error
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolError {r2d2_error}}};
    }
}

impl From<PostgresqlError> for BaseError {
    fn from(
        postgresql_error: PostgresqlError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error}}};
    }
}

impl From<RedisError> for BaseError {
    fn from(
        redis_error: RedisError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::RedisError {redis_error}}};
    }
}

impl From<LettreEmailError> for BaseError {
    fn from(
        lettre_email_error: LettreEmailError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::EmailServerError {email_server_error: EmailServerError::EmailError {email_error: lettre_email_error}}}};
    }
}

impl From<LettreSmtpError> for BaseError {
    fn from(
        lettre_smtp_error: LettreSmtpError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::EmailServerError {email_server_error: EmailServerError::SmtpError {smtp_error: lettre_smtp_error}}}};
    }
}