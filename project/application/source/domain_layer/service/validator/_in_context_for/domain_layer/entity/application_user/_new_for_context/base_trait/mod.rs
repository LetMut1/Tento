use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_encoder_trait::PasswordEncoderTrait;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver_trait::PasswordHashResolverTrait;
use regex::Error as RegexError;
use regex::Regex;
use std::convert::From;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error + From<RegexError>;
    type PasswordHashResolver: PasswordHashResolverTrait;

    const EMAIL_MAXIMUM_LENGTH: u16 = 320;
    const NICKNAME_MAXIMUM_LENGTH: u8 = 55;
    const PASSWORD_MINIMUM_LENGTH: u8 = 7;
    const PASSWORD_MAXIMUM_LENGTH: u8 = 65;

    fn is_valid_email<'a>(          // TODO Перенести реализацию в инфраструктуру
        email: &'a str
    ) -> Result<bool, Self::Error> {
        let is_valid = Regex::new(r"\S+@\S+")?.is_match(email)
            && email.chars().count() <= (Self::EMAIL_MAXIMUM_LENGTH as usize);

        return Ok(is_valid);
    }

    fn is_valid_nickname<'a>(   // TODO Перенести реализацию в инфраструктуру
        nickname: &'a str
    ) -> bool {
        return nickname.chars().count() <= (Self::NICKNAME_MAXIMUM_LENGTH as usize)
            && !nickname.contains('@')
            && !nickname.contains(' ')     // TODO Проверить символ табуляци TAB
            && !nickname.is_empty();
    }

    fn is_valid_password<'a>(       // TODO Перенести реализацию в инфраструктуру
        password: &'a str
    ) -> bool {
        let password_chars_count = password.chars().count();

        return password_chars_count >= (Self::PASSWORD_MINIMUM_LENGTH as usize)
            && password_chars_count <= (Self::PASSWORD_MAXIMUM_LENGTH as usize)
            && !password.contains(' ');
    }   // TODO TODO TODO TODO TODO усилить пароль (ввести обязательность цифр,  и так далее)

    fn is_valid_password_hash<'a>( // TODO убрать вообще
        password: &'a str,
        password_hash: &'a str
    ) -> Result<bool, <<Self::PasswordHashResolver as PasswordHashResolverTrait>::PasswordEncoder as PasswordEncoderTrait>::Error> {
        return <Self::PasswordHashResolver as PasswordHashResolverTrait>::is_valid(password, password_hash);
    }
}