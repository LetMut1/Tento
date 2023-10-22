use super::Generator;
use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_ExpiresAt;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::resolver::date_time::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;

impl Generator<ApplicationUserAccessToken_ExpiresAt> {
    pub fn generate() -> Result<ApplicationUserAccessToken_ExpiresAt, ErrorAuditor> {
        let application_user_access_token_expires_at = match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserAccessToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION) {
            Ok(application_user_access_token_expires_at_) => application_user_access_token_expires_at_,
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

        return Ok(ApplicationUserAccessToken_ExpiresAt(application_user_access_token_expires_at));
    }
}
