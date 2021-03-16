use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::payload::_new_fro_context::common_from::CommonFrom;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user_log_in_token::core::device_id::DeviceId;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::value::Value;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::utility::_in_context_for::entity::entity::json_web_token::json_access_web_token::_new_for_context::date_expiration_creator::DateExpirationCreator;
use std::borrow::Cow;

pub struct Payload<'outer> {
    application_user_id: Cow<'outer, UuidV4>,
    device_id: Cow<'outer, DeviceId>,
    value: Cow<'outer, Value>,
    exp: DateTime
}

impl<'this, 'outer: 'this> Payload<'outer> {
    pub fn new_from_json_refresh_web_token(json_refresh_web_token: &'outer JsonRefreshWebToken<'outer>) -> Self {
        return Self {
            application_user_id: Cow::Borrowed(json_refresh_web_token.get_application_user_id()),
            device_id: Cow::Borrowed(json_refresh_web_token.get_device_id()),
            value: Cow::Borrowed(json_refresh_web_token.get_value()),
            exp: DateExpirationCreator::create()
        };
    }

    pub fn new_from_common_from(common_from: CommonFrom) -> Self {
        return Self {
            application_user_id: Cow::Owned(UuidV4::new_from_str(common_from.application_user_id.as_str())),
            device_id: Cow::Owned(DeviceId::new(common_from.device_id)),
            value: Cow::Owned(Value::new(common_from.json_refresh_web_token_value)),
            exp: DateTime::new_from_string(common_from.exp.as_str())
        };
    }

    pub fn get_application_user_id(&'this self) -> &'this UuidV4 {
        return &self.application_user_id;
    }

    pub fn get_device_id(&'this self) -> &'this DeviceId {
        return &self.device_id;
    }

    pub fn get_value(&'this self) -> &'this Value {
        return &self.value;
    }

    pub fn get_exp(&'this self) -> &'this DateTime {
        return &self.exp;
    }
}