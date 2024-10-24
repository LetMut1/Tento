use super::Generator;
use crate::{
    domain_layer::data::entity::user_access_refresh_token::UserAccessRefreshToken_ExpiresAt,
    infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::service::resolver::{
            Resolver,
            UnixTime,
        },
    },
};
impl Generator<UserAccessRefreshToken_ExpiresAt> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_minutes_interval_from_now(UserAccessRefreshToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION);
    }
}
