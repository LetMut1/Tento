use crate::entity::core::date_time::DateTime;
use crate::entity::core::device_id::DeviceId;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_web_token::json_refresh_web_token::core::value::Value;
use crate::entity::entity::json_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::util::entity::entity::json_web_token::json_access_web_token::date_expiration_creator::DateExpirationCreator;
use maybe_owned::MaybeOwned;

pub struct Payload<'a> {
    user_id: MaybeOwned<'a, UuidV4<'a>>,
    device_id: MaybeOwned<'a, DeviceId<'a>>,
    value: MaybeOwned<'a, Value<'a>>,
    exp: MaybeOwned<'a, DateTime<'a>>,
}

impl<'a> Payload<'a> {
    pub fn new_from_jrwt(json_refresh_web_token: &'a JsonRefreshWebToken) -> Self {
        return Self {
            user_id: MaybeOwned::Borrowed(json_refresh_web_token.get_user_id()),
            device_id: MaybeOwned::Borrowed(json_refresh_web_token.get_device_id()),
            value: MaybeOwned::Borrowed(json_refresh_web_token.get_value()),
            exp: MaybeOwned::Owned(DateExpirationCreator::create_interval())
        };
    }

    pub fn get_user_id(&'a self) -> &'a UuidV4<'a> {
        return &self.user_id;
    }

    pub fn get_device_id(&'a self) -> &'a DeviceId<'a> {
        return &self.device_id;
    }

    pub fn get_value(&'a self) -> &'a Value<'a> {
        return &self.value;
    }

    pub fn get_exp(&'a self) -> &'a DateTime<'a> {
        return &self.exp;
    }
}