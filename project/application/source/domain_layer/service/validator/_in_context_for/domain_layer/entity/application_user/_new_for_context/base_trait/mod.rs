use regex::Regex;
use std::error::Error;
use regex::Error as RegexError;
use std::convert::From;

pub trait BaseTrait {
    type Error: Error + From<RegexError>;

    const EMAIL_MAXIMUM_LENGTH: u16 = 320;
    const NICKNAME_MAXIMUM_LENGTH: u8 = 55;
    const PASSWORD_MINIMUM_LENGTH: u8 = 7;
    const PASSWORD_MAXIMUM_LENGTH: u8 = 65;

    fn is_valid_email<'a>(
        email: &'a str
    ) -> Result<bool, Self::Error> {
        let is_valid: bool = Regex::new(r"\S+@\S+")?.is_match(email)
            && email.chars().count() <= (Self::EMAIL_MAXIMUM_LENGTH as usize);

        return Ok(is_valid);
    }

    fn is_valid_nickname<'a>(
        nickname: &'a str
    ) -> bool {
        return nickname.chars().count() <= (Self::NICKNAME_MAXIMUM_LENGTH as usize);
    }

    fn is_valid_password<'a>(
        password: &'a str
    ) -> bool {
        let password_chars_count: usize = password.chars().count();

        return !password.contains(' ')
            && password_chars_count >= (Self::PASSWORD_MINIMUM_LENGTH as usize)
            && password_chars_count <= (Self::PASSWORD_MAXIMUM_LENGTH as usize);
    }   // TODO TODO TODO TODO TODO усилить пароль (ввести обязательность цифр,  и так далее)
}