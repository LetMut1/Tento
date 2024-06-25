use crate::infrastructure_layer::data::{
    auditor::{
        Backtrace,
    },
    alternative_workflow::AlternativeWorkflow,
    alternative_workflow::ResultConverter,
};
use core::marker::Sized;
use std::convert::TryFrom;
pub struct Converter;
pub trait Convert<F, T>
where
    F: Sized,
    T: Sized,
{
    fn convert(subject: F) -> Result<T, AlternativeWorkflow>;
}
impl Convert<u16, i16> for Converter {
    fn convert(subject: u16) -> Result<i16, AlternativeWorkflow> {
        return i16::try_from(subject).into_internal_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
}
