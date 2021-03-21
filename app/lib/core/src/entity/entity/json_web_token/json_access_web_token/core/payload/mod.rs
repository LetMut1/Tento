use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::payload::_new_fro_context::common::Common;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::value::Value;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::utility::_in_context_for::entity::entity::json_web_token::json_access_web_token::_new_for_context::date_expiration_creator::DateExpirationCreator;
use std::borrow::Cow;

pub struct Payload<'outer> {
    application_user_id: Cow<'outer, UuidV4>,
    application_user_log_in_token_device_id: Cow<'outer, UuidV4>,
    json_refresh_web_token_value: Cow<'outer, Value>,
    exp: DateTime
}

impl<'this, 'outer: 'this> Payload<'outer> {
    pub fn new_from_json_refresh_web_token(json_refresh_web_token: &'outer JsonRefreshWebToken<'outer>) -> Self {
        return Self {
            application_user_id: Cow::Borrowed(json_refresh_web_token.get_application_user_id()),
            application_user_log_in_token_device_id: Cow::Borrowed(json_refresh_web_token.get_application_user_log_in_token_device_id()),
            json_refresh_web_token_value: Cow::Borrowed(json_refresh_web_token.get_value()),
            exp: DateExpirationCreator::create()
        };
    }

    pub fn new_from_common(common: Common<'outer>) -> Self {
        return Self {
            application_user_id: Cow::Owned(UuidV4::new_from_str(common.application_user_id.as_str())),
            application_user_log_in_token_device_id: Cow::Owned(UuidV4::new_from_str(common.application_user_log_in_token_device_id.as_str())),
            json_refresh_web_token_value: Cow::Owned(Value::new(common.json_refresh_web_token_value.into_owned())),
            exp: DateTime::new_from_string(common.exp.as_str())
        };
    }

    pub fn get_application_user_id(&'this self) -> &'this UuidV4 {
        return &self.application_user_id;
    }

    pub fn get_application_user_log_in_token_device_id(&'this self) -> &'this UuidV4 {
        return &self.application_user_log_in_token_device_id;
    }

    pub fn get_json_refresh_web_token_value(&'this self) -> &'this Value {
        return &self.json_refresh_web_token_value;
    }

    pub fn get_exp(&'this self) -> &'this DateTime {
        return &self.exp;
    }
}