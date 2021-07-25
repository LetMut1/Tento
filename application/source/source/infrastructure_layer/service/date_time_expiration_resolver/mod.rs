use crate::domain_layer::entity::proxed_type::date_time::DateTime;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::proxed_type::date_time::_new_for_context::date_time_manipulator::DateTimeManipulator;

pub struct DateTimeExpirationResolver;

impl DateTimeExpirationResolver {   // TODO подумать, правильны ли названия. и что делает этот метод. То есть, зачем леоегировать. Посмотреть, где использутся.(можно ли использовать его в тех слоях, где он сейчас используется)
    pub const QUANTITY_OF_MINUTES_APPLICATION_USER_LOG_IN_TOKEN_FIRST: i64 = 10;
    pub const QUANTITY_OF_MINUTES_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_FIRST: i64 = 60 * 24;
    pub const QUANTITY_OF_MINUTES_APPLICATION_USER_RESET_PASSWORD_TOKEN_FIRST: i64 = 10;
    pub const QUANTITY_OF_MINUTES_JSON_ACCESS_WEB_TOKEN_FIRST: i64 = 30;
    pub const QUANTITY_OF_MINUTES_JSON_REFRESH_WEB_TOKEN_FIRST: i64 = 60 * 24 * 21;

    pub fn create_json_access_web_token_first() -> Result<DateTime, BaseError> {
        return Ok(DateTimeManipulator::add_interval_from_now(Self::QUANTITY_OF_MINUTES_JSON_ACCESS_WEB_TOKEN_FIRST)?);
    }
}