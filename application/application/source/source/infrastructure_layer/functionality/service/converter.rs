use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::ErrorConverter as _;
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
        return Ok(i16::try_from(subject).convert(BacktracePart::new(line!(), file!()))?);
    }
}
