use crate::entity::entity::application_user::_core::email::Email;
use regex::Regex;

pub struct EmailSimpleValidator;

impl EmailSimpleValidator {
    pub fn is_valid<'outer_a>(email: &'outer_a Email) -> bool {
        return Regex::new(r"\S+@\S+").unwrap().is_match(email.get_value());
    }
}