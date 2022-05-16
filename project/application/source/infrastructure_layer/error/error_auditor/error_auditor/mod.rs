use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use super::_component::error_aggregator::error_aggregator::ErrorAggregator;
use super::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use super::_component::simple_backtrace::simple_backtrace::SimpleBacktrace;

#[derive(Debug)]
pub struct ErrorAuditor {
    error_aggregator: ErrorAggregator,
    simple_backtrace: SimpleBacktrace
}

impl ErrorAuditor {
    pub fn new(
        error_aggregator: ErrorAggregator,
        backtrace_part: BacktracePart
    ) -> Self {
        return Self {
            error_aggregator,
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

    pub fn get_error_aggregator<'a>(
        &'a self
    ) -> &'a ErrorAggregator {
        return &self.error_aggregator;
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