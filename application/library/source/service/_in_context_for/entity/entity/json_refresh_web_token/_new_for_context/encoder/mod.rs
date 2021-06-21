use crate::data_transfer_object::_in_context_for::entity::entity::json_refresh_web_token::_new_fro_context::common::Common;
use crate::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::error::base_error::base_error::BaseError;
use crate::utility::environment_variable_resolver::EnvironmentVariableResolver;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct Encoder;

impl Encoder {
    pub fn encode<'outer_a>(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>) -> Result<String, BaseError> {
        let mut hmac: Hmac<Sha512> = Self::get_configured_hmac()?;
        
        hmac.input(serde_json::to_string(&Common::new(json_refresh_web_token))?.as_bytes());

        return Ok(hex::encode(hmac.result().code()));   // TODO time attac
    }

    pub fn is_valid<'outer_a>(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>, json_refresh_web_token_hash: &'outer_a str) -> Result<bool, BaseError> {
        return Ok(Self::encode(json_refresh_web_token)?.as_str() == json_refresh_web_token_hash);
    }

    fn get_configured_hmac() -> Result<Hmac<Sha512>, BaseError> {
        return Ok(Hmac::new(Sha512::new(), EnvironmentVariableResolver::get_security_jrwt_encoding_private_key()?.as_bytes()));
    }
}