use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom,
    infrastructure_layer::functionality::service::resolver::{
        date_time::DateTime,
        Resolver,
    },
};
use aggregate_error::AggregateError;
impl Generator<ApplicationUserResetPasswordToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Ok(Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserResetPasswordToken_CanBeResentFrom::QUANTITY_OF_MINUTES_BEFORE_RESENDING)?);
    }
}
