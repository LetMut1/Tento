use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use bitcode::{
    Encode,
    Decode,
};
pub struct Serializer;
impl Serializer {
    pub fn serialize<'a, T>(subject: &'a T) -> Vec<u8>
    where
        T: Encode,
    {
        return bitcode::encode(subject);
    }
    pub fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, AggregateError>
    where
        T: Decode<'a>,
    {
        return bitcode::decode::<'_, T>(data).into_indefinite_argument(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
}
