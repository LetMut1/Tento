
use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::obfuscation_value_generator_trait::ObfuscationValueGeneratorTrait;
use std::borrow::Cow;
use uuid::Uuid;

pub trait BaseTrait {
    type ObfuscationValueGenerator: ObfuscationValueGeneratorTrait;

    fn create_from_id_registry<'a>(
        application_user_id: &'a i64,
        application_user_log_in_token_device_id: &'a str
    ) -> JsonRefreshWebToken<'a> {
        return JsonRefreshWebToken::new(
            Uuid::new_v4().to_string(),
            Cow::Borrowed(application_user_id),
            Cow::Borrowed(application_user_log_in_token_device_id),
            <Self::ObfuscationValueGenerator as ObfuscationValueGeneratorTrait>::generate()
        );
    }
}