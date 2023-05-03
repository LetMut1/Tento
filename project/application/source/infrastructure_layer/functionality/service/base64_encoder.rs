use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use extern_crate::base64::Config;
use extern_crate::base64::decode_config;
use extern_crate::base64::encode_config;
use extern_crate::base64::STANDARD;
pub use super::encoder::Encoder;

pub struct Base64;

impl Encoder<Base64> {
    const BASE64_STANDARD_CONFIGURATION: Config = STANDARD;   // TODO подходит ли?  // TODO TODO TODO TODO TODO Можно ли здесь использовать Бэйс64 на байтф мессаджПака?

    pub fn encode<'a>(data: &'a [u8]) -> String {
        return encode_config(data, Self::BASE64_STANDARD_CONFIGURATION);
    }

    pub fn decode<'a>(encoded_data: &'a [u8]) -> Result<Vec<u8>, ErrorAuditor> {
        let data = match decode_config(encoded_data, Self::BASE64_STANDARD_CONFIGURATION) {
            Ok(data_) => data_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(data);
    }
}