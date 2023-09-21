use bb8::RunError as Bb8Error;
use lettre::smtp::error::Error as SmtpError;
use lettre_email::error::Error as EmailError;
use redis::RedisError;
use std::error::Error as StdError;
use std::fmt::Display;
use std::fmt::Error as FormatError;
use std::fmt::Formatter;
use tokio_postgres::Error as PostgresqlError;

#[derive(Debug)]
pub struct ErrorAuditor {
    error: Error,
    backtrace: Backtrace,
}

impl ErrorAuditor {
    pub fn new(
        error: Error,
        backtrace_part: BacktracePart,
    ) -> Self {
        return Self {
            error,
            backtrace: Backtrace::new(backtrace_part),
        };
    }

    pub fn add_backtrace_part<'a>(
        &'a mut self,
        backtrace_part: BacktracePart,
    ) -> () {
        self.backtrace.add(backtrace_part);

        return ();
    }

    pub fn get_base_error<'a>(&'a self) -> &'a Error {
        return &self.error;
    }

    pub fn get_backtrace<'a>(&'a self) -> &'a Backtrace {
        return &self.backtrace;
    }
}

impl Display for ErrorAuditor {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

impl StdError for ErrorAuditor {}

#[derive(Debug)]
pub enum Error {
    Logic {
        message: &'static str,
    },
    Runtime {
        runtime: Runtime,
    },
}

impl Error {
    pub fn create_unreachable_state() -> Self {
        return Self::Logic {
            message: "Unreachable state.",
        };
    }

    pub fn create_out_of_range() -> Self {
        return Self::Logic {
            message: "Out of range.",
        };
    }
}

impl Display for Error {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub enum Runtime {
    Other {
        other: Other,
    },
    Resource {
        resource: Resource,
    },
}

impl Display for Runtime {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct Other {
    message: String,
}

impl Other {
    pub fn new<E>(error: E) -> Self
    where
        E: StdError,
    {
        return Self {
            message: format!(
                "{}",
                error
            ),
        };
    }

    pub fn get_message<'a>(&'a self) -> &'a str {
        return self.message.as_str();
    }
}

impl Display for Other {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub enum Resource {
    ConnectionPoolRedis {
        bb8_redis_error: Bb8Error<RedisError>,
    },
    ConnectionPoolPostgresql {
        bb8_postgresql_error: Bb8Error<PostgresqlError>,
    },
    EmailServer {
        email_server: EmailServer,
    },
    Postgresql {
        postgresql_error: PostgresqlError,
    },
    Redis {
        redis_error: RedisError,
    },
}

impl Display for Resource {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub enum EmailServer {
    Email {
        email_error: EmailError,
    },
    Smtp {
        smtp_error: SmtpError,
    },
}

impl Display for EmailServer {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct Backtrace {
    backtrace_part_registry: Vec<BacktracePart>,
}

impl Backtrace {
    pub fn new(backtrace_part: BacktracePart) -> Self {
        return Self {
            backtrace_part_registry: vec![backtrace_part],
        };
    }

    pub fn add<'a>(
        &'a mut self,
        backtrace_part: BacktracePart,
    ) -> () {
        self.backtrace_part_registry.push(backtrace_part);

        return ();
    }

    pub fn get_backtrace_part_registry<'a>(&'a self) -> &'a [BacktracePart] {
        return self.backtrace_part_registry.as_slice();
    }
}

impl Display for Backtrace {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct BacktracePart {
    line_number: u32,
    file_path: &'static str,
    context: Option<String>,
}

impl BacktracePart {
    pub fn new(
        line_number: u32,
        file_path: &'static str,
        context: Option<String>,
    ) -> Self {
        return Self {
            line_number,
            file_path,
            context,
        };
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
