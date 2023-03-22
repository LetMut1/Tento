use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserResetPasswordToken_ExpirationTimeResolver;

impl ApplicationUserResetPasswordToken_ExpirationTimeResolver {
    pub fn is_expired<'a>(application_user_reset_password_token: &'a ApplicationUserResetPasswordToken<'_>) -> bool {
        return !DateTimeResolver::unixtime_is_greater_or_equal_than_now(application_user_reset_password_token.get_expires_at());
    }

    pub fn create_expires_at() -> Result<i64, ErrorAuditor> {
        let application_user_reset_password_token_expires_at = match DateTimeResolver::unixtime_add_minutes_interval_from_now(
            ApplicationUserResetPasswordToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION
        ) {
            Ok(application_user_reset_password_token_expires_at_) => application_user_reset_password_token_expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(application_user_reset_password_token_expires_at);
    }
}