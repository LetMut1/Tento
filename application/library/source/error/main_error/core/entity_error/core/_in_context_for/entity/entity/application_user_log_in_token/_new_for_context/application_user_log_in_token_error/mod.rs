use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum ApplicationUserLogInTokenError {
    InvalidValue,
    NotFound
}

impl Display for ApplicationUserLogInTokenError {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for ApplicationUserLogInTokenError {}