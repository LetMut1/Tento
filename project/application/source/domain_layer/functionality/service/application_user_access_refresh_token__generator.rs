use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::resolver::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use extern_crate::uuid::Uuid;
use super::generator::Generator;

impl Generator<ApplicationUserAccessRefreshToken_ObfuscationValue> {
    pub fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}

impl Generator<ApplicationUserAccessRefreshToken_ExpiresAt> {
    pub fn generate() -> Result<i64, ErrorAuditor> {
        let application_user_access_refresh_token_expires_at = match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(
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
}

impl Generator<ApplicationUserAccessRefreshToken_UpdatedAt> {
    pub fn generate() -> i64 {
        return Resolver::<DateTime>::unixtime_get_now();
    }
}