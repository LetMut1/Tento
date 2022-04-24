use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_component::error_aggregator::error_aggregator::ErrorAggregator;
use super::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use super::_component::simple_backtrace::simple_backtrace::SimpleBacktrace;

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

    pub fn get_error_aggregator<'a>(&'a self) -> &'a ErrorAggregator {
        return &self.error_aggregator;
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