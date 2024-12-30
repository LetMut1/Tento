use super::Extractor;
use crate::{
    domain_layer::{
        data::entity::user_access_token::UserAccessToken,
        functionality::service::encoder::Encoder,
    },
    infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            environment_configuration::run_server::PrivateKey,
        },
        functionality::service::resolver::{
            Resolver,
            UnixTime,
        },
    },
};
use dedicated_crate::user_access_token_encoded::UserAccessTokenEncoded;
impl Extractor<UserAccessToken<'_>> {
    pub fn extract<'a>(
        private_key: &'static PrivateKey,
        user_access_token_encoded: &'a UserAccessTokenEncoded,
    ) -> Result<Extracted<'a>, AggregateError> {
        let user_access_token = Encoder::<UserAccessToken<'_>>::decode(
            private_key,
            user_access_token_encoded,
        )?;
        if user_access_token.expires_at <= Resolver::<UnixTime>::get_now() {
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
