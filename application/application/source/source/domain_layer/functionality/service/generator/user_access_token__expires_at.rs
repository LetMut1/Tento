use super::Generator;
use crate::{
    domain_layer::data::entity::user_access_token::UserAccessToken_ExpiresAt,
    infrastructure_layer::functionality::service::resolver::{
        UnixTime,
        Resolver,
    },
};
use crate::infrastructure_layer::data::aggregate_error::AggregateError;
impl Generator<UserAccessToken_ExpiresAt> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_minutes_interval_from_now(UserAccessToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION);
    }
}
