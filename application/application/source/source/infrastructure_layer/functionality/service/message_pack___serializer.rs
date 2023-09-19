use super::serializer::Serialize;
use super::serializer::Serializer;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use message_pack_serializer::Serializer as Serializer_;
use serde::Deserialize;
use serde::Serialize as SerdeSerialize;

pub use crate::infrastructure_layer::data::control_type::MessagePack;

impl Serialize for Serializer<MessagePack> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, ErrorAuditor>
    where
        T: SerdeSerialize,
    {
        let data = match Serializer_::serialize(subject) {
            Ok(data_) => data_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
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
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        return Ok(subject);
    }
}
