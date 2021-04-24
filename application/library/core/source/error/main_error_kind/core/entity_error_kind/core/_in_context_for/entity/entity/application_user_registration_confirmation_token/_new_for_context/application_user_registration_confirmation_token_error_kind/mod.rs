use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

#[derive(Debug)]
pub enum ApplicationUserRegistrationConfirmationTokenErrorKind {
    AlreadyExpired,
    InvalidValue,
    NotFound
}

impl Display for ApplicationUserRegistrationConfirmationTokenErrorKind {
    fn fmt(&self, _formatter: &mut Formatter<'_>) -> FmtResult {
        return Ok(());  // TODO 
    }
}

impl Error for ApplicationUserRegistrationConfirmationTokenErrorKind {}