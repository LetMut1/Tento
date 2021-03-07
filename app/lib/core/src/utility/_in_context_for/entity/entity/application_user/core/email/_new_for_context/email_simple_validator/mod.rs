use regex::Regex;

pub struct EmailSimpleValidator;

impl<'outer> EmailSimpleValidator {
    pub fn is_valid(email: &'outer String) -> bool {
        return Regex::new(r"\S+@\S+").unwrap().is_match(email.as_str());
    }
}