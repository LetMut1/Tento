use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::functionality::service::serialization_form_resolver::SerializationFormResolver;
use crate::infrastructure_layer::data::argument_result::ArgumentResult;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::UnixTime;
use super::extractor::Extractor;

impl Extractor<ApplicationUserAccessToken<'_>> {
    pub async fn extract<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_token_serialized_form: &'a str
    ) -> Result<ArgumentResult<ExtractorResult>, ErrorAuditor> {
        let application_user_access_token = match SerializationFormResolver::<ApplicationUserAccessToken<'_>>::deserialize(
            environment_configuration,
            application_user_access_token_serialized_form
        ) {
            Ok(application_user_access_token_) => application_user_access_token_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        let application_user_access_token_ = match application_user_access_token {
            ArgumentResult::Ok { subject: application_user_access_token__ } => application_user_access_token__,
            ArgumentResult::InvalidArgument { invalid_argument } => {
                return Ok(ArgumentResult::InvalidArgument { invalid_argument });
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_access_token_.get_expires_at().get()) {
            return Ok(ArgumentResult::Ok { subject: ExtractorResult::ApplicationUserAccessTokenAlreadyExpired });
        }

        return Ok(
            ArgumentResult::Ok {
                subject: ExtractorResult::ApplicationUserAccessToken { application_user_access_token: application_user_access_token_ }
            }
        );
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