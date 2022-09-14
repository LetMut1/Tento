use chrono::Utc;
use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::data::data_transfer_object::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ExpirationTimeResolver;

impl ExpirationTimeResolver {
    pub fn is_expired<'a>(
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<bool, ErrorAuditor> {
        match DateTimeResolver::create_chrono_date_time_utc(application_user_registration_confirmation_token.get_created_at()) {
            Ok(ref date_time) => {
                let date_time_ = match DateTimeResolver::add_interval_from(date_time, ApplicationUserRegistrationConfirmationToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i64) {
                    Ok(date_time__) => date_time__,
                    Err(mut error) => {
                        error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                        return Err(error);
                    }
                };

                return Ok(!DateTimeResolver::is_greater_or_equal_than(&date_time_, &Utc::now()));
            }
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        }
    }
}