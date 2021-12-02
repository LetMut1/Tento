use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::base_trait::BaseTrait as JsonRefreshWebTokenFactoryTrait;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::obfuscation_value_generator::ObfuscationValueGenerator;

pub struct Base;

impl JsonRefreshWebTokenFactoryTrait for Base {
    type ObfuscationValueGenerator = ObfuscationValueGenerator;
}