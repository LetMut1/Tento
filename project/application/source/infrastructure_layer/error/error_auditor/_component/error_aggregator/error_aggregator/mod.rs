use crate::domain_layer::error::entity_error::entity_error::EntityError;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_component::logic_error::LogicError;
use super::_component::run_time_error::run_time_error::RunTimeError;


#[derive(Debug)]
pub enum ErrorAggregator {
    EntityError {
        entity_error: EntityError
    },
    InvalidArgumentError,
    LogicError {
        logic_error: LogicError
    },
    RunTimeError {
        run_time_error: RunTimeError
    }
}

impl Display for ErrorAggregator {
    fn fmt<'a, 'b>(
        &'a self,
        _formatter: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}