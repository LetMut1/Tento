use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::functionality::service::application_user_access_token__expiration_time_resolver::ApplicationUserAccessToken_ExpirationTimeResolver;
use crate::domain_layer::functionality::service::application_user_access_token__serialization_form_resolver::ApplicationUserAccessToken_SerializationFormResolver;
use crate::domain_layer::functionality::service::application_user_access_token__serialization_form_resolver::SerializationFormResolverResult;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::service::environment_configuration_resolver::EnvironmentConfigurationResolver;

pub struct ApplicationUserAccessToken_Extractor;

impl ApplicationUserAccessToken_Extractor {
    pub async fn extract<'a>(
        environment_configuration_resolver: &'a EnvironmentConfigurationResolver,
        application_user_access_token_deserialized_form: &'a str
    ) -> Result<ExtractorResult, ErrorAuditor> {
        let serialization_form_resolver_result = match ApplicationUserAccessToken_SerializationFormResolver::deserialize(
            environment_configuration_resolver, application_user_access_token_deserialized_form
        ) {
            Ok(serialization_form_resolver_result_) => serialization_form_resolver_result_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };
        let application_user_access_token_ = match serialization_form_resolver_result {
            SerializationFormResolverResult::ApplicationUserAccessToken { application_user_access_token } => application_user_access_token,
            SerializationFormResolverResult::InvalidArgument { invalid_argument } => {
                return Ok(ExtractorResult::InvalidArgument { invalid_argument });
            }
        };

        if ApplicationUserAccessToken_ExpirationTimeResolver::is_expired(&application_user_access_token_) {
            return Ok(ExtractorResult::ApplicationUserAccessTokenAlreadyExpired);
        }

        return Ok(ExtractorResult::ApplicationUserAccessToken { application_user_access_token: application_user_access_token_ });
    }
}

pub enum ExtractorResult {
    ApplicationUserAccessToken {
        application_user_access_token: ApplicationUserAccessToken<'static>
    },
    ApplicationUserAccessTokenAlreadyExpired,
    /// Not yet used due to the fact that there is no such flow yet. More
    /// information in ApplicationUserAccessTokenBlackList entity.
    ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList,
    InvalidArgument {
        invalid_argument: InvalidArgument
    }
}