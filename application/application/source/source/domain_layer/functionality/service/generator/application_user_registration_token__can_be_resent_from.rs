use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom,
    infrastructure_layer::{
        functionality::service::resolver::Resolver,
    },
};
use crate::infrastructure_layer::functionality::service::resolver::date_time::DateTime;
use aggregate_error::AggregateError;
impl Generator<ApplicationUserRegistrationToken_CanBeResentFrom> {
    pub fn generate() -> Result<i64, AggregateError> {
        return Ok(Resolver::<DateTime>::unixtime_add_minutes_interval_from_now(ApplicationUserRegistrationToken_CanBeResentFrom::QUANTITY_OF_MINUTES_BEFORE_RESENDING)?);
    }
}
