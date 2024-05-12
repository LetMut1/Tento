use super::Generator;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::functionality::service::resolver::date_time::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use crate::infrastructure_layer::data::error::Error;

impl Generator<ApplicationUserAuthorizationToken_CanBeResentFrom> {
    pub fn generate() -> Result<ApplicationUserAuthorizationToken_CanBeResentFrom, Auditor<Error>> {
        return Ok(
            ApplicationUserAuthorizationToken_CanBeResentFrom(
                Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserAuthorizationToken_CanBeResentFrom::QUANTITY_OF_MINUTES_BEFORE_RESENDING)?
            )
        );
    }
}
