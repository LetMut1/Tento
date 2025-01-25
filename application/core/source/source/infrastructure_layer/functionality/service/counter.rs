use crate::infrastructure_layer::data::aggregate_error::AggregateError;
use num_integer::Integer;
use std::fmt::Display;
pub struct Counter<T>
where
    T: Integer + Display,
{
    value: T,
    step_size: T,
}
impl<T> Counter<T>
where
    T: Integer + Display,
{
    pub fn new(value: T, step_size: T) -> Self {
        return Self {
            value,
            step_size,
        };
    }
}
pub trait Counter_ {
    fn get_next_value<'a>(&'a mut self) -> Result<impl Integer + Display, AggregateError>;
    fn get_next_value_unchecked<'a>(&'a mut self) -> impl Integer + Display;
}
impl Counter<u8> {
    pub fn new_classic() -> Self {
        return Self::new(
            0,
            1,
        );
    }
}
impl Counter_ for Counter<u8> {
    fn get_next_value<'a>(&'a mut self) -> Result<impl Integer + Display, AggregateError> {
        self.value = crate::option_return_logic_out_of_range!(self.value.checked_add(self.step_size));
        return Result::Ok(self.value);
    }
    fn get_next_value_unchecked<'a>(&'a mut self) -> impl Integer + Display {
        self.value += 1;
        return self.value;
    }
}
