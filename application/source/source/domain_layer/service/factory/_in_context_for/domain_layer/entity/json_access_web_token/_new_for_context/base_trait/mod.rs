use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_component::payload::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenPayloadFactoryTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::_new_for_context::payload_common::PayloadCommon;
use std::error::Error;

pub trait BaseTrait {
    type Error: Error;
    type JsonAccessWebTokenPayloadFactory: JsonAccessWebTokenPayloadFactoryTrait<Error = Self::Error>;

    fn new_from_json_refresh_web_token<'outer_a>(
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Result<JsonAccessWebToken<'outer_a>, Self::Error> {
        return Ok(
            JsonAccessWebToken::new(
                Self::JsonAccessWebTokenPayloadFactory::new_from_json_refresh_web_token(json_refresh_web_token)?
            )
        );
    }

    fn new_from_payload_common(
        payload_common: PayloadCommon<'static>
    ) -> JsonAccessWebToken<'static> {
        return JsonAccessWebToken::new(
            Self::JsonAccessWebTokenPayloadFactory::new_from_payload_common(payload_common)
        );
    }
}