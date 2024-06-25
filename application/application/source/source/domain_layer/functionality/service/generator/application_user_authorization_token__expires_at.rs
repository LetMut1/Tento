use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt,
    infrastructure_layer::{
        data::{
            control_type::DateTime,
            error::AlternativeWorkflow,
        },
        functionality::service::resolver::Resolver,
    },
};
impl Generator<ApplicationUserAuthorizationToken_ExpiresAt> {
    pub fn generate() -> Result<i64, AlternativeWorkflow> {
        return Ok(Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserAuthorizationToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION)?);
    }
}
