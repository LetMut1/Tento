use crate::domain_layer::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder_trait::EncoderTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::encoder::_new_fro_context::common::Common;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct Encoder;

impl Encoder {
    fn get_configured_hmac() -> Result<Hmac<Sha512>, BaseError> {
        return Ok(Hmac::new(Sha512::new(), EnvironmentVariableResolver::get_security_jrwt_encoding_private_key()?.as_bytes()));
    }
}

impl EncoderTrait for Encoder {
    type Error = BaseError;

    fn encode<'outer_a>(
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<String, Self::Error> {
        let mut hmac: Hmac<Sha512> = Self::get_configured_hmac()?;
        
        hmac.input(serde_json::to_string(&Common::new(json_refresh_web_token))?.as_bytes());

        return Ok(hex::encode(hmac.result().code()));   // TODO time attac
    }

    fn is_valid<'outer_a>(
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>,
        json_refresh_web_token_hash: &'outer_a str
    ) -> Result<bool, Self::Error> {
        return Ok(Self::encode(json_refresh_web_token)?.as_str() == json_refresh_web_token_hash);
    }
}