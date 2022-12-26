use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::functionality::service::application_user_access_token__expiration_time_resolver::ApplicationUserAccessToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_access_token__serialization_form_resolver::ApplicationUserAccessToken_SerializationFormResolver;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;


#[allow(non_camel_case_types)]
pub struct ApplicationUserAccessToken_Extractor;

impl ApplicationUserAccessToken_Extractor {
    pub async fn extract<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_access_token_web_form: &'a str
    ) -> Result<ExtractorResult, ErrorAuditor> {
        let application_user_access_token = match ApplicationUserAccessToken_SerializationFormResolver::deserialize(environment_configuration_resolver, application_user_access_token_web_form) {
            Ok(application_user_access_token_) => application_user_access_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let is_expired = match ApplicationUserAccessToken_ExpirationTimeResolver::is_expired(&application_user_access_token) {
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

pub enum ExtractorResult {
    ApplicationUserAccessToken {
        application_user_access_token: ApplicationUserAccessToken<'static>
    },
    ApplicationUserAccessTokenAlreadyExpired,
    /// Not yet used due to the fact that there is no such flow yet. More
    /// information in ApplicationUserAccessTokenBlackList entity.
    ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList
}