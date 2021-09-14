use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::obfuscation_value_generator_trait::ObfuscationValueGeneratorTrait;
use uuid::Uuid;

pub struct ObfuscationValueGenerator;

impl ObfuscationValueGeneratorTrait for ObfuscationValueGenerator {
    fn generate(
    ) -> String {
        return Uuid::new_v4().to_string();
    }
}

