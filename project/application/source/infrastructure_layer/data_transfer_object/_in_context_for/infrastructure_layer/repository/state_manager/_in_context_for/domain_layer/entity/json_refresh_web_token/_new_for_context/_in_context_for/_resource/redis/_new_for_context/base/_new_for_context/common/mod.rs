use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'a> {
    json_access_web_token_id: Cow<'a, str>,
    application_user_id: i64,
    application_user_log_in_token_device_id: Cow<'a, str>,
    json_refresh_web_token_obfuscation_value: Cow<'a, str>
}

impl<'a> Common<'a> {
    pub fn new(
        json_refresh_web_token: &'a JsonRefreshWebToken<'_>
    ) -> Self {
        return Self {
            json_access_web_token_id: Cow::Borrowed(json_refresh_web_token.get_json_access_web_token_id()),
            application_user_id: json_refresh_web_token.get_application_user_id(),
            application_user_log_in_token_device_id: Cow::Borrowed(json_refresh_web_token.get_application_user_log_in_token_device_id()),
            json_refresh_web_token_obfuscation_value: Cow::Borrowed(json_refresh_web_token.get_obfuscation_value())
        }
    }

    pub fn into_inner(
        self
    ) -> (Cow<'a, str>, i64, Cow<'a, str>, Cow<'a, str>) {
        return (
            self.json_access_web_token_id,
            self.application_user_id,
            self.application_user_log_in_token_device_id,
            self.json_refresh_web_token_obfuscation_value
        );
    }
}