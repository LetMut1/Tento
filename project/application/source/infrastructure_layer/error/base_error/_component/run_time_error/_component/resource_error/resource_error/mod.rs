use postgres::Error as PostgresqlError;
use r2d2::Error as R2d2Error;
use redis_ref::RedisError;
use redis::RedisError as RedisEr;
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
    EmailServerError {
        email_server_error: EmailServerError
    },
    PostgresqlError {
        postgresql_error: PostgresqlError
    },
    RedisErr {
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