use super::Serialize;
use super::Serializer;
use crate::infrastructure_layer::data::error::BacktracePart;
use crate::infrastructure_layer::data::error::Auditor;
use crate::infrastructure_layer::data::error::Runtime;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use message_pack_serializer::Serializer as Serializer_;
use serde::Deserialize;
use crate::infrastructure_layer::functionality::service::formatter::Format_;
use crate::infrastructure_layer::functionality::service::formatter::Formatter_;
use serde::Serialize as SerdeSerialize;

pub use crate::infrastructure_layer::data::control_type::MessagePack;

impl Serialize for Serializer<MessagePack> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, Auditor<Error>>
    where
        T: SerdeSerialize,
    {
        let data = match Serializer_::serialize(subject) {
            Ok(data_) => data_,
            Err(error) => {
                todo!();
                // return Err(
                //     Auditor::<Error>::new(
                //         Error::Runtime {
                //             runtime: Runtime::Other {
                //                 other: Other::new_(
                //                     Formatter_::prepare(&error).into()
                //                 ),
                //             },
                //         },
                //         BacktracePart::new(
                //             line!(),
                //             file!(),
                //         ),
                //     ),
                // );
            }
        };

        return Ok(data);
    }

    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, Auditor<Error>>
    where
        T: Deserialize<'a>,
    {
        let subject = match Serializer_::deserialize::<'_, T>(data) {
            Ok(subject_) => subject_,
            Err(error) => {
                todo!();
                // return Err(
                //     Auditor::<Error>::new(
                //         Error::Runtime {
                //             runtime: Runtime::Other {
                //                 other: Other::new_(
                //                     Formatter_::prepare(&error).into()
                //                 ),
                //             },
                //         },
                //         BacktracePart::new(
                //             line!(),
                //             file!(),
                //         ),
                //     ),
                // );
            }
        };

        return Ok(subject);
    }
}
