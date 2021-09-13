use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct LogicError {
    message: &'static str
}

impl LogicError {
    pub fn new(
        message: &'static str
    ) -> Self {
        return Self {
            message
        };
    }

    pub fn get_message<'this>(
        &'this self
    ) -> &'static str {
        return self.message;
    }
}

impl Display for LogicError {
    fn fmt<'this, 'outer_a>(
        &'this self,
        _: &'outer_a mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

impl Error for LogicError {}