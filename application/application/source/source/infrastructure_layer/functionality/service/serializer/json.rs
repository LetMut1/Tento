use super::Serialize;
use super::Serializer;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use serde::Deserialize;
use serde::Serialize as SerdeSerialize;
use serde_json;

#[cfg(feature = "manual_testing")]
pub use crate::infrastructure_layer::data::control_type::Json;

#[cfg(feature = "manual_testing")]
impl Serialize for Serializer<Json> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, ErrorAuditor_>
    where
        T: SerdeSerialize,
    {
        let data = match serde_json::to_vec(subject) {
            Ok(data_) => data_,
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
                );
            }
        };

        return Ok(data);
    }

    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, ErrorAuditor_>
    where
        T: Deserialize<'a>,
    {
        let subject = match serde_json::from_slice::<'_, T>(data) {
            Ok(subject_) => subject_,
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
                );
            }
        };

        return Ok(subject);
    }
}
