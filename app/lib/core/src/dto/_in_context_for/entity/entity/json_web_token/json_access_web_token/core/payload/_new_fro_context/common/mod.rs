use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use maybe_owned::MaybeOwned;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'outer> {
    application_user_id: MaybeOwned<'outer, String>,
    device_id: MaybeOwned<'outer, String>,
    json_refresh_web_token_value: MaybeOwned<'outer, String>,
    exp: MaybeOwned<'outer, String>
}

impl<'this, 'outer: 'this> Common<'outer> {
    pub fn new_from_entity(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> Self {
        return Self {
            application_user_id: MaybeOwned::Owned(json_access_web_token.get_payload().get_application_user_id().get_value().to_string()),
            device_id: MaybeOwned::Borrowed(json_access_web_token.get_payload().get_device_id().get_value()),
            json_refresh_web_token_value: MaybeOwned::Borrowed(json_access_web_token.get_payload().get_value().get_value()),
            exp: MaybeOwned::Owned(json_access_web_token.get_payload().get_exp().get_value().to_rfc3339())
        };
    }

    pub fn get_application_user_id(&'this self) -> &'this String {
        return &self.application_user_id;
    }

    pub fn get_device_id(&'this self) -> &'this String {
        return &self.device_id;
    }

    pub fn get_json_refresh_web_token_value(&'this self) -> &'this String {
        return &self.json_refresh_web_token_value;
    }

    pub fn get_exp(&'this self) -> &'this String {
        return &self.exp;
    }
}