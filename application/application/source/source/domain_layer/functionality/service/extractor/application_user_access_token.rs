use super::Extractor;
use crate::{
    domain_layer::{
        data::entity::application_user_access_token::ApplicationUserAccessToken,
        functionality::service::encoder::Encoder,
    },
    infrastructure_layer::{
        data::{
            control_type::UnixTime,
            environment_configuration::environment_configuration::EnvironmentConfiguration,
        },
        functionality::service::expiration_time_checker::ExpirationTimeChecker,
    },
};
use application_user_access_token_encoded::ApplicationUserAccessTokenEncoded;
use aggregate_error::AggregateError;
impl Extractor<ApplicationUserAccessToken<'_>> {
    pub fn extract<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        application_user_access_token_encoded: &'a ApplicationUserAccessTokenEncoded,
    ) -> Result<Extracted, AggregateError> {
        let application_user_access_token = Encoder::<ApplicationUserAccessToken<'_>>::decode(
            environment_configuration,
            application_user_access_token_encoded,
        )?;
        if ExpirationTimeChecker::<UnixTime>::is_expired(application_user_access_token.expires_at) {
            return Ok(Extracted::ApplicationUserAccessTokenAlreadyExpired);
        }
        return Ok(
            Extracted::ApplicationUserAccessToken {
                application_user_access_token,
            },
        );
    }
}
pub enum Extracted {
    ApplicationUserAccessToken {
        application_user_access_token: ApplicationUserAccessToken<'static>,
    },
    ApplicationUserAccessTokenAlreadyExpired,
    // Not yet used due to the fact that there is no such flow yet. More
    // information in ApplicationUserAccessTokenBlackList entity.
    ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList,
}
