use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum JsonAccessWebTokenErrorKind {
    AlreadyExpired,
    InJsonAccessWebTokenBlackList,
    NotExpired
}

impl Display for JsonAccessWebTokenErrorKind {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for JsonAccessWebTokenErrorKind {}