use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub struct PreparedStatementParameterCounter {
    counter: u8
}

impl PreparedStatementParameterCounter {
    pub fn new() -> Self {
        return Self {
            counter: 0
        };
    }

    pub fn get_next<'this>(&'this mut self) -> Result<u8, BaseError> {
        if self.counter != u8::max_value() {
            self.counter = self.counter + 1;

            return Ok(self.counter);
        }

        return Err(BaseError::LogicError("Out of range for `u8` type."));
    }
}