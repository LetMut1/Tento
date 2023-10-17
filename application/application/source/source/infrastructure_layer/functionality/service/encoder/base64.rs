pub use super::Encoder;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor_;
use crate::infrastructure_layer::data::error_auditor::Other;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use base64::decode_config;
use base64::encode_config;
use base64::Config;
use base64::STANDARD;

pub use crate::infrastructure_layer::data::control_type::Base64;

impl Encoder<Base64> {
    const BASE64_STANDARD_CONFIGURATION: Config = STANDARD; // TODO подходит ли?  // TODO TODO TODO TODO TODO Можно ли здесь использовать Бэйс64 на байтф мессаджПака?

    pub fn encode<'a>(data: &'a [u8]) -> String {
        return encode_config(
            data,
            Self::BASE64_STANDARD_CONFIGURATION,
        );
    }

    pub fn decode<'a>(encoded_data: &'a [u8]) -> Result<Vec<u8>, ErrorAuditor_> {
        let data = match decode_config(
            encoded_data,
            Self::BASE64_STANDARD_CONFIGURATION,
        ) {
            Ok(data_) => data_,
            Err(error) => {
                return Err(
                    ErrorAuditor_::new(
                        Error::Runtime {
                            runtime: Runtime::Other {
                                other: Other::new(error),
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        return Ok(data);
    }
}
