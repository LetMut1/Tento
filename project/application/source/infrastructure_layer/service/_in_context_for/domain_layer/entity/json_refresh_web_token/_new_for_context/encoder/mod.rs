use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder_trait::EncoderTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder::_new_fro_context::common::Common;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct Encoder;

impl Encoder {
    fn get_configured_hmac(
    ) -> Result<Hmac<Sha512>, ErrorAuditor> {
        match EnvironmentVariableResolver::get_security_jrwt_encoding_private_keyXXXDELETE() {
            Ok(security_jrwt_encoding_private_key) => {
                return Ok(Hmac::new(Sha512::new(), security_jrwt_encoding_private_key.as_bytes()));
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}

impl EncoderTrait for Encoder {
    type Error = ErrorAuditor;

    fn encode<'a>(
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Result<String, Self::Error> {
        match serde_json::to_vec(&Common::new(json_refresh_web_token)) {
            Ok(data) => {
                match Self::get_configured_hmac() {
                    Ok(mut hmac) => {
                        hmac.input(&data[..]);
        
                        return Ok(hex::encode(hmac.result().code()));   // TODO  TODO TODO time attac
                    }
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));
        
                        return Err(error);
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::OtherError {other_error: OtherError::new(error)}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    fn is_valid<'a>(
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>,
        json_refresh_web_token_hash: &'a str
    ) -> Result<bool, Self::Error> {
        match Self::encode(json_refresh_web_token) {
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