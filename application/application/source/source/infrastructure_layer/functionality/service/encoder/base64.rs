use super::Encoder;
use crate::infrastructure_layer::data::{
    alternative_workflow::{
        AlternativeWorkflow,
        ResultConverter,
    },
    auditor::Backtrace,
    control_type::Base64,
};
use base64::{
    Config,
    STANDARD,
};
impl Encoder<Base64> {
    const BASE64_STANDARD_CONFIGURATION: Config = STANDARD;
    pub fn encode<'a>(data: &'a [u8]) -> String {
        // TODO подходит ли?  // TODO TODO TODO TODO TODO Можно ли здесь использовать Бэйс64 на байтф мессаджПака?
        return base64::encode_config(
            data,
            Self::BASE64_STANDARD_CONFIGURATION,
        );
    }
    pub fn decode<'a>(encoded_data: &'a [u8]) -> Result<Vec<u8>, AlternativeWorkflow> {
        return base64::decode_config(
            encoded_data,
            Self::BASE64_STANDARD_CONFIGURATION,
        )
        .into_internal_error_runtime(
            Backtrace::new(
                line!(),
                file!(),
            ),
        );
    }
}
