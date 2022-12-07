use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::expiration_time_resolver::ExpirationTimeResolver;
use crate::domain_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::data::data_transfer_object::_in_context_for::infrastructure_layer::functionality::service::_in_context_for::domain_layer::data::entity::application_user_access_token::_new_for_context::extractor::_new_for_context::result::Result as ExtractorResult;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;

pub struct Extractor;

impl Extractor {
    pub async fn extract<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_access_token_web_form: &'a str
    ) -> Result<ExtractorResult, ErrorAuditor> {
        let application_user_access_token = match SerializationFormResolver::deserialize(environment_configuration_resolver, application_user_access_token_web_form) {
            Ok(application_user_access_token_) => application_user_access_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let is_expired = match ExpirationTimeResolver::is_expired(&application_user_access_token) {
            Ok(is_expired_) => is_expired_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        if !is_expired {
            return Ok(ExtractorResult::ApplicationUserAccessToken { application_user_access_token });
        }

        return Ok(ExtractorResult::ApplicationUserAccessTokenAlreadyExpired);
    }
}