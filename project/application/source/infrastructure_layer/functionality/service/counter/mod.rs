use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;

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