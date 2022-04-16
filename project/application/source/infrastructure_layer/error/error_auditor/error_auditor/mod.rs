use anyhow::Error as AnyhowError;
use argon2::Error as Argon2Error;
use base64::DecodeError as Base64DecodeError;
use bb8::RunError as Bb8Error;
use chrono::ParseError as ChronoParseError;
use crate::domain_layer::error::entity_error::entity_error::EntityError;
use dotenv::Error as DotenvError;
use http::Error as HttpError;
use hyper::Error as HyperError;
use lettre_email::error::Error as LettreEmailError;
use lettre::smtp::error::Error as LettreSmtpError;
use log::SetLoggerError;
use log4rs::config::runtime::ConfigErrors as Log4rsConfigErrors;
use redis::RedisError;
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
use super::_component::error_aggregator::_component::logic_error::LogicError;
use super::_component::error_aggregator::_component::run_time_error::_component::other_error::OtherError;
use super::_component::error_aggregator::_component::run_time_error::_component::resource_error::_component::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;
use super::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use super::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use super::_component::error_aggregator::error_aggregator::ErrorAggregator;
use super::_component::simple_backtrace::_component::backtrace_part;
use super::_component::simple_backtrace::simple_backtrace::SimpleBacktrace;
use super::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use tokio_postgres::Error as PostgresqlError;

#[derive(Debug)]
pub struct ErrorAuditor {
    error_aggregator: ErrorAggregator,
    simple_backtrace: SimpleBacktrace
}

impl ErrorAuditor {
    pub fn new(
        error_aggregator: ErrorAggregator,
        backtrace_part: BacktracePart
    ) -> Self {
        return Self {
            error_aggregator,
            simple_backtrace: SimpleBacktrace::new(backtrace_part)
        };
    }
}

impl Display for ErrorAuditor {
    fn fmt<'a, 'b>(
        &'a self,
        formatter: &'b mut Formatter<'_>
    ) -> Result {
        // match self {
        //     Self::LogicError {logic_error} => {
        //         if *logic_error.is_unreachable() {
        //             write!(formatter, "ErrorAuditor-LogicError: [Unreachable] {}", logic_error.get_message())?;
        //         } else {
        //             write!(formatter, "ErrorAuditor-LogicError: {}", logic_error.get_message())?;
        //         }
        //     }
        //     Self::RunTimeError {run_time_error} => {
        //         match run_time_error {
        //             RunTimeError::OtherError {other_error} => {
        //                 write!(formatter, "ErrorAuditor-RunTimeError-OtherError-{}: {}", other_error.get_error_kind_description(), other_error.get_message())?;
        //             }
        //             RunTimeError::ResourceError {resource_error} => {
        //                 match resource_error {
        //                     ResourceError::ConnectionPoolRedisError {bb8_redis_error} => {
        //                         write!(formatter, "ErrorAuditor-RunTimeError-ResourceError-ConnectionPoolRedisError: {}", bb8_redis_error)?;
        //                     }
        //                     ResourceError::ConnectionPoolPostgresqlError {bb8_postgresql_error} => {
        //                         write!(formatter, "ErrorAuditor-RunTimeError-ResourceError-ConnectionPoolPostgresqlError: {}", bb8_postgresql_error)?;
        //                     }
        //                     ResourceError::EmailServerError {email_server_error} => {
        //                         match email_server_error {
        //                             EmailServerError::EmailError {email_error} => {
        //                                 write!(formatter, "ErrorAuditor-RunTimeError-ResourceError-EmailServerError-EmailError: {}", email_error)?;
        //                             }
        //                             EmailServerError::SmtpError {smtp_error} => {
        //                                 write!(formatter, "ErrorAuditor-RunTimeError-ResourceError-EmailServerError-SmtpError: {}", smtp_error)?;
        //                             }
        //                         }
        //                     }
        //                     ResourceError::PostgresqlError {postgresql_error} => {
        //                         write!(formatter, "ErrorAuditor-RunTimeError-ResourceError-PostgresqlError: {}", postgresql_error)?;
        //                     }
        //                     ResourceError::RedisError {redis_error} => {
        //                         write!(formatter, "ErrorAuditor-RunTimeError-ResourceError-RedisError: {}", redis_error)?;
        //                     }
        //                 }
        //             }
        //         }
        //     }
        //     _ => {}
        // }

        return Ok(());
    }
}

impl Error for ErrorAuditor {}

// impl From<EntityError> for ErrorAuditor {                           // TODO Нужно ли 
//     fn from(
//         entity_error: EntityError
//     ) -> Self {
//         return Self::EntityError {entity_error};
//     }
// }

