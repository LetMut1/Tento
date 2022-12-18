use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use extern_crate::crypto::hmac::Hmac;
use extern_crate::crypto::mac::Mac;
use extern_crate::crypto::sha2::Sha512;
use extern_crate::hex;

pub struct SignatureCreator;

impl SignatureCreator {
    pub fn create<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_access_token_serialized: &'a str
    ) -> String {
        let mut hmac = Hmac::new(
            Sha512::new(),
            environment_configuration_resolver.get_security_auat_signature_encoding_private_key().as_bytes()
        );
        hmac.input(application_user_access_token_serialized.as_bytes());

        return hex::encode(hmac.result().code());   // TODO TIme attack
    }

    pub fn is_valid<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_access_token_serialized: &'a str,
        application_user_access_token_signature: &'a str
    ) -> bool {
        return Self::create(environment_configuration_resolver, application_user_access_token_serialized).as_bytes() == application_user_access_token_signature.as_bytes();
    }
}