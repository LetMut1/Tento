use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom,
    infrastructure_layer::functionality::service::resolver::{
        date_time::UnixTime,
        Resolver,
    },
};
use aggregate_error::AggregateError;
impl Generator<ApplicationUserAuthorizationToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Result::Ok(Resolver::<UnixTime>::add_minutes_interval_from_now(ApplicationUserAuthorizationToken_CanBeResentFrom::QUANTITY_OF_MINUTES_BEFORE_RESENDING)?);
    }
}
