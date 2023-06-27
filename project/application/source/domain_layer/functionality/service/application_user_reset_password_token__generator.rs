use super::generator::Generator;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::generator::Generator as Generator_;
use crate::infrastructure_layer::functionality::service::generator::NumberRow;
use crate::infrastructure_layer::functionality::service::resolver::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;

impl Generator<ApplicationUserResetPasswordToken_Value> {
    pub fn generate() -> ApplicationUserResetPasswordToken_Value {
        return ApplicationUserResetPasswordToken_Value::new(Generator_::<NumberRow>::generate_6());
    }
}

impl Generator<ApplicationUserResetPasswordToken_ExpiresAt> {
    pub fn generate() -> Result<ApplicationUserResetPasswordToken_ExpiresAt, ErrorAuditor> {
        let application_user_reset_password_token_expires_at = match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION) {
            Ok(application_user_reset_password_token_expires_at_) => application_user_reset_password_token_expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        return Ok(ApplicationUserResetPasswordToken_ExpiresAt::new(application_user_reset_password_token_expires_at));
    }
}

impl Generator<ApplicationUserResetPasswordToken_CanBeResentFrom> {
    pub fn generate() -> Result<ApplicationUserResetPasswordToken_CanBeResentFrom, ErrorAuditor> {
        let application_user_reset_password_token_can_be_resent_from = match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_BEFORE_RESENDING) {
            Ok(application_user_reset_password_token_can_be_resent_from_) => application_user_reset_password_token_can_be_resent_from_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                        None,
                    ),
                );

                return Err(error);
            }
        };

        return Ok(ApplicationUserResetPasswordToken_CanBeResentFrom::new(application_user_reset_password_token_can_be_resent_from));
    }
}
