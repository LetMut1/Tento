use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

#[allow(non_camel_case_types)]
pub struct ApplicationUserAccessToken_ExpiresAtGenerator;

impl ApplicationUserAccessToken_ExpiresAtGenerator {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let expires_at = match DateTimeResolver::add_interval_from_now(ApplicationUserAccessToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i64) {
            Ok(expires_at_) => expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(expires_at);
    }
}