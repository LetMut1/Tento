use super::Generator;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::functionality::service::resolver::date_time::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::data::error::Error;

impl Generator<ApplicationUserResetPasswordToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, Auditor<Error>> {
        return Ok(
            Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(
                ApplicationUserResetPasswordToken_CanBeResentFrom::QUANTITY_OF_MINUTES_BEFORE_RESENDING
            )?
        );
    }
}
