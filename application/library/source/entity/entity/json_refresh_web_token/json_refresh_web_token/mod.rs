use crate::data_transfer_object::_in_context_for::_resource::_new_for_context::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::common::Common;
use crate::entity::_core::uuid_v4::UuidV4;
use crate::entity::entity::application_user_log_in_token::_core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::entity::entity::application_user::_core::id::Id as ApplicationUserId;
use crate::entity::entity::json_access_web_token::_core::payload::_core::id::Id as JsonAccessWebTokenId;
use crate::error::base_error::base_error::BaseError;
use std::borrow::Cow;
use super::_core::obfuscation_value::ObfuscationValue;

pub struct JsonRefreshWebToken<'outer_a> {
    json_access_web_token_id: JsonAccessWebTokenId,
    application_user_id: Cow<'outer_a, ApplicationUserId>,
    application_user_log_in_token_device_id: Cow<'outer_a, ApplicationUserLogInTokenDeviceId>,
    obfuscation_value: ObfuscationValue
}

impl<'outer_a> JsonRefreshWebToken<'outer_a> {
    pub fn new(
        application_user_id: &'outer_a ApplicationUserId, application_user_log_in_token_device_id: &'outer_a ApplicationUserLogInTokenDeviceId
    ) -> Self {
        return Self {
            json_access_web_token_id: JsonAccessWebTokenId::new(),
            application_user_id: Cow::Borrowed(application_user_id),
            application_user_log_in_token_device_id: Cow::Borrowed(application_user_log_in_token_device_id),
            obfuscation_value: ObfuscationValue::new(UuidV4::new().get_value().to_string())
        };
    }

    pub fn new_from_model(common: Common<'_>) -> Result<Self, BaseError> {
        return Ok(
            Self {
                json_access_web_token_id: JsonAccessWebTokenId::new_from_string(common.json_access_web_token_id)?,
                application_user_id: Cow::Owned(ApplicationUserId::new_from_string(common.application_user_id)?),
                application_user_log_in_token_device_id: Cow::Owned(ApplicationUserLogInTokenDeviceId::new_from_string(common.application_user_log_in_token_device_id)?),
                obfuscation_value: ObfuscationValue::new(common.obfuscation_value.into_owned())
            }
        );
    }

    pub fn refresh<'this>(&'this mut self) -> &'this mut Self {
        self.obfuscation_value = ObfuscationValue::new(UuidV4::new().get_value().to_string());

        return self;
    }

    pub fn get_json_access_web_token_id<'this>(&'this self) -> &'this JsonAccessWebTokenId {
        return &self.json_access_web_token_id;
    }

    pub fn get_application_user_id<'this>(&'this self) -> &'this ApplicationUserId {
        return self.application_user_id.as_ref();
    }

    pub fn get_application_user_log_in_token_device_id<'this>(&'this self) -> &'this ApplicationUserLogInTokenDeviceId {
        return self.application_user_log_in_token_device_id.as_ref();
    }

    pub fn get_obfuscation_value<'this>(&'this self) -> &'this ObfuscationValue {
        return &self.obfuscation_value;
    }
}