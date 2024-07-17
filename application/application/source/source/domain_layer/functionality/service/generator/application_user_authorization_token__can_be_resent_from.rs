use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom,
    infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            control_type::DateTime,
        },
        functionality::service::resolver::Resolver,
    },
};
impl Generator<ApplicationUserAuthorizationToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Ok(Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserAuthorizationToken_CanBeResentFrom::QUANTITY_OF_MINUTES_BEFORE_RESENDING)?);
    }
}
