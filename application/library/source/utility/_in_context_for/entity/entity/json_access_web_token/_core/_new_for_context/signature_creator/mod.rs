use crate::error::main_error::main_error::MainError;
use crate::utility::environment_variable_resolver::EnvironmentVariableResolver;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct SignatureCreator;

impl SignatureCreator {
    pub fn create<'outer_a>(header_and_payload: &'outer_a str) -> Result<String, MainError> {
        let mut hmac: Hmac<Sha512> = Self::get_configured_hmac()?;
        hmac.input(header_and_payload.as_bytes());

        return Ok(hex::encode(hmac.result().code()));
    }

    pub fn is_valid<'outer_a>(header_and_payload: &'outer_a str, signature: &'outer_a str) -> Result<bool, MainError> {
        return Ok(Self::create(header_and_payload)?.as_str() == signature);
    }

    fn get_configured_hmac() -> Result<Hmac<Sha512>, MainError> {
        return Ok(Hmac::new(Sha512::new(), EnvironmentVariableResolver::get_security_jawt_signature_encoding_private_key()?.as_bytes()));
    }
}