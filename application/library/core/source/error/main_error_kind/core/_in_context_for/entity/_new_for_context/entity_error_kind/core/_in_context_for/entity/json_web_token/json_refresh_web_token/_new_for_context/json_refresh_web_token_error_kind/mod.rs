use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum JsonRefreshWebTokenErrorKind {
    NotExist
}

impl Display for JsonRefreshWebTokenErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for JsonRefreshWebTokenErrorKind {}