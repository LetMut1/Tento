use diesel::result::Error as DieselError;
use postgres::Error as PostgresError;
use r2d2::Error as R2d2Error;
use redis::RedisError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_component::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;

#[derive(Debug)]
pub enum ResourceError {
    ConnectionPoolError {
        r2d2_error: R2d2Error
    },
    EmailServerError {
        email_server_error: EmailServerError
    },
    PostgresqlError {
        postgres_error: PostgresError
    },
    PostgresqlXXXDELETEError {
        diesel_error: DieselError
    },
    RedisError {
        redis_error: RedisError
    }
}

impl Display for ResourceError {
    fn fmt<'this, 'outer_a>(
        &'this self,
        _: &'outer_a mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

impl Error for ResourceError {}