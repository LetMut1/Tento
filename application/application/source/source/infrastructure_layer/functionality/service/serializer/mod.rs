#[cfg(feature = "manual_testing")]
mod json;
mod bit_code;
pub use self::bit_code::BitCode;
#[cfg(feature = "manual_testing")]
pub use self::json::Json;
use bitcode::{
    Encode,
    Decode,
};
use aggregate_error::AggregateError;
#[cfg(feature = "manual_testing")]
use serde::{
    Deserialize as SerdeDeserialize,
    Serialize as SerdeSerialize,
};
use std::marker::PhantomData;
pub struct Serializer<T> {
    _format: PhantomData<T>,
}
#[cfg(not(feature = "manual_testing"))]
pub trait Serialize_: Encode {}
#[cfg(not(feature = "manual_testing"))]
impl<T> Serialize_ for T
where
    T: Encode,
{}
#[cfg(feature = "manual_testing")]
pub trait Serialize_: Encode + SerdeSerialize {}
#[cfg(feature = "manual_testing")]
impl<T> Serialize_ for T
where
    T: Encode + SerdeSerialize,
{}
#[cfg(not(feature = "manual_testing"))]
pub trait Deserialize_<'a>: Decode<'a> {}
#[cfg(not(feature = "manual_testing"))]
impl<'a, T> Deserialize_<'a> for T
where
    T: Decode<'a>,
{}
#[cfg(feature = "manual_testing")]
pub trait Deserialize_<'a>: Decode<'a> + SerdeDeserialize<'a> {}
#[cfg(feature = "manual_testing")]
impl<'a, T> Deserialize_<'a> for T
where
    T: Decode<'a> + SerdeDeserialize<'a>,
{}
pub trait Serialize {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, AggregateError>
    where
        T: Serialize_;
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, AggregateError>
    where
        T: Deserialize_<'a>;
}
