use crate::domain_layer::entity::entity::application_user::_core::email::Email;
use regex::Regex;
use crate::domain_layer::error::base_error::base_error::BaseError;

pub struct Validator;

impl Validator {
    pub fn is_valid<'outer_a>(email: &'outer_a Email) -> Result<bool, BaseError> {
        return Ok(Regex::new(r"\S+@\S+")?.is_match(email.get_value()));
    }
}