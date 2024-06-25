use super::Extractor;
use crate::{
    domain_layer::{
        data::entity::application_user_access_token::ApplicationUserAccessToken,
        functionality::service::form_resolver::FormResolver,
    },
    infrastructure_layer::{
        data::{
            control_type::UnixTime,
            environment_configuration::EnvironmentConfiguration,
            error::AlternativeWorkflow,
        },
        functionality::service::expiration_time_checker::ExpirationTimeChecker,
    },
};
impl Extractor<ApplicationUserAccessToken<'_>> {
    pub async fn extract<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_access_token_encrypted: &'a str,
    ) -> Result<ExtractorResult, AlternativeWorkflow> {
        let application_user_access_token = FormResolver::<ApplicationUserAccessToken<'_>>::from_encrypted(
            environment_configuration,
            application_user_access_token_encrypted,
        )?;
        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_access_token.expires_at) {
            return Ok(ExtractorResult::ApplicationUserAccessTokenAlreadyExpired);
        }
        return Ok(
            ExtractorResult::ApplicationUserAccessToken {
                application_user_access_token,
            }
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
