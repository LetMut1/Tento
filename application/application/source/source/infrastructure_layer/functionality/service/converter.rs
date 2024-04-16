use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::Auditor;
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
    fn convert(subject: F) -> Result<T, Auditor<Error>>;
}

impl Convert<u16, i16> for Converter {
    fn convert(subject: u16) -> Result<i16, Auditor<Error>> {
        let converted_subject = match i16::try_from(subject) {
            Ok(converted_subject_) => converted_subject_,
            Err(error) => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                )
            }
        };

        return Ok(converted_subject);
    }
}
