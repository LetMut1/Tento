use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct OtherError {
    message: String
}

impl OtherError {
    pub fn new<E>(
        error: E
    ) -> Self
    where
        E: Error
    {
        return Self {
            message: format!("{}", error)
        };
    }

    pub fn get_message<'a>(
        &'a self
    ) -> &'a str {
        return self.message.as_str();
    }
}

impl Display for OtherError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}