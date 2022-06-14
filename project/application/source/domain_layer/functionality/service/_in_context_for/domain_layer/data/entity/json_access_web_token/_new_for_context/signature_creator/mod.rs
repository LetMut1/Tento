use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct SignatureCreator;

impl SignatureCreator {
    pub fn create<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        header: &'a str,
        payload: &'a str
    ) -> String {
        let mut hmac = Hmac::new(
            Sha512::new(),
            environment_configuration_resolver.get_security_jawt_signature_encoding_private_key().as_bytes()
        );
        hmac.input((header.to_string() + payload).as_bytes());

        return hex::encode(hmac.result().code());   // TODO TIme attack
    }

    pub fn is_valid<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        header: &'a str,
        payload: &'a str,
        signature: &'a str
    ) -> bool {
        return Self::create(environment_configuration_resolver, header, payload).as_bytes() == signature.as_bytes();
    }
}