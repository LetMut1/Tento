use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use core::marker::Sized;
use std::convert::TryFrom;

pub struct Converter;

pub trait Convert<F, T>
where
    F: Sized,
    T: Sized,
{
    fn convert(subject: F) -> Result<T, ErrorAuditor_>;
}

impl Convert<u16, i16> for Converter {
    fn convert(subject: u16) -> Result<i16, ErrorAuditor_> {
        let converted_subject = match i16::try_from(subject) {
            Ok(converted_subject_) => converted_subject_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                )
            }
        };

        return Ok(converted_subject);
    }
}
