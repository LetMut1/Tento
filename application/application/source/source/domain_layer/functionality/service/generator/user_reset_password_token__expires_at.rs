use super::Generator;
use crate::{
    domain_layer::data::entity::user_reset_password_token::UserResetPasswordToken_ExpiresAt,
    infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::service::resolver::{
            Resolver,
            UnixTime,
        },
    },
};
impl Generator<UserResetPasswordToken_ExpiresAt> {
    pub fn generate(now: i64) -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_interval(UserResetPasswordToken_ExpiresAt::QUANTITY_OF_SECONDS_FOR_EXPIRATION, now);
    }
}
