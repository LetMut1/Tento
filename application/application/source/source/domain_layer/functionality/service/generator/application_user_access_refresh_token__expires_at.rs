use super::Generator;
use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::functionality::service::resolver::date_time::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::data::error::Error;

impl Generator<ApplicationUserAccessRefreshToken_ExpiresAt> {
    pub fn generate() -> Result<ApplicationUserAccessRefreshToken_ExpiresAt, Auditor<Error>> {
        let application_user_access_refresh_token_expires_at = match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserAccessRefreshToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION) {
            Ok(application_user_access_refresh_token_expires_at_) => application_user_access_refresh_token_expires_at_,
            Err(mut error) => {
                error.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                return Err(error);
            }
        };

        return Ok(ApplicationUserAccessRefreshToken_ExpiresAt(application_user_access_refresh_token_expires_at));
    }
}
