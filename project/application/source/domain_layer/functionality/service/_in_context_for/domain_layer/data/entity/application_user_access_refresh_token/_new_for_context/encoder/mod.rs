use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::_component::other_error::OtherError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::data::error_auditor::_component::base_error::base_error::BaseError;
use crate::infrastructure_layer::data::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha512;

pub struct Encoder;

impl Encoder {
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