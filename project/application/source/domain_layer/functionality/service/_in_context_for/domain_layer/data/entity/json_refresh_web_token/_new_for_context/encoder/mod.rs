use crate::domain_layer::data::data_transfer_object::_in_context_for::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::json_refresh_web_token::_new_for_context::encoder::_new_fro_context::common::Common;
use crate::domain_layer::data::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct Encoder;

impl Encoder {
    pub fn encode<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<String, ErrorAuditor> {
        match serde_json::to_vec(&Common::new(json_refresh_web_token)) {
            Ok(data) => {
                let mut hmac = Hmac::new(
                    Sha512::new(),
                    environment_configuration_resolver.get_security_jrwt_encoding_private_key().as_bytes()
                );
                hmac.input(&data[..]);

                return Ok(hex::encode(hmac.result().code()));   // TODO  TODO TODO time attac
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub fn is_valid<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>,
        json_refresh_web_token_hash: &'a str
    ) -> Result<bool, ErrorAuditor> {
        match Self::encode(environment_configuration_resolver, json_refresh_web_token) {
            Ok(json_refresh_web_token_hash_) => {
                return Ok(json_refresh_web_token_hash_.as_bytes() == json_refresh_web_token_hash.as_bytes());
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}