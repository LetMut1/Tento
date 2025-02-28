#[cfg(feature = "json_for_manual_test")]
pub use self::json::Json;
#[cfg(feature = "json_for_manual_test")]
mod json {
    use crate::infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::service::serializer::{
            Deserialize_,
            Serialize,
            Serialize_,
            Serializer,
        },
    };
    pub struct Json;
    impl Serialize for Serializer<Json> {
        fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, AggregateError>
        where
            T: Serialize_,
        {
            return crate::result_into_logic!(serde_json::to_vec(subject));
        }
        fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, AggregateError>
        where
            T: Deserialize_<'a>,
        {
            return crate::result_into_indefinite_argument!(serde_json::from_slice::<'_, T>(data));
        }
    }
}
