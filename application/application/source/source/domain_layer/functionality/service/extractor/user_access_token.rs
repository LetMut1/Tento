use super::Extractor;
use crate::{
    domain_layer::{
        data::entity::user_access_token::UserAccessToken,
        functionality::service::encoder::Encoder,
    },
    infrastructure_layer::{
        data::environment_configuration::EnvironmentConfiguration,
        functionality::service::resolver::{
            expiration::Expiration,
            Resolver,
        },
    },
};
use aggregate_error::AggregateError;
use user_access_token_encoded::UserAccessTokenEncoded;
impl Extractor<UserAccessToken<'_>> {
    pub fn extract<'a>(
        environment_configuration: &'static EnvironmentConfiguration,
        user_access_token_encoded: &'a UserAccessTokenEncoded,
    ) -> Result<Extracted<'a>, AggregateError> {
        let user_access_token = Encoder::<UserAccessToken<'_>>::decode(
            environment_configuration,
            user_access_token_encoded,
        )?;
        if Resolver::<Expiration>::is_expired(user_access_token.expires_at) {
            return Result::Ok(Extracted::UserAccessTokenAlreadyExpired);
        }
        return Result::Ok(
            Extracted::UserAccessToken {
                user_access_token,
            },
        );
    }
}
pub enum Extracted<'a> {
    UserAccessToken {
        user_access_token: UserAccessToken<'a>,
    },
    UserAccessTokenAlreadyExpired,
    // Not yet used due to the fact that there is no such flow yet. More
    // information in UserAccessTokenBlackList entity.
    UserAccessTokenInUserAccessTokenBlackList,
}
