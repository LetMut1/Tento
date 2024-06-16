use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom,
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            control_type::DateTime,
            error::Error,
        },
        functionality::service::resolver::Resolver,
    },
};
impl Generator<ApplicationUserAuthorizationToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, Auditor<Error>> {
        return Ok(Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserAuthorizationToken_CanBeResentFrom::QUANTITY_OF_MINUTES_BEFORE_RESENDING)?);
    }
}
