use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;

pub struct Counter<T> {
    value: T,
    step_size: T,
}

impl Counter<i16> {
    const CLASSIC_STEP_SIZE: i16 = 1;

    pub fn new(
        value: i16,
        step_size: i16,
    ) -> Self {
        return Self {
            value,
            step_size,
        };
    }

    pub fn new_classic() -> Self {
        return Self {
            value: 0,
            step_size: Self::CLASSIC_STEP_SIZE,
        };
    }

    pub fn get_next_value<'a>(&'a mut self) -> Result<i16, ErrorAuditor_> {
        self.value = match self.value.checked_add(self.step_size) {
            Some(value_) => value_,
            None => {
                return Err(
                    ErrorAuditor_::new(
                        Error::create_out_of_range(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        return Ok(self.value);
    }
}
