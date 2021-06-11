use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum ApplicationUserRegistrationConfirmationTokenErrorKind {
    InvalidValue,
    NotFound
}

impl Display for ApplicationUserRegistrationConfirmationTokenErrorKind {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for ApplicationUserRegistrationConfirmationTokenErrorKind {}