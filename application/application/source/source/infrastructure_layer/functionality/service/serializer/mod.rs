#[cfg(feature = "manual_testing")]
pub mod json;
pub mod message_pack;
use crate::infrastructure_layer::data::{
    auditor::Auditor,
    error::Error,
    invalid_argument::InvalidArgument,
};
use serde::{
    Deserialize,
    Serialize as SerdeSerialize,
};
use std::marker::PhantomData;
pub struct Serializer<T> {
    _format: PhantomData<T>,
}
pub trait Serialize {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, Auditor<Error>>
    where
        T: SerdeSerialize;
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, Auditor<InvalidArgument>>
    where
        T: Deserialize<'a>;
}