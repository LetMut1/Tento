use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum ApplicationUserLogInTokenErrorKind {
    InvalidValue,
    NotFound
}

impl Display for ApplicationUserLogInTokenErrorKind {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for ApplicationUserLogInTokenErrorKind {}