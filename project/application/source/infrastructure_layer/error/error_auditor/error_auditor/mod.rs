use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_component::base_error::base_error::BaseError;
use super::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use super::_component::simple_backtrace::simple_backtrace::SimpleBacktrace;

#[derive(Debug)]
pub struct ErrorAuditor {
    base_error: BaseError,
    simple_backtrace: SimpleBacktrace
}

impl ErrorAuditor {
    pub fn new(
        base_error: BaseError,
        backtrace_part: BacktracePart
    ) -> Self {
        return Self {
            base_error,
            simple_backtrace: SimpleBacktrace::new(backtrace_part)
        };
    }

    pub fn add_backtrace_part<'a>(
        &'a mut self,
        backtrace_part: BacktracePart
    ) -> () {
        self.simple_backtrace.add(backtrace_part);

        return ();
    }

    pub fn get_base_error(
        self
    ) -> BaseError {
        return self.base_error;
    }
}

impl Display for ErrorAuditor {
    fn fmt<'a, 'b>(
        &'a self,
        _formatter: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}

impl Error for ErrorAuditor {}