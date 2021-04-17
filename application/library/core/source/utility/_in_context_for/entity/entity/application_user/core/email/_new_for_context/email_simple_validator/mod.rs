use crate::entity::entity::application_user::core::email::Email;
use regex::Regex;

pub struct EmailSimpleValidator;

impl<'outer> EmailSimpleValidator {
    pub fn is_valid(email: &'outer Email) -> bool {
        return Regex::new(r"\S+@\S+").unwrap().is_match(email.get_value());
    }
}