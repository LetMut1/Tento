use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::payload::_new_fro_context::common::Common;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::device_id::DeviceId;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::value::Value;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::utility::_in_context_for::entity::entity::json_web_token::json_access_web_token::_new_for_context::date_expiration_creator::DateExpirationCreator;
use maybe_owned::MaybeOwned;

pub struct Payload<'outer> {
    application_user_id: UuidV4<'outer>,
    device_id: DeviceId<'outer>,
    value: Value<'outer>,
    exp: DateTime<'outer>
}

impl<'this, 'outer: 'this> Payload<'outer> {
    pub fn new_from_json_refresh_web_token(json_refresh_web_token: &'outer JsonRefreshWebToken<'outer>) -> Self {
        return Self {
            application_user_id: UuidV4::new_from_uuid(json_refresh_web_token.get_application_user_id().get_value()),
            device_id: DeviceId::new(MaybeOwned::Borrowed(json_refresh_web_token.get_device_id().get_value())),
            value: Value::new(MaybeOwned::Borrowed(json_refresh_web_token.get_value().get_value())),
            exp: DateExpirationCreator::create()
        };
    }

    pub fn new_from_dto_common(common: &'outer Common<'outer>) -> Self {
        return Self {
            application_user_id: UuidV4::new_from_string(common.get_application_user_id()),
            device_id: DeviceId::new(MaybeOwned::Borrowed(common.get_device_id())),
            value: Value::new(MaybeOwned::Borrowed(common.get_json_refresh_web_token_value())),
            exp: DateTime::new_from_string(common.get_exp())
        };
    }

    pub fn get_application_user_id(&'this self) -> &'this UuidV4<'outer> {
        return &self.application_user_id;
    }

    pub fn get_device_id(&'this self) -> &'this DeviceId<'outer> {
        return &self.device_id;
    }

    pub fn get_value(&'this self) -> &'this Value<'outer> {
        return &self.value;
    }

    pub fn get_exp(&'this self) -> &'this DateTime<'this> {
        return &self.exp;
    }
}