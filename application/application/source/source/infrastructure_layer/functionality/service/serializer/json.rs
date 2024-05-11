use super::Serialize;
use super::Serializer;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::auditor::Converter;
use serde::Deserialize;
use serde::Serialize as SerdeSerialize;
use serde_json;

#[cfg(feature = "manual_testing")]
pub use crate::infrastructure_layer::data::control_type::Json;

#[cfg(feature = "manual_testing")]
impl Serialize for Serializer<Json> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, Auditor<Error>>
    where
        T: SerdeSerialize,
    {
        return Ok(serde_json::to_vec(subject).convert(BacktracePart::new(line!(), file!()))?);
    }

    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, Auditor<Error>>
    where
        T: Deserialize<'a>,
    {
        return Ok(serde_json::from_slice::<'_, T>(data).convert(BacktracePart::new(line!(), file!()))?);
    }
}
