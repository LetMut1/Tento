use crate::infrastructure_layer::data::{
    auditor::{
        Auditor,
        Backtrace,
        ErrorConverter as _,
    },
    error::Error,
};
use core::marker::Sized;
use std::convert::TryFrom;
pub struct Converter;
pub trait Convert<F, T>
where
    F: Sized,
    T: Sized,
{
    fn convert(subject: F) -> Result<T, Auditor<Error>>;
}
impl Convert<u16, i16> for Converter {
    fn convert(subject: u16) -> Result<i16, Auditor<Error>> {
        return Ok(i16::try_from(subject).convert(Backtrace::new(line!(), file!()))?);
    }
}
