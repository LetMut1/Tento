use bb8::RunError as Bb8Error;
use lettre::smtp::error::Error as SmtpError;
use lettre_email::error::Error as EmailError;
use redis::RedisError;
use std::fmt::Display;
use std::fmt::Error as FormatError;
use std::fmt::Formatter as FmtFormatter;
use tokio_postgres::Error as PostgresqlError;
use formatter::Format;
use formatter::Formatter;

#[derive(Debug)]
pub enum ResourceError {
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

impl Display for ResourceError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut FmtFormatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

impl Format<ResourceError> for Formatter<ResourceError> {
    fn prepare<'a>(subject: &'a ResourceError) -> String {
        let message = match *subject {
            ResourceError::ConnectionPoolRedis {
                ref bb8_redis_error,
            } => {
                format!(
                    "Redis connection pool: {}.",
                    bb8_redis_error,
                )
            }
            ResourceError::ConnectionPoolPostgresql {
                ref bb8_postgresql_error,
            } => {
                format!(
                    "Postgresql connection pool: {}.",
                    bb8_postgresql_error,
                )
            }
            ResourceError::EmailServer {
                ref email_server,
            } => match *email_server {
                EmailServer::Email {
                    ref email_error,
                } => {
                    format!(
                        "Email: {}.",
                        email_error,
                    )
                }
                EmailServer::Smtp {
                    ref smtp_error,
                } => {
                    format!(
                        "Email: {}.",
                        smtp_error,
                    )
                }
            },
            ResourceError::Postgresql {
                ref postgresql_error,
            } => {
                format!(
                    "Postgresql: {}.",
                    postgresql_error,
                )
            }
            ResourceError::Redis {
                ref redis_error,
            } => {
                format!(
                    "Redis: {}.",
                    redis_error
                )
            }
        };

        return message;
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
        _: &'b mut FmtFormatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}