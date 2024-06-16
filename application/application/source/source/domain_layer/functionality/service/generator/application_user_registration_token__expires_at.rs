use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_ExpiresAt,
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            control_type::DateTime,
            error::Error,
        },
        functionality::service::resolver::Resolver,
    },
};
impl Generator<ApplicationUserRegistrationToken_ExpiresAt> {
    pub fn generate() -> Result<i64, Auditor<Error>> {
        return Ok(Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserRegistrationToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION)?);
    }
}
