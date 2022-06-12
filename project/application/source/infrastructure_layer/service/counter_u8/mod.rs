use crate::infrastructure_layer::error::error_auditor::_component::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;

pub struct CounterU8 {
    counter: u8
}

impl CounterU8 {
    pub fn new(
    ) -> Self {
        return Self {
            counter: 0
        };
    }

    pub fn get_next<'a>(
        &'a mut self
    ) -> Result<u8, ErrorAuditor> {
        if self.counter == u8::max_value() {
            return Err(
                ErrorAuditor::new(
                    BaseError::LogicError { logic_error: LogicError::new(false, "Out of range for `u8` type.") },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }
        self.counter = self.counter + 1;

        return Ok(self.counter);
    }
}