use super::{
    Serialize,
    Serializer,
};
pub use crate::infrastructure_layer::data::control_type::MessagePack;
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    error::Error,
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
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, Auditor<Error>>
    where
        T: Deserialize<'a>,
    {
        return Ok(Serializer_::deserialize::<'_, T>(data)?);
    }
}
