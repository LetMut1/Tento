use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct OtherError {
    description: &'static str,
    displaying: String
}

impl OtherError {
    pub fn new<E>(description: &'static str, error: E) -> Self
    where
        E: Error
    {
        return Self {
            description,
            displaying: format!("{}", error)
        };
    }

    pub fn get_description<'this>(&'this self) -> &'static str {
        return self.description;
    }

    pub fn get_displaying<'this>(&'this self) -> &'this str {
        return self.displaying.as_str();
    }
}

impl Display for OtherError {
    fn fmt<'this, 'outer_a>(&'this self, _: &'outer_a mut Formatter<'_>) -> Result {
        return Ok(());
    }
}

impl Error for OtherError {}