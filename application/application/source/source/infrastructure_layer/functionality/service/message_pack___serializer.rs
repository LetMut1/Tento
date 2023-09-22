use super::serializer::Serialize;
use super::serializer::Serializer;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::Other;
use message_pack_serializer::Serializer as Serializer_;
use serde::Deserialize;
use super::formatter::Format;
use super::formatter::Formatter;
use serde::Serialize as SerdeSerialize;

pub use crate::infrastructure_layer::data::control_type::MessagePack;

impl Serialize for Serializer<MessagePack> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, ErrorAuditor_>
    where
        T: SerdeSerialize,
    {
        let data = match Serializer_::serialize(subject) {
            Ok(data_) => data_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new_(
                                    Formatter::prepare(&error).into()
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

    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, ErrorAuditor_>
    where
        T: Deserialize<'a>,
    {
        let subject = match Serializer_::deserialize::<'_, T>(data) {
            Ok(subject_) => subject_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new_(
                                    Formatter::prepare(&error).into()
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
