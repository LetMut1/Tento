use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum JsonRefreshWebTokenErrorKind {
    NotFound
}

impl Display for JsonRefreshWebTokenErrorKind {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for JsonRefreshWebTokenErrorKind {}