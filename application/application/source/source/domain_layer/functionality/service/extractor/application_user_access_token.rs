use super::Extractor;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::domain_layer::functionality::service::form_resolver::FormResolver;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::ExpirationTimeChecker;
use crate::infrastructure_layer::functionality::service::expiration_time_checker::unix_time::UnixTime;

impl Extractor<ApplicationUserAccessToken<'_>> {
    pub async fn extract<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_token_encrypted: &'a str
    ) -> Result<Result<ExtractorResult, Auditor<InvalidArgument>>, Auditor<Error>> {
        let application_user_access_token = match FormResolver::<ApplicationUserAccessToken<'_>>::from_encrypted(environment_configuration, application_user_access_token_encrypted)? {
            Ok(application_user_access_token_) => application_user_access_token_,
            Err(invalid_argument_auditor) => {
                return Ok(Err(invalid_argument_auditor));
            }
        };

        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_access_token.expires_at.0) {
            return Ok(Ok(ExtractorResult::ApplicationUserAccessTokenAlreadyExpired));
        }

        return Ok(
            Ok(
                ExtractorResult::ApplicationUserAccessToken {
                    application_user_access_token,
                },
            ),
        );
    }
}

pub enum ExtractorResult {
    ApplicationUserAccessToken {
        application_user_access_token: ApplicationUserAccessToken<'static>,
    },
    ApplicationUserAccessTokenAlreadyExpired,
    /// Not yet used due to the fact that there is no such flow yet. More
    /// information in ApplicationUserAccessTokenBlackList entity.
    ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList,
}
