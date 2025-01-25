use super::{
    Deserialize_,
    Serialize,
    Serialize_,
    Serializer,
};
use crate::infrastructure_layer::data::aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use dedicated::bit_code_serializer::Serializer as Serializer_;
pub struct BitCode;
impl Serialize for Serializer<BitCode> {
    fn serialize<'a, T>(subject: &'a T) -> Result<Vec<u8>, AggregateError>
    where
        T: Serialize_,
    {
        return Result::Ok(Serializer_::serialize(subject));
    }
    fn deserialize<'a, T>(data: &'a [u8]) -> Result<T, AggregateError>
    where
        T: Deserialize_<'a>,
    {
        // let a = Result::<_, AggregateError>::Ok(());
        // let b = aaa!(a);



        return Serializer_::deserialize::<'_, T>(data).into_indefinite_argument(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
}

// macro_rules! aaa {
//     ($result:expr) => {
//         match $result {
//             std::result::Result::Ok(value) => value,
//             std::result::Result::Err(error) => {
//                 panic!();
//             }
//         }
//     };
// }
// pub(crate) use aaa;