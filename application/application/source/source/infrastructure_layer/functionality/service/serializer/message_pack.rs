use super::{
    Serialize,
    Serializer,
};
use crate::infrastructure_layer::data::{
    auditor::{
        Auditor,
        Backtrace
    },
    control_type::MessagePack,
    error::Error,
    invalid_argument::InvalidArgument
};
use message_pack_serializer::Serializer as Serializer_;
use serde::{
    Deserialize,
    Serialize as SerdeSerialize,
};
impl Serialize for Serializer<MessagePack> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, Auditor<Error>>
    where
        T: SerdeSerialize,
    {
        return Ok(Serializer_::serialize(subject)?);
    }
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, Auditor<InvalidArgument>>
    where
        T: Deserialize<'a>,
    {
        return Serializer_::deserialize::<'_, T>(data).map_err(
            |_: _| -> _ {
                return Auditor::<InvalidArgument>::new(
                    InvalidArgument,
                    Backtrace::new(
                        line!(),
                        file!(),
                    )
                );
            }
        );
    }
}
