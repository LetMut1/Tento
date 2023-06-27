use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use extern_crate::serde::Deserialize;
use extern_crate::serde::Serialize as SerdeSerialize;
use std::marker::PhantomData;

pub use super::message_pack__serializer::MessagePack;

#[cfg(feature = "manual_testing")]
pub use super::json__serializer::Json;

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
