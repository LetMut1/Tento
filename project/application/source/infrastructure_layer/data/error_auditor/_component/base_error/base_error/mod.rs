use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_component::logic_error::LogicError;
use super::_component::run_time_error::run_time_error::RunTimeError;

#[derive(Debug)]
pub enum BaseError {
    InvalidArgumentError,
    LogicError {
        logic_error: LogicError
    },
    RunTimeError {
        run_time_error: RunTimeError
    }
}

impl Display for BaseError {
    fn fmt<'a, 'b>(
        &'a self,
        _formatter: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}