pub mod json;
pub mod message_pack;
pub mod bit_code;
use aggregate_error::AggregateError;
use serde::{
    Deserialize,
    Serialize as SerdeSerialize,
};
use std::marker::PhantomData;
pub struct Serializer<T> {
    _format: PhantomData<T>,
}
pub trait Serialize {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, AggregateError>
    where
        T: SerdeSerialize;
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, AggregateError>
    where
        T: Deserialize<'a>;
}
