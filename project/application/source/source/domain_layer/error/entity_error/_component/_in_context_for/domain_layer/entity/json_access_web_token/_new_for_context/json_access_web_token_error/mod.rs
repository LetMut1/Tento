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
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

impl Error for JsonAccessWebTokenError {}