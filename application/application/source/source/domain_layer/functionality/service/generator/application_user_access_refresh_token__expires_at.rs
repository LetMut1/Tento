use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt,
    infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            control_type::DateTime,
        },
        functionality::service::resolver::Resolver,
    },
};
impl Generator<ApplicationUserAccessRefreshToken_ExpiresAt> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Ok(Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserAccessRefreshToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION)?);
    }
}
