use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum ApplicationUserError {
    AlreadyExist,
    InvalidEmail,
    InvalidNickname,
    InvalidPassword,
    NotFound,
    WrongPassword
}

impl Display for ApplicationUserError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

impl Error for ApplicationUserError {}