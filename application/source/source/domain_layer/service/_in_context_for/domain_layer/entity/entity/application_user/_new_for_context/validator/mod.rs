use crate::domain_layer::entity::entity::application_user::_component::email::Email;
use crate::domain_layer::entity::entity::application_user::_component::password::Password;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use regex::Regex;

pub struct Validator;

impl Validator {
    const EMAIL_MAXIMUM_LENGTH: u16 = 320;
    const PASSWORD_MAXIMUM_LENGTH: u8 = 7;

    pub fn is_valid_email<'outer_a>(email: &'outer_a Email) -> Result<bool, BaseError> {
        return Ok(
            Regex::new(r"\S+@\S+")?.is_match(email.get_value())
            && email.get_value().chars().count() <= (Self::EMAIL_MAXIMUM_LENGTH as usize)
        );
    }

    pub fn is_valid_password<'outer_a>(password: &'outer_a Password) -> bool {
        return !password.get_value().contains(' ') && password.get_value().chars().count() > (Self::PASSWORD_MAXIMUM_LENGTH as usize)
    }   // TODO усилить пароль (ввести обязательность цифр,  и так далее)
}