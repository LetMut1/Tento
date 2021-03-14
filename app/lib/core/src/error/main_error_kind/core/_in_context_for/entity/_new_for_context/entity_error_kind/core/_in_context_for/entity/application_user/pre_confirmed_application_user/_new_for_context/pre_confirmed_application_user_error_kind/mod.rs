use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum PreConfirmedApplicationUserErrorKind {
    AlreadyExist,
    AlreadyRegistered,
    NotFound,
}

impl Display for PreConfirmedApplicationUserErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for PreConfirmedApplicationUserErrorKind {}