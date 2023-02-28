use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::rmp_serde;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize;

pub struct MessagePackEncoder;

impl MessagePackEncoder {
    pub fn encode<'a, T>(subject: &'a T) -> Result<Vec<u8>, ErrorAuditor>
    where
        T: Serialize
    {
        let data = match rmp_serde::to_vec(subject) {
            Ok(data_) => data_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(data);
    }

    pub fn decode<'a, T>(data: &'a [u8]) -> Result<T, ErrorAuditor>
    where
        T: for<'de> Deserialize<'de>
    {
        let subject = match rmp_serde::from_read_ref::<'_, [u8], T>(data) {
            Ok(subject_) => subject_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(subject);
    }
}