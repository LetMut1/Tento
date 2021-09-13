use crate::infrastructure_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_component::payload::_new_for_context::base::Base as JsonAccessWebTokenPayloadFactory;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_access_web_token::_new_for_context::base_trait::BaseTrait as JsonAccessWebTokenFactoryTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub struct Base;

impl JsonAccessWebTokenFactoryTrait for Base {
    type Error = BaseError;
    type JsonAccessWebTokenPayloadFactory = JsonAccessWebTokenPayloadFactory;
}