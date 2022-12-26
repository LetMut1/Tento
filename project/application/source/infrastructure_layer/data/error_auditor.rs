use extern_crate::bb8::RunError as Bb8Error;
use extern_crate::lettre_email::error::Error as EmailError;
use extern_crate::lettre::smtp::error::Error as SmtpError;
use extern_crate::redis::RedisError;
use extern_crate::tokio_postgres::Error as PostgresqlError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct ErrorAuditor {
    base_error: BaseError,
    simple_backtrace: SimpleBacktrace
}

impl ErrorAuditor {
    pub fn new(
        base_error: BaseError,
        backtrace_part: BacktracePart
    ) -> Self {
        return Self {
            base_error,
            simple_backtrace: SimpleBacktrace::new(backtrace_part)
        };
    }

    pub fn add_backtrace_part<'a>(
        &'a mut self,
        backtrace_part: BacktracePart
    ) -> () {
        self.simple_backtrace.add(backtrace_part);

        return ();
    }

    pub fn get_base_error(
        self
    ) -> BaseError {
        return self.base_error;
    }
}

impl Display for ErrorAuditor {
    fn fmt<'a, 'b>(
        &'a self,
        _formatter: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

impl Error for ErrorAuditor {}

#[derive(Debug)]
pub enum BaseError {
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
        _formatter: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct LogicError {
    unreachable: bool,
    message: &'static str,
}

impl LogicError {
    pub fn new(
        unreachable: bool,
        message: &'static str
    ) -> Self {
        return Self {
            unreachable,
            message
        };
    }

    pub fn get_message<'a>(
        &'a self
    ) -> &'static str {
        return self.message;
    }

    pub fn is_unreachable<'a>(
        &'a self
    ) -> &'a bool {
        return &self.unreachable;
    }
}

impl Display for LogicError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

#[derive(Debug)]
pub enum RunTimeError {
    OtherError {
        other_error: OtherError
    },
    ResourceError {
        resource_error: ResourceError
    }
}

impl Display for RunTimeError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct OtherError {
    message: String
}

impl OtherError {
    pub fn new<E>(
        error: E
    ) -> Self
    where
        E: Error
    {
        return Self {
            message: format!("{}", error)
        };
    }

    pub fn get_message<'a>(
        &'a self
    ) -> &'a str {
        return self.message.as_str();
    }
}

impl Display for OtherError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

#[derive(Debug)]
pub enum ResourceError {
    ConnectionPoolRedisError {
        bb8_redis_error: Bb8Error<RedisError>
    },
    ConnectionPoolPostgresqlError {
        bb8_postgresql_error: Bb8Error<PostgresqlError>
    },
    EmailServerError {
        email_server_error: EmailServerError
    },
    PostgresqlError {
        postgresql_error: PostgresqlError
    },
    RedisError {
        redis_error: RedisError
    }
}

impl Display for ResourceError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

#[derive(Debug)]
pub enum EmailServerError {
    EmailError {
        email_error: EmailError
    },
    SmtpError {
        smtp_error: SmtpError
    }
}

impl Display for EmailServerError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct SimpleBacktrace {
    backtrace_part_registry: Vec<BacktracePart>
}

impl SimpleBacktrace {
    pub fn new(
        backtrace_part: BacktracePart
    ) -> Self {
        return Self {
            backtrace_part_registry: vec![backtrace_part]
        };
    }

    pub fn add<'a>(
        &'a mut self,
        backtrace_part: BacktracePart
    ) -> () {
        self.backtrace_part_registry.push(backtrace_part);

        return ();
    }
}

#[derive(Debug)]
pub struct BacktracePart {
    line_number: u32,
    file_path: &'static str,
    context: Option<String>
}

impl BacktracePart {
    pub fn new(
        line_number: u32,
        file_path: &'static str,
        context: Option<String>
    ) -> Self {
        return Self {
            line_number,
            file_path,
            context
        }
    }
}