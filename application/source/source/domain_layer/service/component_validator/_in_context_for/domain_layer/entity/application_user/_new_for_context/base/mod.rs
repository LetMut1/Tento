use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use regex::Regex;

pub struct Base;

impl Base {
    const EMAIL_MAXIMUM_LENGTH: u16 = 320;
    const NICKNAME_MAXIMUM_LENGTH: u8 = 55;
    const PASSWORD_MINIMUM_LENGTH: u8 = 7;
    const PASSWORD_MAXIMUM_LENGTH: u8 = 65;

    pub fn is_valid_email<'outer_a>(
        email: &'outer_a str
    ) -> Result<bool, BaseError> {
        return Ok(
            Regex::new(r"\S+@\S+")?.is_match(email)
            && email.chars().count() <= (Self::EMAIL_MAXIMUM_LENGTH as usize)
        );
    }

    pub fn is_valid_nickname<'outer_a>(
        nickname: &'outer_a str
    ) -> bool {
        return nickname.chars().count() <= (Self::NICKNAME_MAXIMUM_LENGTH as usize);
    }

    pub fn is_valid_password<'outer_a>(
        password: &'outer_a str
    ) -> bool {
        return !password.contains(' ')
            && password.chars().count() >= (Self::PASSWORD_MINIMUM_LENGTH as usize)
            && password.chars().count() <= (Self::PASSWORD_MAXIMUM_LENGTH as usize);
    }   // TODO усилить пароль (ввести обязательность цифр,  и так далее)
}