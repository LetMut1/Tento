use crate::domain_layer::entity::entity::application_user_log_in_token::_core::device_id::DeviceId as ApplicationUserLogInTokenDeviceId;
use crate::domain_layer::entity::entity::application_user::_core::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::json_access_web_token::_core::payload::_core::id::Id as JsonAccessWebTokenId;
use std::borrow::Cow;
use super::_core::obfuscation_value::ObfuscationValue;
use uuid::Uuid;

pub struct JsonRefreshWebToken<'outer_a> {
    json_access_web_token_id: JsonAccessWebTokenId,
    application_user_id: Cow<'outer_a, ApplicationUserId>,
    application_user_log_in_token_device_id: Cow<'outer_a, ApplicationUserLogInTokenDeviceId>,
    obfuscation_value: ObfuscationValue
}

impl<'outer_a> JsonRefreshWebToken<'outer_a> {
    pub fn new(
        json_access_web_token_id: JsonAccessWebTokenId,
        application_user_id: Cow<'outer_a, ApplicationUserId>,
        application_user_log_in_token_device_id: Cow<'outer_a, ApplicationUserLogInTokenDeviceId>,
        obfuscation_value: ObfuscationValue
    ) -> Self {
        return Self {
            json_access_web_token_id, application_user_id, application_user_log_in_token_device_id, obfuscation_value
        };
    }

    pub fn refresh<'this>(&'this mut self) -> &'this mut Self {
        self.obfuscation_value = ObfuscationValue::new(Uuid::new_v4().to_string());

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