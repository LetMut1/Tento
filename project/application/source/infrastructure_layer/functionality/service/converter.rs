use core::marker::Sized;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use std::convert::TryFrom;

pub struct Converter;

pub trait Convert<F, T>
where
    F: Sized,
    T: Sized
{
    fn convert(subject: F) -> Result<T, ErrorAuditor>;
}

impl Convert<u16, i16> for Converter {
    fn convert(subject: u16) -> Result<i16, ErrorAuditor> {
        let converted_subject = match i16::try_from(subject) {
            Ok(converted_subject_) => converted_subject_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                )
            }
        };

        return Ok(converted_subject);
    }
}