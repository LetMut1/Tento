use super::Serialize;
use super::Serializer;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::Other;
use message_pack_serializer::Serializer as Serializer_;
use serde::Deserialize;
use crate::infrastructure_layer::functionality::service::formatter::Format;
use crate::infrastructure_layer::functionality::service::formatter::Formatter_;
use serde::Serialize as SerdeSerialize;

pub use crate::infrastructure_layer::data::control_type::MessagePack;

impl Serialize for Serializer<MessagePack> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, ErrorAuditor>
    where
        T: SerdeSerialize,
    {
        let data = match Serializer_::serialize(subject) {
            Ok(data_) => data_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new_(
                                    Formatter_::prepare(&error).into()
                                ),
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

    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, ErrorAuditor>
    where
        T: Deserialize<'a>,
    {
        let subject = match Serializer_::deserialize::<'_, T>(data) {
            Ok(subject_) => subject_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new_(
                                    Formatter_::prepare(&error).into()
                                ),
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
