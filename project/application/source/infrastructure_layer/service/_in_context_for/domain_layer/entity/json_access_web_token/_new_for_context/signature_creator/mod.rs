use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::signature_creator_trait::SignatureCreatorTrait;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct SignatureCreator;

impl SignatureCreator {
    fn get_configured_hmac(
    ) -> Result<Hmac<Sha512>, ErrorAuditor> {
        match EnvironmentVariableResolver::get_security_jawt_signature_encoding_private_keyXXXDELETE() {
            Ok(security_jawt_signature_encoding_private_key) => {
                return Ok(Hmac::new(Sha512::new(), security_jawt_signature_encoding_private_key.as_bytes()));
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}

impl SignatureCreatorTrait for SignatureCreator {
    type Error = ErrorAuditor;

    fn create<'a>(
        header: &'a str,
        payload: &'a str
    ) -> Result<String, Self::Error> {

        match Self::get_configured_hmac() {
            Ok(mut hmac) => {
                hmac.input((header.to_string() + payload).as_bytes());

                return Ok(hex::encode(hmac.result().code()));   // TODO TIme attack
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }

    fn is_valid<'a>(
        header: &'a str,
        payload: &'a str,
        signature: &'a str
    ) -> Result<bool, Self::Error> {
        match Self::create(header, payload) {
            Ok(signature_) => {
                return Ok(signature_.as_bytes() == signature.as_bytes());
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}