use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::LogicError;

pub struct Counter {
    counter: usize
}

impl Counter {
    const STEP_SIZE: usize = 1;

    pub fn new() -> Self {
        return Self {
            counter: 0
        };
    }

    pub fn get_next<'a>(&'a mut self) -> Result<usize, ErrorAuditor> {
        self.counter = match self.counter.checked_add(Self::STEP_SIZE) {
            Some(counter) => counter,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::LogicError { logic_error: LogicError::new(false, "Out of range for `usize` type.") },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(self.counter);
    }
}