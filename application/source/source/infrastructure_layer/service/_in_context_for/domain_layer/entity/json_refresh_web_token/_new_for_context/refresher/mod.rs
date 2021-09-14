use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::refresher_trait::RefresherTrait;
use super::obfuscation_value_generator::ObfuscationValueGenerator;

pub struct Refresher;

impl RefresherTrait for Refresher {
    type ObfuscationValueGenerator = ObfuscationValueGenerator;
}