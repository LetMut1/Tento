use super::Generator;
use crate::{
    domain_layer::data::entity::user_authorization_token::UserAuthorizationToken_ExpiresAt,
    infrastructure_layer::functionality::service::resolver::{
        UnixTime,
        Resolver,
    },
};
use aggregate_error::AggregateError;
impl Generator<UserAuthorizationToken_ExpiresAt> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_minutes_interval_from_now(UserAuthorizationToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION);
    }
}
