mod bit_code;
#[cfg(feature = "json_for_manual_test")]
mod json;
pub use self::bit_code::BitCode;
#[cfg(feature = "json_for_manual_test")]
pub use self::json::Json;
use crate::infrastructure_layer::data::aggregate_error::AggregateError;
use bitcode::{
    Decode,
    Encode,
};
#[cfg(feature = "serde_for_manual_test")]
use serde::{
    Deserialize as SerdeDeserialize,
    Serialize as SerdeSerialize,
};
use std::marker::PhantomData;
pub struct Serializer<T> {
    _format: PhantomData<T>,
}
#[cfg(not(feature = "serde_for_manual_test"))]
pub trait Serialize_: Encode {}
#[cfg(not(feature = "serde_for_manual_test"))]
impl<T> Serialize_ for T where T: Encode {}
#[cfg(feature = "serde_for_manual_test")]
pub trait Serialize_: Encode + SerdeSerialize {}
#[cfg(feature = "serde_for_manual_test")]
impl<T> Serialize_ for T where T: Encode + SerdeSerialize {}
#[cfg(not(feature = "serde_for_manual_test"))]
pub trait Deserialize_<'a>: Decode<'a> {}
#[cfg(not(feature = "serde_for_manual_test"))]
impl<'a, T> Deserialize_<'a> for T where T: Decode<'a> {}
#[cfg(feature = "serde_for_manual_test")]
pub trait Deserialize_<'a>: Decode<'a> + SerdeDeserialize<'a> {}
#[cfg(feature = "serde_for_manual_test")]
impl<'a, T> Deserialize_<'a> for T where T: Decode<'a> + SerdeDeserialize<'a> {}
pub trait Serialize {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, AggregateError>
    where
        T: Serialize_;
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, AggregateError>
    where
        T: Deserialize_<'a>;
}
