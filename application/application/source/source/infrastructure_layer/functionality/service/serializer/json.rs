#[cfg(feature = "manual_testing")]
pub use self::json::Json;
#[cfg(feature = "manual_testing")]
pub mod json {
    use crate::infrastructure_layer::functionality::service::serializer::{
        Serialize,
        Serializer,
    };
    use aggregate_error::{
        AggregateError,
        Backtrace,
        ResultConverter,
    };
    use serde::{
        Deserialize,
        Serialize as SerdeSerialize,
    };
    use serde_json;
    pub struct Json;
    impl Serialize for Serializer<Json> {
        fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, AggregateError>
        where
            T: SerdeSerialize,
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
            T: Deserialize<'a>,
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
