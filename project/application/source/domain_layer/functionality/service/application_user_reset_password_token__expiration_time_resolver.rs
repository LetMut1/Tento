use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use extern_crate::chrono::Utc;

#[allow(non_camel_case_types)]
pub struct ApplicationUserResetPasswordToken_ExpirationTimeResolver;

impl ApplicationUserResetPasswordToken_ExpirationTimeResolver {
    pub fn is_expired<'a>(
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken
    ) -> Result<bool, ErrorAuditor> {
        match DateTimeResolver::create_chrono_date_time_utc(application_user_reset_password_token.get_expires_at()) {
            Ok(ref date_time) => {
                return Ok(!DateTimeResolver::is_greater_or_equal_than(date_time, &Utc::now()));
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}