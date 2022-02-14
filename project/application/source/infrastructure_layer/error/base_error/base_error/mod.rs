use actix_web::Error as ActixWebError;
use anyhow::Error as AnyhowError;
use argon2::Error as Argon2Error;
use base64::DecodeError as Base64DecodeError;
use bb8::RunError as Bb8Error;
use chrono::ParseError as ChronoParseError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use dotenv::Error as DotenvError;
use hyper::Error as HyperError;
use lettre_email::error::Error as LettreEmailError;
use lettre::smtp::error::Error as LettreSmtpError;
use log::SetLoggerError;
use log4rs::config::runtime::ConfigErrors as Log4rsConfigErrors;
use postgres::Error as PostgresqlErr;
use r2d2::Error as R2d2Error;
use redis_ref::RedisError;
use redis::RedisError as RedisEr;
use regex::Error as RegexError;
use rmp_serde::decode::Error as RmpSerdeDecodeError;
use rmp_serde::encode::Error as RmpSerdeEncodeError;
use serde_json::Error as SerdeJsonError;
use std::convert::From;
use std::env::VarError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::net::AddrParseError;
use std::string::FromUtf8Error;
use super::_component::logic_error::LogicError;
use super::_component::run_time_error::_component::other_error::OtherError;
use super::_component::run_time_error::_component::resource_error::_component::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;
use super::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use super::_component::run_time_error::run_time_error::RunTimeError;
use tokio_postgres::Error as PostgresqlError;

#[derive(Debug)]
pub enum BaseError {                // TODO Как понять и отследить Бэктрейс ошибки? (Например, ЛогикЕррор). Нужно ли отслеживать? Или же покрыть все функциональными тестами?
    EntityError {
        entity_error: EntityError
    },
    InvalidArgumentError,
    LogicError {
        logic_error: LogicError
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
            Self::LogicError {logic_error} => {
                if *logic_error.is_unreachable() {
                    write!(formatter, "BaseError-LogicError: [Unreachable] {}", logic_error.get_message())?;
                } else {
                    write!(formatter, "BaseError-LogicError: {}", logic_error.get_message())?;
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
                            ResourceError::ConnectionPoolErrorXXXxDelete {r2d2_error} => {
                                write!(formatter, "BaseError-RunTimeError-ResourceError-ConnectionPoolError: {}", r2d2_error)?;
                            },
                            ResourceError::ConnectionPoolRedisError {bb8_redis_error} => {
                                write!(formatter, "BaseError-RunTimeError-ResourceError-ConnectionPoolRedisError: {}", bb8_redis_error)?;
                            },
                            ResourceError::ConnectionPoolPostgresqlError {bb8_postgresql_error} => {
                                write!(formatter, "BaseError-RunTimeError-ResourceError-ConnectionPoolPostgresqlError: {}", bb8_postgresql_error)?;
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
                            ResourceError::PostgresqlErrorXXXxDel {postgresql_error} => {
                                write!(formatter, "BaseError-RunTimeError-ResourceError-PostgresqlError: {}", postgresql_error)?;
                            },
                            ResourceError::RedisErrXXXxDel {redis_error} => {
                                write!(formatter, "BaseError-RunTimeError-ResourceError-RedisError: {}", redis_error)?;
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
        return Self::LogicError {logic_error}
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

impl From<RmpSerdeEncodeError> for BaseError {
    fn from(
        rmp_serde_encode_error: RmpSerdeEncodeError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("RmpSerdeEncodeError", rmp_serde_encode_error)}};
    }
}

impl From<RmpSerdeDecodeError> for BaseError {
    fn from(
        rmp_serde_decode_error: RmpSerdeDecodeError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("RmpSerdeEncodeError", rmp_serde_decode_error)}};
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

impl From<AddrParseError> for BaseError {
    fn from(
        addr_parse_error: AddrParseError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("AddrParseError", addr_parse_error)}};
    }
}

impl From<HyperError> for BaseError {
    fn from(
        hyper_error: HyperError
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("AddrParseError", hyper_error)}};
    }
}

impl From<R2d2Error> for BaseError {
    fn from(
        r2d2_error: R2d2Error
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolErrorXXXxDelete {r2d2_error}}};
    }
}


impl From<Bb8Error<RedisError>> for BaseError {
    fn from(
        bb8_redis_error: Bb8Error<RedisError>
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolRedisError {bb8_redis_error}}};
    }
}

impl From<Bb8Error<PostgresqlError>> for BaseError {
    fn from(
        bb8_postgresql_error: Bb8Error<PostgresqlError>
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolPostgresqlError {bb8_postgresql_error}}};
    }
}

impl From<PostgresqlErr> for BaseError {
    fn from(
        postgresql_error: PostgresqlErr
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlErrorXXXxDel {postgresql_error}}};
    }
}

impl From<RedisEr> for BaseError {
    fn from(
        redis_error: RedisEr
    ) -> Self {
        return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::RedisErrXXXxDel {redis_error}}};
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