use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use redis::RedisError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_core::_in_context_for::_resource::email_server::_new_for_context::email_server_error::EmailServerError;

#[derive(Debug)]
pub enum ResourceError {
    ConnectionPoolError(R2d2Error),
    EmailServerError(EmailServerError),
    PostgresqlError(DieselError),
    RedisError(RedisError)
}

impl Display for ResourceError {
    fn fmt<'this, 'outer_a>(&'this self, _: &'outer_a mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for ResourceError {}