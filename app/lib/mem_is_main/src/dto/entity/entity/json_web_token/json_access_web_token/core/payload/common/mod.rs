use crate::entity::entity::json_web_token::json_access_web_token::JsonAccessWebToken;
use maybe_owned::MaybeOwned;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'b> {
    user_id: MaybeOwned<'b, String>,
    device_id: MaybeOwned<'b, String>,
    json_refresh_web_token_value: MaybeOwned<'b, String>,
    exp: MaybeOwned<'b, String>
}

impl<'a, 'b: 'a> Common<'b> {
    pub fn new_from_entity(json_access_web_token: &'b JsonAccessWebToken<'a, 'b>) -> Self {
        return Self {
            user_id: MaybeOwned::Owned(json_access_web_token.get_payload().get_user_id().get_value().to_string()),
            device_id: MaybeOwned::Borrowed(json_access_web_token.get_payload().get_device_id().get_value()),
            json_refresh_web_token_value: MaybeOwned::Borrowed(json_access_web_token.get_payload().get_value().get_value()),
            exp: MaybeOwned::Owned(json_access_web_token.get_payload().get_exp().get_value().to_rfc3339())
        };
    }

    pub fn get_user_id(&'a self) -> &'a String {
        return &self.user_id;
    }

    pub fn get_device_id(&'a self) -> &'a String {
        return &self.device_id;
    }

    pub fn get_json_refresh_web_token_value(&'a self) -> &'a String {
        return &self.json_refresh_web_token_value;
    }

    pub fn get_exp(&'a self) -> &'a String {
        return &self.exp;
    }
}