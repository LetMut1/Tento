use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt,
    infrastructure_layer::functionality::service::resolver::{
        date_time::DateTime,
        Resolver,
    },
};
use aggregate_error::AggregateError;
impl Generator<ApplicationUserAuthorizationToken_ExpiresAt> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Result::Ok(Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserAuthorizationToken_ExpiresAt::QUANTITY_OF_MINUTES_FOR_EXPIRATION)?);
    }
}
