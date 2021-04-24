use redis::RedisError;
use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub struct RedisConnectionError {
    redis_error: RedisError
}

impl RedisConnectionError {
    pub fn new(redis_error: RedisError) -> Self {
        return Self {
            redis_error
        };
    }
}

impl Display for RedisConnectionError {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for RedisConnectionError {}