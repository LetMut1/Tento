
use crate::domain_layer::entity::entity::application_user_log_in_token::_component::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_access_web_token::_component::payload::_component::id::Id as JsonAccessWebTokenId;
use crate::domain_layer::entity::entity::json_refresh_web_token::_component::obfuscation_value::ObfuscationValue;
use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::infrastructure_layer::data_transfer_object::_in_context_for::infrastructure_layer::repository::_in_context_for::domain_layer::entity::entity::json_refresh_web_token::_new_for_context::_in_context_for::_resource::redis::_new_for_context::base_repository::_new_for_context::common::Common;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use std::borrow::Cow;
use uuid::Uuid;

pub struct Base;

impl Base {
    pub fn new_from_id_registry<'outer_a>(
        application_user_id: &'outer_a ApplicationUserId, application_user_log_in_token_device_id: &'outer_a ApplicationUserLogInTokenDeviceId
    ) -> JsonRefreshWebToken<'outer_a> {
        return JsonRefreshWebToken::new(
            JsonAccessWebTokenId::new(),
            Cow::Borrowed(application_user_id),
            Cow::Borrowed(application_user_log_in_token_device_id),
            ObfuscationValue::new(Uuid::new_v4().to_string())
        );
    }

    pub fn new_from_common(common: Common<'_>) -> Result<JsonRefreshWebToken<'_>, BaseError> {
        let (
            json_access_web_token_id,
            application_user_id,
            application_user_log_in_token_device_id,
            obfuscation_value
        ) = common.into_inner();

        return Ok(
            JsonRefreshWebToken::new(
                JsonAccessWebTokenId::new_from_string(json_access_web_token_id)?,
                Cow::Owned(ApplicationUserId::new(application_user_id)),
                Cow::Owned(ApplicationUserLogInTokenDeviceId::new_from_string(application_user_log_in_token_device_id)?),
                ObfuscationValue::new(obfuscation_value.into_owned())
            )
        );
    }
}