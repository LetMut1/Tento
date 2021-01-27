use crate::entity::core::date_time::DateTime;
use crate::entity::core::device_id::DeviceId;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::json_web_token::json_refresh_web_token::JsonRefreshWebToken;
use crate::util::entity::entity::json_web_token::json_access_web_token::date_expiration_creator::DateExpirationCreator;

pub struct Payload<'a> {
    user_id: &'a UuidV4<'a>,
    device_id: &'a DeviceId<'a>,
    json_refresh_web_token_id: &'a UuidV4<'a>,
    exp: DateTime<'a>,
}

impl<'a> Payload<'a> {
    pub fn new(json_refresh_web_token: &'a JsonRefreshWebToken) -> Self {
        return Self {
            user_id: json_refresh_web_token.get_user_id(),
            device_id: json_refresh_web_token.get_device_id(),
            json_refresh_web_token_id: json_refresh_web_token.get_id(),
            exp: DateExpirationCreator::create_interval()
        };
    }

    pub fn get_user_id(&'a self) -> &'a UuidV4<'a> {
        return self.user_id;
    }

    pub fn get_device_id(&'a self) -> &'a DeviceId<'a> {
        return self.device_id;
    }

    pub fn get_json_refresh_web_token_id(&'a self) -> &'a UuidV4<'a> {
        return self.json_refresh_web_token_id;
    }

    pub fn get_exp(&'a self) -> &'a DateTime<'a> {
        return &self.exp;
    }
}