use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_core::payload::_new_for_context::factory::Factory as PayloadFactory;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::json_access_web_token::_new_for_context::serialization_form_resolver::_new_for_context::payload_common::PayloadCommon;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub struct Factory;

impl Factory {
    pub fn new_from_json_refresh_web_token<'outer_a>(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>) -> Result<JsonAccessWebToken<'outer_a>, BaseError> {
        return Ok(
            JsonAccessWebToken::new(
                PayloadFactory::new_from_json_refresh_web_token(json_refresh_web_token)?
            )
        );
    }

    pub fn new_from_payload_common(payload_common: PayloadCommon) -> Result<JsonAccessWebToken<'static>, BaseError> {
        return Ok(
            JsonAccessWebToken::new(
                PayloadFactory::new_from_payload_common(payload_common)?
            )
        );
    }
}