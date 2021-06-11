use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum ApplicationUserErrorKind {
    AlreadyConfirmed,
    AlreadyExist,
    InvalidEmail,
    NotFound,
    WrongPassword
}

impl Display for ApplicationUserErrorKind {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for ApplicationUserErrorKind {}