use super::{
    Serialize,
    Serializer,
};
#[cfg(feature = "manual_testing")]
use crate::infrastructure_layer::data::control_type::Json;
use crate::infrastructure_layer::data::{
    aggregate_error::{
        AggregateError,
        ResultConverter,
    },
    aggregate_error::Backtrace,
};
use serde::{
    Deserialize,
    Serialize as SerdeSerialize,
};
use serde_json;
#[cfg(feature = "manual_testing")]
impl Serialize for Serializer<Json> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, AggregateError>
    where
        T: SerdeSerialize,
    {
        return serde_json::to_vec(subject).into_invalid_argument_from_client_code(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, AggregateError>
    where
        T: Deserialize<'a>,
    {
        return serde_json::from_slice::<'_, T>(data).into_invalid_argument_from_client_code(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
}
