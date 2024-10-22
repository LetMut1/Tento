use aggregate_error::AggregateError;
use bit_code_serializer::Serializer as Serializer_;
use super::{
    Deserialize_,
    Serialize_,
    Serialize,
    Serializer,
};
pub struct BitCode;
impl Serialize for Serializer<BitCode> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, AggregateError>
    where
        T: Serialize_,
    {
        return Result::Ok(
            Serializer_::serialize(subject)
        );
    }
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, AggregateError>
    where
        T: Deserialize_<'a>,
    {
        return Serializer_::deserialize::<'_, T>(data);
    }
}