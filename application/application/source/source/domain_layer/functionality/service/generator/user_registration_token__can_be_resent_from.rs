use super::Generator;
use crate::{
    domain_layer::data::entity::user_registration_token::UserRegistrationToken_CanBeResentFrom,
    infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::service::resolver::{
            Resolver,
            UnixTime,
        },
    },
};
impl Generator<UserRegistrationToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_minutes_interval_from_now(UserRegistrationToken_CanBeResentFrom::QUANTITY_OF_MINUTES_BEFORE_RESENDING);
    }
}
