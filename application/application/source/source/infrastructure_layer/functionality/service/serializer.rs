use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use serde::Deserialize;
use serde::Serialize as SerdeSerialize;
use std::marker::PhantomData;

pub use super::message_pack___serializer::MessagePack;

#[cfg(feature = "manual_testing")]
pub use super::json___serializer::Json;

pub struct Serializer<T> {
    _format: PhantomData<T>,
}

pub trait Serialize {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, ErrorAuditor>
    where
        T: SerdeSerialize;

    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, ErrorAuditor>
    where
        T: Deserialize<'a>;
}
