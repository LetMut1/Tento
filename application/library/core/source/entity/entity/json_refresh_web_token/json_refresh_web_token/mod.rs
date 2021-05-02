use crate::data_transfer_object::resource_model::_in_context_for::entity::entity::json_refresh_web_token::_new_for_context::common::Common;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::error::main_error_kind::core::invalid_argument_error::InvalidArgumentError;
use std::borrow::Cow;
use super::core::obfuscation_value::ObfuscationValue;

pub struct JsonRefreshWebToken<'outer> {
    json_access_web_token_id: UuidV4,
    application_user_id: Cow<'outer, UuidV4>,
    application_user_log_in_token_device_id: Cow<'outer, UuidV4>,
    obfuscation_value: ObfuscationValue
}

impl<'this, 'outer: 'this> JsonRefreshWebToken<'outer> {
    pub fn new(application_user_id: &'outer UuidV4, application_user_log_in_token_device_id: &'outer UuidV4) -> Self {
        return Self {
            json_access_web_token_id: UuidV4::new(),
            application_user_id: Cow::Borrowed(application_user_id),
            application_user_log_in_token_device_id: Cow::Borrowed(application_user_log_in_token_device_id),
            obfuscation_value: ObfuscationValue::new(UuidV4::new().get_value().to_string())
        };
    }

    pub fn new_from_model(common: Common<'outer>) -> Result<Self, InvalidArgumentError> {
        return Ok(
            Self {
                json_access_web_token_id: UuidV4::new_from_string(common.json_access_web_token_id)?,
                application_user_id: Cow::Owned(UuidV4::new_from_string(common.application_user_id)?),
                application_user_log_in_token_device_id: Cow::Owned(UuidV4::new_from_string(common.application_user_log_in_token_device_id)?),
                obfuscation_value: ObfuscationValue::new(common.obfuscation_value.into_owned())
            }
        );
    }

    pub fn refresh(&'this mut self) -> &'this mut Self {
        self.obfuscation_value = ObfuscationValue::new(UuidV4::new().get_value().to_string());

        return self;
    }

    pub fn get_json_access_web_token_id(&'this self) -> &'this UuidV4 {
        return &self.json_access_web_token_id;
    }

    pub fn get_application_user_id(&'this self) -> &'this UuidV4 {
        return &self.application_user_id;
    }

    pub fn get_application_user_log_in_token_device_id(&'this self) -> &'this UuidV4 {
        return self.application_user_log_in_token_device_id.as_ref();
    }

    pub fn get_obfuscation_value(&'this self) -> &'this ObfuscationValue {
        return &self.obfuscation_value;
    }
}