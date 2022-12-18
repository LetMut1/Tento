use extern_crate::bb8::RunError as Bb8Error;
use extern_crate::redis::RedisError;
use extern_crate::tokio_postgres::Error as PostgresqlError;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_component::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;

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