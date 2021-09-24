use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::base_trait::BaseTrait as JsonRefreshWebTokenFactoryTrait;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::obfuscation_value_generator::ObfuscationValueGenerator;
use std::borrow::Cow;

pub struct Base;

impl Base {
    pub fn create_from_common(
        common: Common<'_>
    ) -> JsonRefreshWebToken<'_> {
        let (
            json_access_web_token_id,
            application_user_id,
            application_user_log_in_token_device_id,
            obfuscation_value
        ) : (
            Cow<'_, str>,
            Cow<'_, i64>,
            Cow<'_, str>,
            Cow<'_, str>
        ) = common.into_inner();

        return JsonRefreshWebToken::new(
            json_access_web_token_id.into_owned(),
            application_user_id,
            application_user_log_in_token_device_id,
            obfuscation_value.into_owned()
        );
    }
}

impl JsonRefreshWebTokenFactoryTrait for Base {
    type ObfuscationValueGenerator = ObfuscationValueGenerator;
}