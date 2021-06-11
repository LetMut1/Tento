use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum ApplicationUserResetPasswordTokenErrorKind {
    InvalidValue,
    NotFound,
}

impl Display for ApplicationUserResetPasswordTokenErrorKind {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for ApplicationUserResetPasswordTokenErrorKind {}