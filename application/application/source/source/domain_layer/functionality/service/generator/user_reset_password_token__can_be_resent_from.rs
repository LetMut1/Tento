use super::Generator;
use crate::{
    domain_layer::data::entity::user_reset_password_token::UserResetPasswordToken_CanBeResentFrom,
    infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::service::resolver::{
            Resolver,
            UnixTime,
        },
    },
};
impl Generator<UserResetPasswordToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_minutes_interval_from_now(UserResetPasswordToken_CanBeResentFrom::QUANTITY_OF_MINUTES_BEFORE_RESENDING);
    }
}
