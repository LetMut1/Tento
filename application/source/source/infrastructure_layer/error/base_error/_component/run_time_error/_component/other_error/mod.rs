use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct OtherError {
    error_kind_description: &'static str,
    message: String
}

impl OtherError {
    pub fn new<E>(
        error_kind_description: &'static str,
        error: E
    ) -> Self
    where
        E: Error
    {
        return Self {
            error_kind_description,
            message: format!("{}", error)
        };
    }

    pub fn get_error_kind_description<'this>(
        &'this self
    ) -> &'static str {
        return self.error_kind_description;
    }

    pub fn get_message<'this>(
        &'this self
    ) -> &'this str {
        return self.message.as_str();
    }
}

impl Display for OtherError {
    fn fmt<'this, 'outer_a>(
        &'this self,
        _: &'outer_a mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

impl Error for OtherError {}