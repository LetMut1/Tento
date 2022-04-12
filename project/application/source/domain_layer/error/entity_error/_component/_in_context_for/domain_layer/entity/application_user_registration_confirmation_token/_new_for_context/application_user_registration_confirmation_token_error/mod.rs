use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum ApplicationUserRegistrationConfirmationTokenError {
    InvalidValue,
    NotFound
}

impl Display for ApplicationUserRegistrationConfirmationTokenError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}