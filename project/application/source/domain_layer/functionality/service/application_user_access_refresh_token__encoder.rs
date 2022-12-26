use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::OtherError;
use crate::infrastructure_layer::data::error_auditor::RunTimeError;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use extern_crate::crypto::hmac::Hmac;
use extern_crate::crypto::mac::Mac;
use extern_crate::crypto::sha2::Sha512;
use extern_crate::hex;
use extern_crate::rmp_serde;

#[allow(non_camel_case_types)]
pub struct ApplicationUserAccessRefreshToken_Encoder;

impl ApplicationUserAccessRefreshToken_Encoder {
    pub fn encode<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>
    ) -> Result<String, ErrorAuditor> {
        let mut data: Vec<u8> = vec![];
        if let Err(error) = rmp_serde::encode::write(&mut data, application_user_access_refresh_token) {
            return Err(
                ErrorAuditor::new(
                    BaseError::RunTimeError { run_time_error: RunTimeError::OtherError { other_error: OtherError::new(error) } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        }

        let mut hmac = Hmac::new(
            Sha512::new(),
            environment_configuration_resolver.get_security_auart_encoding_private_key().as_bytes()
        );
        hmac.input(data.as_slice());

        let application_user_access_refresh_token_web_form = hex::encode(hmac.result().code());     // TODO  TODO TODO time attac// TODO TODO TODO TODO TODO Валидно ли кодирует ХЕКС, если это Байты МессаджПака?

        return Ok(application_user_access_refresh_token_web_form);
    }

    pub fn is_valid<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_access_refresh_token: &'a ApplicationUserAccessRefreshToken<'_>,
        application_user_access_refresh_token_web_form: &'a str
    ) -> Result<bool, ErrorAuditor> {
        match Self::encode(environment_configuration_resolver, application_user_access_refresh_token) {
            Ok(application_user_access_refresh_token_web_form_) => {
                return Ok(application_user_access_refresh_token_web_form_.as_bytes() == application_user_access_refresh_token_web_form.as_bytes());
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}