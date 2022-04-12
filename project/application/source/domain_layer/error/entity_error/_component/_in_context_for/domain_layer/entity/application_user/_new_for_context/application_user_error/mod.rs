use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub enum ApplicationUserError {
    EmailAlreadyExist,
    InvalidEmail,
    InvalidNickname,
    InvalidPassword,
    NicknameAlreadyExist,
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