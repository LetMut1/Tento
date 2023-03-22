use extern_crate::bb8::RunError as Bb8Error;
use extern_crate::lettre_email::error::Error as EmailError;
use extern_crate::lettre::smtp::error::Error as SmtpError;
use extern_crate::redis::RedisError;
use extern_crate::tokio_postgres::Error as PostgresqlError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Error as FormatError;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct ErrorAuditor {
    base_error: BaseError,
    backtrace: Backtrace
}

impl ErrorAuditor {
    pub fn new(base_error: BaseError, backtrace_part: BacktracePart) -> Self {
        return Self {
            base_error,
            backtrace: Backtrace::new(backtrace_part)
        };
    }

    pub fn add_backtrace_part<'a>(&'a mut self, backtrace_part: BacktracePart) -> () {
        self.backtrace.add(backtrace_part);

        return ();
    }

    pub fn get_base_error<'a>(&'a self) -> &'a BaseError {
        return &self.base_error;
    }

    pub fn get_backtrace<'a>(&'a self) -> &'a Backtrace {
        return &self.backtrace;
    }
}

impl Display for ErrorAuditor {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FormatError> {
        return Ok(());
    }
}

impl Error for ErrorAuditor {}

#[derive(Debug)]
pub enum BaseError {
    LogicError {
        message: &'static str
    },
    RuntimeError {
        runtime_error: RuntimeError
    }
}

impl BaseError {
    pub fn create_unreachable_state() -> Self {
        return Self::LogicError { message: "Unreachable state." }
    }

    pub fn create_out_of_range() -> Self {
        return Self::LogicError { message: "Out of range." }
    }
}

impl Display for BaseError {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    OtherError {
        other_error: OtherError
    },
    ResourceError {
        resource_error: ResourceError
    }
}

impl Display for RuntimeError {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct OtherError {
    message: String
}

impl OtherError {
    pub fn new<E>(error: E) -> Self
    where
        E: Error
    {
        return Self {
            message: format!("{}", error)
        };
    }

    pub fn get_message<'a>(&'a self) -> &'a str {
        return self.message.as_str();
    }
}

impl Display for OtherError {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FormatError> {
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
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FormatError> {
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
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct Backtrace {
    backtrace_part_registry: Vec<BacktracePart>
}

impl Backtrace {
    pub fn new(backtrace_part: BacktracePart) -> Self {
        return Self {
            backtrace_part_registry: vec![backtrace_part]
        };
    }

    pub fn add<'a>(&'a mut self, backtrace_part: BacktracePart) -> () {
        self.backtrace_part_registry.push(backtrace_part);

        return ();
    }

    pub fn get_backtrace_part_registry<'a>(&'a self) -> &'a Vec<BacktracePart> {
        return &self.backtrace_part_registry;
    }
}

impl Display for Backtrace {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct BacktracePart {
    line_number: u32,
    file_path: &'static str,
    context: Option<String>
}

impl BacktracePart {
    pub fn new(line_number: u32, file_path: &'static str, context: Option<String>) -> Self {
        return Self {
            line_number,
            file_path,
            context
        }
    }

    pub fn get_line_number<'a>(&'a self) -> u32 {
        return self.line_number;
    }

    pub fn get_file_path<'a>(&'a self) -> &'static str {
        return self.file_path;
    }

    pub fn get_context<'a>(&'a self) -> Option<&'a str> {
        return self.context.as_deref();
    }
}