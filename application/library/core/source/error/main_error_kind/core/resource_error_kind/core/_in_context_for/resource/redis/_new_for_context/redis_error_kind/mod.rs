use redis::RedisError;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum RedisErrorKind {
    ConnectionError(RedisError),
    RuntimeError(RedisError)
}

impl Display for RedisErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for RedisErrorKind {}