// impl From<LogicError> for ErrorAuditor {                                         // TODO Нужно ли 
//     fn from(
//         logic_error: LogicError
//     ) -> Self {
//         return Self::LogicError {logic_error}
//     }
// }

// impl From<IoError> for ErrorAuditor {
//     fn from(
//         io_error: IoError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("IoError", io_error)}};
//     }
// }

// impl From<AnyhowError> for ErrorAuditor {
//     fn from(
//         anyhow_error: AnyhowError
//     ) -> Self {
//         return Self::from(IoError::new(IoErrorKind::Other, format!("{}", anyhow_error)));
//     }
// }

// impl From<VarError> for ErrorAuditor {
//     fn from(
//         var_error: VarError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("EnvironmentVariableError", var_error)}};
//     }
// }

// impl From<SerdeJsonError> for ErrorAuditor {
//     fn from(
//         serde_json_error: SerdeJsonError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("SerdeJsonError", serde_json_error)}};
//     }
// }

// impl From<RmpSerdeEncodeError> for ErrorAuditor {
//     fn from(
//         rmp_serde_encode_error: RmpSerdeEncodeError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("RmpSerdeEncodeError", rmp_serde_encode_error)}};
//     }
// }

// impl From<RmpSerdeDecodeError> for ErrorAuditor {
//     fn from(
//         rmp_serde_decode_error: RmpSerdeDecodeError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("RmpSerdeEncodeError", rmp_serde_decode_error)}};
//     }
// }

// impl From<Base64DecodeError> for ErrorAuditor {
//     fn from(
//         base64_decode_error: Base64DecodeError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("Base64DecodeError", base64_decode_error)}};
//     }
// }

// impl From<RegexError> for ErrorAuditor {
//     fn from(
//         regex_error: RegexError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("RegexError", regex_error)}};
//     }
// }

// impl From<Argon2Error> for ErrorAuditor {
//     fn from(
//         argon2_error: Argon2Error
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("Argon2Error", argon2_error)}};
//     }
// }

// impl From<DotenvError> for ErrorAuditor {
//     fn from(
//         dotenv_error: DotenvError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("DotenvError", dotenv_error)}};
//     }
// }

// impl From<Log4rsConfigErrors> for ErrorAuditor {
//     fn from(
//         log4rs_config_errors: Log4rsConfigErrors
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("Log4rsConfigErrors", log4rs_config_errors)}};
//     }
// }

// impl From<SetLoggerError> for ErrorAuditor {
//     fn from(
//         set_logger_error: SetLoggerError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("SetLoggerError", set_logger_error)}};
//     }
// }

// impl From<FromUtf8Error> for ErrorAuditor {
//     fn from(
//         from_utf8_error: FromUtf8Error
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("FromUtf8Error", from_utf8_error)}};
//     }
// }

// impl From<ChronoParseError> for ErrorAuditor {
//     fn from(
//         chrono_parse_error: ChronoParseError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("ChronoParseError", chrono_parse_error)}};
//     }
// }

// impl From<HttpError> for ErrorAuditor {
//     fn from(
//         http_error: HttpError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("ChronoParseError", http_error)}};
//     }
// }

// impl From<AddrParseError> for ErrorAuditor {
//     fn from(
//         addr_parse_error: AddrParseError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("AddrParseError", addr_parse_error)}};
//     }
// }

// impl From<HyperError> for ErrorAuditor {
//     fn from(
//         hyper_error: HyperError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new("AddrParseError", hyper_error)}};
//     }
// }

// impl From<Bb8Error<RedisError>> for ErrorAuditor {
//     fn from(
//         bb8_redis_error: Bb8Error<RedisError>
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolRedisError {bb8_redis_error}}};
//     }
// }

// impl From<Bb8Error<PostgresqlError>> for ErrorAuditor {
//     fn from(
//         bb8_postgresql_error: Bb8Error<PostgresqlError>
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::ConnectionPoolPostgresqlError {bb8_postgresql_error}}};
//     }
// }

// impl From<PostgresqlError> for ErrorAuditor {
//     fn from(
//         postgresql_error: PostgresqlError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error}}};
//     }
// }

// impl From<RedisError> for ErrorAuditor {
//     fn from(
//         redis_error: RedisError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::RedisError {redis_error}}};
//     }
// }

// impl From<LettreEmailError> for ErrorAuditor {
//     fn from(
//         lettre_email_error: LettreEmailError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::EmailServerError {email_server_error: EmailServerError::EmailError {email_error: lettre_email_error}}}};
//     }
// }

// impl From<LettreSmtpError> for ErrorAuditor {
//     fn from(
//         lettre_smtp_error: LettreSmtpError
//     ) -> Self {
//         return Self::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::EmailServerError {email_server_error: EmailServerError::SmtpError {smtp_error: lettre_smtp_error}}}};
//     }
// }