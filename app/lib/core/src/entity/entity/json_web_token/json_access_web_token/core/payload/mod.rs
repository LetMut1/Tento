use crate::dto::_in_context_for::entity::entity::json_web_token::json_access_web_token::core::payload::_new_fro_context::common::Common;
use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::device_id::DeviceId;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::value::Value;
use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::utility::_in_context_for::entity::entity::json_web_token::json_access_web_token::_new_for_context::date_expiration_creator::DateExpirationCreator;
use maybe_owned::MaybeOwned;

pub struct Payload<'this, 'outer: 'this> {
    user_id: MaybeOwned<'outer, UuidV4<'outer>>,
    device_id: MaybeOwned<'outer, DeviceId<'outer>>,
    value: MaybeOwned<'outer, Value<'outer>>,
    exp: DateTime<'this>
}

impl<'this, 'outer: 'this> Payload<'this, 'outer> {
    pub fn new_from_json_refresh_web_token(json_refresh_web_token: &'outer JsonRefreshWebToken<'outer, 'outer>) -> Self {
        return Self {
            user_id: MaybeOwned::Borrowed(json_refresh_web_token.get_user_id()),
            device_id: MaybeOwned::Borrowed(json_refresh_web_token.get_device_id()),
            value: MaybeOwned::Borrowed(json_refresh_web_token.get_value()),
            exp: DateExpirationCreator::create_interval()
        };
    }

    pub fn new_from_dto_common(common: &'outer Common<'outer>) -> Self {
        return Self {
            user_id: MaybeOwned::Owned(UuidV4::new_from_string(common.get_user_id())),
            device_id: MaybeOwned::Owned(DeviceId::new(MaybeOwned::Borrowed(common.get_device_id()))),
            value: MaybeOwned::Owned(Value::new(MaybeOwned::Borrowed(common.get_json_refresh_web_token_value()))),
            exp: DateTime::new_from_string(common.get_exp())
        };
    }

    pub fn get_user_id(&'this self) -> &'this UuidV4<'outer> {
        return &self.user_id;
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