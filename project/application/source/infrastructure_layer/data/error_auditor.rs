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
}

impl Display for ErrorAuditor {
    fn fmt<'a, 'b>(&'a self, formatter: &'b mut Formatter<'_>) -> Result<(), FormatError> {
        write!(formatter, "{} > {}", &self.backtrace, &self.base_error)?;

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
    RuntimeError {
        runtime_error: RuntimeError
    }
}

impl Display for BaseError {
    fn fmt<'a, 'b>(&'a self, formatter: &'b mut Formatter<'_>) -> Result<(), FormatError> {
        match *self {
            Self::InvalidArgumentError => {
                write!(formatter, "invalid argument.")?;
            }
            Self::LogicError { ref logic_error } => {
                write!(formatter, "logic: {}.", logic_error.message)?;
            }
            Self::RuntimeError { runtime_error: ref run_time_error } => {
                match *run_time_error {
                    RuntimeError::OtherError { ref other_error } => {
                        write!(formatter, "runtime, other: {}.", other_error.message.as_str())?;
                    }
                    RuntimeError::ResourceError { ref resource_error } => {
                        match *resource_error {
                            ResourceError::ConnectionPoolRedisError { ref bb8_redis_error } => {
                                write!(formatter, "runtime, resource, Redis connection pool : {}.", bb8_redis_error)?;
                            }
                            ResourceError::ConnectionPoolPostgresqlError { ref bb8_postgresql_error } => {
                                write!(formatter, "runtime, resource, Postgresql connection pool : {}.", bb8_postgresql_error)?;
                            }
                            ResourceError::EmailServerError { ref email_server_error } => {
                                match *email_server_error {
                                    EmailServerError::EmailError { ref email_error } => {
                                        write!(formatter, "runtime, resource, email : {}.", email_error)?;
                                    }
                                    EmailServerError::SmtpError { ref smtp_error } => {
                                        write!(formatter, "runtime, resource, email : {}.", smtp_error)?;
                                    }
                                }
                            }
                            ResourceError::PostgresqlError { ref postgresql_error } => {
                                write!(formatter, "runtime, resource,  Postgresql : {}.", postgresql_error)?;
                            }
                            ResourceError::RedisError { ref redis_error } => {
                                write!(formatter, "runtime, resource, Redis : {}.", redis_error)?;
                            }
                        }
                    }
                }
            }
        }

        return Ok(());
    }
}

#[derive(Debug)]
pub struct LogicError {
    message: &'static str
}

impl LogicError {
    pub fn new(message: &'static str) -> Self {
        return Self {
            message
        };
    }
}

impl Display for LogicError {
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
}

impl Display for Backtrace {
    fn fmt<'a, 'b>(&'a self, formatter: &'b mut Formatter<'_>) -> Result<(), FormatError> {
        for (index, backtrace_part) in self.backtrace_part_registry.iter().enumerate() {
            match backtrace_part.context {
                Some(ref context) => {
                    writeln!(formatter, "({}){}:{} ({}).", index, backtrace_part.file_path, backtrace_part.line_number, context)?;
                }
                None => {
                    writeln!(formatter, "({}){}:{}.", index, backtrace_part.file_path, backtrace_part.line_number)?;
                }
            }
        }

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
}