use super::Generator;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt;
use crate::infrastructure_layer::data::auditor::BacktracePart;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::functionality::service::resolver::date_time::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::data::error::Error;

impl Generator<ApplicationUserAuthorizationToken_ExpiresAt> {
    pub fn generate() -> Result<ApplicationUserAuthorizationToken_ExpiresAt, Auditor<Error>> {
        let application_user_authorization_token_expires_at = match Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserAuthorizationToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION) {
            Ok(application_user_authorization_token_expires_at_) => application_user_authorization_token_expires_at_,
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

        return Ok(ApplicationUserAuthorizationToken_ExpiresAt(application_user_authorization_token_expires_at));
    }
}
