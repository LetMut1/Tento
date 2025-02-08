use {
    crate::{
        domain_layer::data::entity::user_access_refresh_token::UserAccessRefreshToken_ExpiresAt,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::service::resolver::{
                Resolver,
                UnixTime,
            },
        },
    },
    super::Generator,
};
impl Generator<UserAccessRefreshToken_ExpiresAt> {
    pub fn generate(now: i64) -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_interval(UserAccessRefreshToken_ExpiresAt::QUANTITY_OF_SECONDS_FOR_EXPIRATION, now);
    }
}
