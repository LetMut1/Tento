use super::Generator;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::functionality::service::resolver::date_time::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::data::error::Error;

impl Generator<ApplicationUserAuthorizationToken_ExpiresAt> {
    pub fn generate() -> Result<ApplicationUserAuthorizationToken_ExpiresAt, Auditor<Error>> {
        return Ok(
            ApplicationUserAuthorizationToken_ExpiresAt(
                Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserAuthorizationToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION)?
            )
        );
    }
}
