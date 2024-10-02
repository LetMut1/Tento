use super::Generator;
use crate::{
    domain_layer::data::entity::user_reset_password_token::UserResetPasswordToken_ExpiresAt,
    infrastructure_layer::functionality::service::resolver::{
        date_time::UnixTime,
        Resolver,
    },
};
use aggregate_error::AggregateError;
impl Generator<UserResetPasswordToken_ExpiresAt> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Result::Ok(Resolver::<UnixTime>::add_minutes_interval_from_now(UserResetPasswordToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION)?);
    }
}
