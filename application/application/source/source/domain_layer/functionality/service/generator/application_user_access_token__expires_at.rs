use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_ExpiresAt,
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            error::Error,
        },
        functionality::service::resolver::{
            date_time::DateTime,
            Resolver,
        },
    },
};
impl Generator<ApplicationUserAccessToken_ExpiresAt> {
    pub fn generate() -> Result<i64, Auditor<Error>> {
        return Ok(Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserAccessToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION)?);
    }
}
