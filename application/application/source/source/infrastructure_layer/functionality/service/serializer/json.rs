use super::{
    Serialize,
    Serializer,
};
#[cfg(feature = "manual_testing")]
use crate::infrastructure_layer::data::control_type::Json;
use crate::infrastructure_layer::data::{
    auditor::{
        Auditor,
        Backtrace,
        ResultConverter,
    },
    error::Error,
};
use serde::{
    Deserialize,
    Serialize as SerdeSerialize,
};
use serde_json;
#[cfg(feature = "manual_testing")]
impl Serialize for Serializer<Json> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, Auditor<Error>>
    where
        T: SerdeSerialize,
    {
        return Ok(serde_json::to_vec(subject).convert_into_error(Backtrace::new(line!(), file!()))?);
    }
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, Auditor<Error>>
    where
        T: Deserialize<'a>,
    {
        return Ok(serde_json::from_slice::<'_, T>(data).convert_into_error(Backtrace::new(line!(), file!()))?);
    }
}
