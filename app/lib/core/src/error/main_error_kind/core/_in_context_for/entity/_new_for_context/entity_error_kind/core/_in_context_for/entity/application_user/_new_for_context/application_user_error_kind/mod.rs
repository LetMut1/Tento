use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum ApplicationUserErrorKind {
    AlreadyConfirmed,
    AlreadyExist,
    InvalidEmail,
    NotConfirmed,
    NotFound,
    WrongPassword
}

impl Display for ApplicationUserErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for ApplicationUserErrorKind {}