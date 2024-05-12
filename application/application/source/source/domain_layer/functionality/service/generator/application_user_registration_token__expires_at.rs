use super::Generator;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_ExpiresAt;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::functionality::service::resolver::date_time::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::data::error::Error;

impl Generator<ApplicationUserRegistrationToken_ExpiresAt> {
    pub fn generate() -> Result<ApplicationUserRegistrationToken_ExpiresAt, Auditor<Error>> {
        return Ok(
            ApplicationUserRegistrationToken_ExpiresAt(
                Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserRegistrationToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION)?
            )
        );
    }
}
