use super::Encoder;
use crate::infrastructure_layer::data::control_type::Base64;
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
impl Encoder<Base64> {
    // const BASE64_STANDARD_CONFIGURATION: Config = STANDARD;
    pub fn encode<'a>(data: &'a [u8]) -> String {
        // // TODO подходит ли?  // TODO TODO TODO TODO TODO Можно ли здесь использовать Бэйс64 на байтф мессаджПака?
        // return base64::encode_config(
        //     data,
        //     Self::BASE64_STANDARD_CONFIGURATION,
        // );
        todo!();
    }
    pub fn decode<'a>(encoded_data: &'a [u8]) -> Result<Vec<u8>, AggregateError> {
        // return base64::decode_config(
        //     encoded_data,
        //     Self::BASE64_STANDARD_CONFIGURATION,
        // )
        // .into_indefinite_argument(
        //     Backtrace::new(
        //         line!(),
        //         file!(),
        //     ),
        // );
        todo!();
    }
}
