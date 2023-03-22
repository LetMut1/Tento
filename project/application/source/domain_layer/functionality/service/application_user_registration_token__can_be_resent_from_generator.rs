use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserRegistrationToken_CanBeResentFromGenerator;

impl ApplicationUserRegistrationToken_CanBeResentFromGenerator {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let application_user_registration_token_can_be_resent_from = match DateTimeResolver::unixtime_add_minutes_interval_from_now(
            ApplicationUserRegistrationToken::QUANTITY_OF_MINUTES_BEFORE_RESENDING
        ) {
            Ok(application_user_registration_token_can_be_resent_from_) => application_user_registration_token_can_be_resent_from_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(application_user_registration_token_can_be_resent_from);
    }
}