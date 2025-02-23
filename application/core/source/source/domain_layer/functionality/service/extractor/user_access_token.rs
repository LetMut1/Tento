use {
    super::Extractor,
    crate::{
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
    },
    dedicated::user_access_token_encoded::UserAccessTokenEncoded,
};
impl Extractor<UserAccessToken> {
    // user_access_token__id: &'a str,
    // user__id: i64,
    // user_device__id: &'a str,
    // user_access_token__expires_at: i64,
    pub fn extract<'a>(private_key: &'static PrivateKey, user_access_token_encoded: &'a UserAccessTokenEncoded) -> Result<Extracted<'a>, AggregateError> {
        let (
            user_access_token__id,
            user__id,
            user_device__id,
            user_access_token__expires_at,
        ) = Encoder::<UserAccessToken>::decode(
            private_key,
            user_access_token_encoded,
        )?;
        if user_access_token__expires_at <= Resolver::<UnixTime>::get_now_in_seconds() {
            return Result::Ok(Extracted::AlreadyExpired);
        }
        return Result::Ok(
            Extracted::Data {
                user_access_token__id,
                user__id,
                user_device__id,
                user_access_token__expires_at,
            },
        );
    }
}
pub enum Extracted<'a> {
    Data {
        user_access_token__id: &'a str,
        user__id: i64,
        user_device__id: &'a str,
        user_access_token__expires_at: i64,
    },
    AlreadyExpired,
    // Not yet used due to the fact that there is no such flow yet. More
    // information in UserAccessTokenBlackList entity.
    InUserAccessTokenBlackList,
}
