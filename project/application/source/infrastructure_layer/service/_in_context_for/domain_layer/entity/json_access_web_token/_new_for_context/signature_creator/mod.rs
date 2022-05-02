use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::signature_creator_trait::SignatureCreatorTrait;
use crate::infrastructure_layer::service::environment_variable_resolver::EnvironmentVariableResolver;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct SignatureCreator;

impl SignatureCreator {
    fn get_configured_hmac<'a>(
        environment_variable_resolver: &'a EnvironmentVariableResolver,
    ) -> Hmac<Sha512> {
        return Hmac::new(
            Sha512::new(),
            environment_variable_resolver.get_security_jawt_signature_encoding_private_key().as_bytes()
        );
    }
}

impl SignatureCreatorTrait for SignatureCreator {
    fn create<'a>(
        environment_variable_resolver: &'a EnvironmentVariableResolver,
        header: &'a str,
        payload: &'a str
    ) -> String {
        let mut hmac = Self::get_configured_hmac(environment_variable_resolver);
        hmac.input((header.to_string() + payload).as_bytes());

        return hex::encode(hmac.result().code());   // TODO TIme attack
    }

    fn is_valid<'a>(
        environment_variable_resolver: &'a EnvironmentVariableResolver,
        header: &'a str,
        payload: &'a str,
        signature: &'a str
    ) -> bool {
        return Self::create(environment_variable_resolver, header, payload).as_bytes() == signature.as_bytes();
    }
}