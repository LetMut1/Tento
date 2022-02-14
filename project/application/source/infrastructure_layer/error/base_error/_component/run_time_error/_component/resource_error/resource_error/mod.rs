use postgres::Error as PostgresqlErrorXXXxDel;
use r2d2::Error as R2d2Error;
use redis_ref::RedisError;
use tokio_postgres::Error as PostgresqlError;
use redis::RedisError as RedisEr;
use bb8::RunError as Bb8Error;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_component::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;

#[derive(Debug)]
pub enum ResourceError {
    ConnectionPoolErrorXXXxDelete {
        r2d2_error: R2d2Error
    },
    ConnectionPoolRedisError {
        bb8_redis_error: Bb8Error<RedisError>
    },
    ConnectionPoolPostgresqlError {
        bb8_postgresql_error: Bb8Error<PostgresqlError>
    },
    EmailServerError {
        email_server_error: EmailServerError
    },
    PostgresqlErrorXXXxDel {
        postgresql_error: PostgresqlErrorXXXxDel
    },
    RedisErrXXXxDel {
        redis_error: RedisEr
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

impl Error for ResourceError {}