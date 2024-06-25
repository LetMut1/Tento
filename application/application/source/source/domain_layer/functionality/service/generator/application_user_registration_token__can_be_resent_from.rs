use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom,
    infrastructure_layer::{
        data::{
            alternative_workflow::AlternativeWorkflow,
            control_type::DateTime,
        },
        functionality::service::resolver::Resolver,
    },
};
impl Generator<ApplicationUserRegistrationToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, AlternativeWorkflow> {
        return Ok(Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserRegistrationToken_CanBeResentFrom::QUANTITY_OF_MINUTES_BEFORE_RESENDING)?);
    }
}
