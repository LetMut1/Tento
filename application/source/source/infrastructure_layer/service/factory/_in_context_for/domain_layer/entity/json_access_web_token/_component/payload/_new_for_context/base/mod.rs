use crate::domain_layer::entity::json_access_web_token::_component::payload::Payload;
use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_component::payload::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenPayloadFactoryTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::date_time_resolver::DateTimeResolver;
use std::borrow::Cow;

pub struct Base;

impl JsonAccessWebTokenPayloadFactoryTrait for Base {
    type Error = BaseError;

    fn create_from_json_refresh_web_token<'outer_a>(
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<Payload<'outer_a>, Self::Error> {
        return Ok(
            Payload::new(
                Cow::Borrowed(json_refresh_web_token.get_json_access_web_token_id()),
                Cow::Borrowed(json_refresh_web_token.get_application_user_id()),
                Cow::Borrowed(json_refresh_web_token.get_application_user_log_in_token_device_id()),
                DateTimeResolver::add_interval_from_now(&(JsonAccessWebToken::QUANTITY_OF_MINUTES_FOR_EXPIRATION as i64))?
            )
        );
    }
}