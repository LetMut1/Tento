pub use super::Encoder;
pub use crate::infrastructure_layer::data::control_type::Base64;
use crate::infrastructure_layer::data::{
    auditor::{
        Auditor,
        Backtrace,
        ErrorConverter,
    },
    error::Error,
};
use base64::{
    Config,
    STANDARD,
};
impl Encoder<Base64> {
    const BASE64_STANDARD_CONFIGURATION: Config = STANDARD;
    // TODO подходит ли?  // TODO TODO TODO TODO TODO Можно ли здесь использовать Бэйс64 на байтф мессаджПака?
    pub fn encode<'a>(data: &'a [u8]) -> String {
        return base64::encode_config(data, Self::BASE64_STANDARD_CONFIGURATION);
    }
    pub fn decode<'a>(encoded_data: &'a [u8]) -> Result<Vec<u8>, Auditor<Error>> {
        let data = base64::decode_config(encoded_data, Self::BASE64_STANDARD_CONFIGURATION).convert(Backtrace::new(line!(), file!()))?;
        return Ok(data);
    }
}
