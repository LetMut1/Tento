use redis::RedisError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum RedisErrorKind {
    ConnectionError(RedisError),
    RuntimeError(RedisError)
}

impl Display for RedisErrorKind {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for RedisErrorKind {}