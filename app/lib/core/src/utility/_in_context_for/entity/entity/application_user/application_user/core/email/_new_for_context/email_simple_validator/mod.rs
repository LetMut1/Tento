use regex::Regex;

pub struct EmailSimpleValidator;

impl<'outer> EmailSimpleValidator {
    pub fn is_valid(email: &'outer str) -> bool {
        return Regex::new(r"\S+@\S+").unwrap().is_match(email);
    }
}