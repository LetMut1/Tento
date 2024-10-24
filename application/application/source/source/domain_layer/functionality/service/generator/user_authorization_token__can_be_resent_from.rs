use super::Generator;
use crate::{
    domain_layer::data::entity::user_authorization_token::UserAuthorizationToken_CanBeResentFrom,
    infrastructure_layer::functionality::service::resolver::{
        UnixTime,
        Resolver,
    },
};
use crate::infrastructure_layer::data::aggregate_error::AggregateError;
impl Generator<UserAuthorizationToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_minutes_interval_from_now(UserAuthorizationToken_CanBeResentFrom::QUANTITY_OF_MINUTES_BEFORE_RESENDING);
    }
}
