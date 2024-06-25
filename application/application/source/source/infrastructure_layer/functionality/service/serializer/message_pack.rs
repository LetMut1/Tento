use super::{
    Serialize,
    Serializer,
};
use crate::infrastructure_layer::data::{
    control_type::MessagePack,
    error::Error,
};
use message_pack_serializer::Serializer as Serializer_;
use serde::{
    Deserialize,
    Serialize as SerdeSerialize,
};
impl Serialize for Serializer<MessagePack> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, Error>
    where
        T: SerdeSerialize,
    {
        return Serializer_::serialize(subject);
    }
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, Error>
    where
        T: Deserialize<'a>,
    {
        return Serializer_::deserialize::<'_, T>(data);
    }
}
