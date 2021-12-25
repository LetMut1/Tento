use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct LogicError {
    unreachable: bool,
    message: &'static str,
}

impl LogicError {
    pub fn new(
        unreachable: bool,
        message: &'static str
    ) -> Self {
        return Self {
            unreachable,
            message
        };
    }

    pub fn get_message<'a>(
        &'a self
    ) -> &'static str {
        return self.message;
    }

    pub fn is_unreachable<'a>(
        &'a self
    ) -> &'a bool {
        return &self.unreachable;
    }
}

impl Display for LogicError {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

impl Error for LogicError {}