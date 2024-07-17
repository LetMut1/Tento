use super::{
    Serialize,
    Serializer,
};
use crate::infrastructure_layer::data::{
    aggregate_error::AggregateError,
    control_type::MessagePack,
};
use message_pack_serializer::Serializer as Serializer_;
use serde::{
    Deserialize,
    Serialize as SerdeSerialize,
};
impl Serialize for Serializer<MessagePack> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, AggregateError>
    where
        T: SerdeSerialize,
    {
        return Serializer_::serialize(subject);
    }
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, AggregateError>
    where
        T: Deserialize<'a>,
    {
        return Serializer_::deserialize::<'_, T>(data);
    }
}
