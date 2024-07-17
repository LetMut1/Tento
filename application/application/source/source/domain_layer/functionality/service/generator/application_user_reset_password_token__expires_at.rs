use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_ExpiresAt,
    infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            control_type::DateTime,
        },
        functionality::service::resolver::Resolver,
    },
};
impl Generator<ApplicationUserResetPasswordToken_ExpiresAt> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Ok(Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserResetPasswordToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION)?);
    }
}
