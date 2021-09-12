
use crate::domain_layer::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::state_manager::_in_context_for::domain_layer::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base::_new_for_context::common::Common;
use std::borrow::Cow;
use uuid::Uuid;

pub struct Base;

impl Base {
    pub fn new_from_id_registry<'outer_a>(
        application_user_id: &'outer_a i64,
        application_user_log_in_token_device_id: &'outer_a str
    ) -> JsonRefreshWebToken<'outer_a> {
        return JsonRefreshWebToken::new(
            Uuid::new_v4().to_string(),
            Cow::Borrowed(application_user_id),
            Cow::Borrowed(application_user_log_in_token_device_id),
            Uuid::new_v4().to_string()
        );
    }

    pub fn new_from_common(
        common: Common<'_>
    ) -> JsonRefreshWebToken<'_> {
        let (
            json_access_web_token_id,
            application_user_id,
            application_user_log_in_token_device_id,
            obfuscation_value
        ) = common.into_inner();

        return JsonRefreshWebToken::new(
            json_access_web_token_id.into_owned(),
            application_user_id,
            application_user_log_in_token_device_id,
            obfuscation_value.into_owned()
        );
    }
}