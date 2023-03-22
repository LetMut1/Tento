use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;
use extern_crate::uuid::Uuid;

pub struct ApplicationUserAccessRefreshToken_PropertyGenerator;

impl ApplicationUserAccessRefreshToken_PropertyGenerator {
    pub fn generate_obfuscation_value() -> String {
        return Uuid::new_v4().to_string();
    }

    pub fn generate_expires_at() -> Result<i64, ErrorAuditor> {
        let application_user_access_refresh_token_expires_at = match DateTimeResolver::unixtime_add_minutes_interval_from_now(
            ApplicationUserAccessRefreshToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION
        ) {
            Ok(application_user_access_refresh_token_expires_at_) => application_user_access_refresh_token_expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        return Ok(application_user_access_refresh_token_expires_at);
    }

    pub fn generate_updated_at() -> i64 {
        return DateTimeResolver::unixtime_get_now();
    }
}