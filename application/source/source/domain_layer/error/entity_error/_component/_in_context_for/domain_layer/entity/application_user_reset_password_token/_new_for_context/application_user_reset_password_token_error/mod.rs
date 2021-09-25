use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum ApplicationUserResetPasswordTokenError {
    InvalidValue,
    NotFound
}

impl Display for ApplicationUserResetPasswordTokenError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

impl Error for ApplicationUserResetPasswordTokenError {}