use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::signature_creator_trait::SignatureCreatorTrait;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct SignatureCreator;

impl SignatureCreator {
    fn get_configured_hmac(
    ) -> Result<Hmac<Sha512>, ErrorAggregator> {
        return Ok(Hmac::new(Sha512::new(), EnvironmentVariableResolver::get_security_jawt_signature_encoding_private_key()?.as_bytes()));
    }
}

impl SignatureCreatorTrait for SignatureCreator {
    type Error = ErrorAggregator;

    fn create<'a>(
        header: &'a str,
        payload: &'a str
    ) -> Result<String, Self::Error> {
        let mut hmac = Self::get_configured_hmac()?;
        hmac.input((header.to_string() + payload).as_bytes());

        return Ok(hex::encode(hmac.result().code()));   // TODO TIme attack
    }

    fn is_valid<'a>(
        header: &'a str,
        payload: &'a str,
        signature: &'a str
    ) -> Result<bool, Self::Error> {
        return Ok(Self::create(header, payload)?.as_str() == signature);
    }
}