use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use extern_crate::base64;
use extern_crate::crypto::hmac::Hmac;
use extern_crate::crypto::mac::Mac;
use extern_crate::crypto::sha2::Sha512;
use extern_crate::hex;
use extern_crate::rmp_serde;

#[allow(non_camel_case_types)]
pub struct ApplicationUserAccessToken_SerializationFormResolver;

impl ApplicationUserAccessToken_SerializationFormResolver {
    const TOKEN_PARTS_SEPARATOR: &'static str = ".";

    pub fn serialize<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_access_token: &'a ApplicationUserAccessToken<'_>
    ) -> Result<String, ErrorAuditor> {
        let mut data: Vec<u8> = vec![];
        if let Err(error) = rmp_serde::encode::write(&mut data, application_user_access_token) {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }
        let application_user_access_token_serialized = base64::encode_config(data.as_slice(), base64::STANDARD);  // TODO TODO TODO TODO TODO Можно ли здесь использовать Бэйс64 на байтф мессаджПака?

        let application_user_access_token_signature = ApplicationUserAccessToken_SignatureCreator::create(environment_configuration_resolver, application_user_access_token_serialized.as_str());

        let application_user_access_token_web_form = application_user_access_token_serialized + Self::TOKEN_PARTS_SEPARATOR + application_user_access_token_signature.as_str();

        return Ok(application_user_access_token_web_form);
    }

    pub fn deserialize<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_access_token_classic_form: &'a str
    ) -> Result<ApplicationUserAccessToken<'static>, ErrorAuditor> {
        let token_part_registry = application_user_access_token_classic_form
            .split::<'_, &'_ str>(Self::TOKEN_PARTS_SEPARATOR)
            .collect::<Vec<&'_ str>>();

        if token_part_registry.len() == 2
            && ApplicationUserAccessToken_SignatureCreator::is_valid(environment_configuration_resolver, token_part_registry[0], token_part_registry[1]) {
            match base64::decode_config(token_part_registry[0].as_bytes(), base64::STANDARD) {
                Ok(data) => {
                    match rmp_serde::from_read_ref::<'_, [u8], ApplicationUserAccessToken<'static>>(data.as_slice()) {
                        Ok(application_user_access_token) => {
                            return Ok(application_user_access_token);
                        }
                        Err(error) => {
                            return Err(
                                ErrorAuditor::new(
                                    BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                                    BacktracePart::new(line!(), file!(), None)
                                )
                            );
                        }
                    }
                }
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                            BacktracePart::new(line!(), file!(), None)
                        )
                    );
                }
            }
        }

        return Err(
            ErrorAuditor::new(
                BaseError::InvalidArgumentError,
                BacktracePart::new(line!(), file!(), None)
            )
        );
    }
}

#[allow(non_camel_case_types)]
struct ApplicationUserAccessToken_SignatureCreator;

impl ApplicationUserAccessToken_SignatureCreator {
    fn create<'a>(
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

    fn is_valid<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_access_token_serialized: &'a str,
        application_user_access_token_signature: &'a str
    ) -> bool {
        return Self::create(environment_configuration_resolver, application_user_access_token_serialized).as_bytes() == application_user_access_token_signature.as_bytes();
    }
}