use crate::infrastructure_layer::data::aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use num_integer::Integer;
use std::convert::TryFrom;
pub struct Converter;
pub trait Convert<F, T>
where
    F: Integer,
    T: Integer,
{
    fn convert(subject: F) -> Result<T, AggregateError>;
}
impl Convert<u16, i16> for Converter {
    fn convert(subject: u16) -> Result<i16, AggregateError> {
        return i16::try_from(subject).into_logic(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
}
