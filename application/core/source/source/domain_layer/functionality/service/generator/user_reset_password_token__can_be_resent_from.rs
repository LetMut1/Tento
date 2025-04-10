use {
    super::Generator,
    crate::{
        domain_layer::data::entity::user_reset_password_token::UserResetPasswordToken_CanBeResentFrom,
        infrastructure_layer::{
            data::aggregate_error::AggregateError,
            functionality::service::resolver::{
                Resolver,
                UnixTime,
            },
        },
    },
};
impl Generator<UserResetPasswordToken_CanBeResentFrom> {
    pub fn generate(now: i64) -> Result<i64, AggregateError> {
        return Resolver::<UnixTime>::add_interval_microseconds(
            UserResetPasswordToken_CanBeResentFrom::QUANTITY_OF_MICROSECONDS_BEFORE_RESENDING,
            now,
        );
    }
}
