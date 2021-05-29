use crate::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'outer_a> {
    #[serde(rename = "ti")]
    pub json_access_web_token_id: String,
    #[serde(rename = "ui")]
    pub application_user_id: String,
    #[serde(rename = "di")]
    pub application_user_log_in_token_device_id: String,
    #[serde(rename = "v")]
    pub obfuscation_value: Cow<'outer_a, str>
}

impl<'outer_a> Common<'outer_a> {
    pub fn new(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>) -> Self {
        return Self {
            json_access_web_token_id: json_refresh_web_token.get_json_access_web_token_id().to_string(),
            application_user_id: json_refresh_web_token.get_application_user_id().to_string(),
            application_user_log_in_token_device_id: json_refresh_web_token.get_application_user_log_in_token_device_id().to_string(),
            obfuscation_value: Cow::Borrowed(json_refresh_web_token.get_obfuscation_value().get_value())
        }
    }
}