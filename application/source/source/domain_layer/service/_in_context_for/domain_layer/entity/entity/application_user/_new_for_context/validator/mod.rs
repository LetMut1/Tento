use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use crate::domain_layer::entity::entity::application_user::_core::password::Password;
use crate::domain_layer::error::base_error::base_error::BaseError;
use regex::Regex;

pub struct Validator;

impl Validator {
    pub fn is_valid_email<'outer_a>(email: &'outer_a Email) -> Result<bool, BaseError> {
        return Ok(Regex::new(r"\S+@\S+")?.is_match(email.get_value()));
    }

    pub fn is_valid_password<'outer_a>(password: &'outer_a Password) -> bool {
        return !password.get_value().contains(' ') && password.get_value().chars().count() > 7;     // TODO усилить пароль (ввести обязательность цифр,  и так далее)
    }
}