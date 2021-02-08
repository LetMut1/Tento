use crate::entity::core::date_time::DateTime;
use crate::entity::core::device_id::DeviceId;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::value::Value;
use crate::entity::entity::json_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::util::entity::entity::json_web_token::json_access_web_token::date_expiration_creator::DateExpirationCreator;
use maybe_owned::MaybeOwned;
use crate::dto::entity::entity::json_web_token::json_access_web_token::core::payload::common::Common;

pub struct Payload<'b> {
    user_id: MaybeOwned<'b, UuidV4<'b>>,
    device_id: MaybeOwned<'b, DeviceId<'b>>,
    value: MaybeOwned<'b, Value<'b>>,
    exp: MaybeOwned<'b, DateTime<'b>>
}

impl<'a, 'b: 'a> Payload<'b> {
    pub fn new_from_jrwt(json_refresh_web_token: &'b JsonRefreshWebToken<'a, 'b>) -> Self {
        return Self {
            user_id: MaybeOwned::Borrowed(json_refresh_web_token.get_user_id()),
            device_id: MaybeOwned::Borrowed(json_refresh_web_token.get_device_id()),
            value: MaybeOwned::Borrowed(json_refresh_web_token.get_value()),
            exp: MaybeOwned::Owned(DateExpirationCreator::create_interval())
        };
    }

    pub fn new_from_dto_common(common: Common<'b>) -> Self {
        return Self {
            user_id: MaybeOwned::Owned(UuidV4::new_from_string(&common.get_user_id())),
            device_id: MaybeOwned::Owned(DeviceId::new(MaybeOwned::Borrowed(&common.get_device_id()))),
            value: MaybeOwned::Owned(Value::new(MaybeOwned::Borrowed(&common.get_json_refresh_web_token_value()))),
            exp: MaybeOwned::Owned(DateTime::new_from_string(&common.get_exp()))
        }
    }

    pub fn get_user_id(&'a self) -> &'a UuidV4<'b> {
        return &self.user_id;
    }

    pub fn get_device_id(&'a self) -> &'a DeviceId<'b> {
        return &self.device_id;
    }

    pub fn get_value(&'a self) -> &'a Value<'b> {
        return &self.value;
    }

    pub fn get_exp(&'a self) -> &'a DateTime<'b> {
        return &self.exp;
    }
}