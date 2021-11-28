use crate::domain_layer::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenFactoryTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::serialization_form_resolver::_new_for_context::payload_common::PayloadCommon;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_component::payload::_new_for_context::base::Base as JsonAccessWebTokenPayloadFactory;

pub struct Base;

impl Base {
    pub fn create_from_payload_common(
        payload_common: PayloadCommon<'static>
    ) -> JsonAccessWebToken<'static> {
        return JsonAccessWebToken::new(
            JsonAccessWebTokenPayloadFactory::create_from_payload_common(payload_common)
        );
    }
}

impl JsonAccessWebTokenFactoryTrait for Base {
    type Error = BaseError;
    type JsonAccessWebTokenPayloadFactory = JsonAccessWebTokenPayloadFactory;
}