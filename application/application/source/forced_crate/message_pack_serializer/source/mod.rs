use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use serde::{
    Deserialize,
    Serialize as SerdeSerialize,
};
pub struct Serializer;
impl Serializer {
    pub fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, AggregateError>
    where
        T: SerdeSerialize,
    {
        return rmp_serde::to_vec(subject).into_logic(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
    pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, AggregateError>
    where
        T: Deserialize<'a>,
    {
        return rmp_serde::from_read_ref::<'_, [u8], T>(data).into_indefinite_argument(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
}
