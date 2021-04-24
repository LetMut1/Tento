use redis::RedisError;
use std::convert::From;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum RedisErrorKind {
    ConnectionError(RedisError)     // TODO посмотреть, нужно ли разделять тут на Коннектион и рантайм
}

impl Display for RedisErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for RedisErrorKind {}