use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;

pub struct Counter {
    counter: usize
}

impl Counter {
    pub fn new(
    ) -> Self {
        return Self {
            counter: 0
        };
    }

    pub fn get_next<'a>(
        &'a mut self
    ) -> Result<usize, ErrorAuditor> {
        if self.counter == usize::max_value() {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { logic_error: LogicError::new(false, "Out of range for `usize` type.") },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }
        self.counter += 1;

        return Ok(self.counter);
    }
}