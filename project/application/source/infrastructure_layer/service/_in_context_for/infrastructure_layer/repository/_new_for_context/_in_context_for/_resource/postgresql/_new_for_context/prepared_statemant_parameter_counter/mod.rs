use crate::infrastructure_layer::error::error_aggregator::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;

pub struct PreparedStatementParameterCounter {
    counter: u8
}

impl PreparedStatementParameterCounter {
    pub fn new(
    ) -> Self {
        return Self {
            counter: 0
        };
    }

    pub fn get_next<'a>(
        &'a mut self
    ) -> Result<&'a u8, ErrorAggregator> {
        if self.counter == u8::max_value() {
            return Err(ErrorAggregator::LogicError {logic_error: LogicError::new(false, "Out of range for `u8` type.")});
        }

        self.counter = self.counter + 1;

        return Ok(&self.counter);
    }
}