use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum JsonAccessWebTokenError {
    AlreadyExpired,
    InJsonAccessWebTokenBlackList,
    NotExpired,
    NotFound
}

impl Display for JsonAccessWebTokenError {
    fn fmt<'this, 'outer_a>(&'this self, _: &'outer_a mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for JsonAccessWebTokenError {}