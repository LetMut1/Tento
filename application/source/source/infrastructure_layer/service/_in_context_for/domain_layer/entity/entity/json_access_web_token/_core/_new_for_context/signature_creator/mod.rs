use crate::domain_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_core::_new_for_context::signature_creator_trait::SignatureCreatorTrait;
use crate::domain_layer::utility::environment_variable_resolver::EnvironmentVariableResolver;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct SignatureCreator;

impl SignatureCreator {
    fn get_configured_hmac() -> Result<Hmac<Sha512>, BaseError> {
        return Ok(Hmac::new(Sha512::new(), EnvironmentVariableResolver::get_security_jawt_signature_encoding_private_key()?.as_bytes()));
    }
}

impl SignatureCreatorTrait for SignatureCreator {
    fn create<'outer_a>(header_and_payload: &'outer_a str) -> Result<String, BaseError> {
        let mut hmac: Hmac<Sha512> = Self::get_configured_hmac()?;
        hmac.input(header_and_payload.as_bytes());

        return Ok(hex::encode(hmac.result().code()));   // TODO TIme attack
    }

    fn is_valid<'outer_a>(header_and_payload: &'outer_a str, signature: &'outer_a str) -> Result<bool, BaseError> {
        return Ok(Self::create(header_and_payload)?.as_str() == signature);
    }
}