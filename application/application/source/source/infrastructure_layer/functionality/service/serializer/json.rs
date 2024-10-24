#[cfg(feature = "json_for_manual_test")]
pub use self::json::Json;
#[cfg(feature = "json_for_manual_test")]
mod json {
    use crate::infrastructure_layer::functionality::service::serializer::{
        Serialize,
        Serializer,
        Deserialize_,
        Serialize_,
    };
    use crate::infrastructure_layer::data::aggregate_error::{
        AggregateError,
        Backtrace,
        ResultConverter,
    };
    pub struct Json;
    impl Serialize for Serializer<Json> {
        fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, AggregateError>
        where
            T: Serialize_,
        {
            return serde_json::to_vec(subject).into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            );
        }
        fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, AggregateError>
        where
            T: Deserialize_<'a>,
        {
            return serde_json::from_slice::<'_, T>(data).into_indefinite_argument(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            );
        }
    }
}
