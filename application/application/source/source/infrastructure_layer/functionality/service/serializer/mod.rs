mod json;
mod bit_code;
pub use self::bit_code::BitCode;
#[cfg(feature = "manual_testing")]
pub use self::json::Json;
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
