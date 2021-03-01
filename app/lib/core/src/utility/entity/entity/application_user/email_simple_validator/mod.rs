use regex::Regex;

pub struct EmailSimpleValidator;

impl<'outer> EmailSimpleValidator {
    pub fn is_valid(email: &'outer String) -> bool {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"\S+@\S+").unwrap();
        }

        return REGEX.is_match(email.as_str());
    }
}