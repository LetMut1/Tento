use crate::domain_layer::entity::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'outer_a> {
    #[serde(rename = "ti")]
    json_access_web_token_id: String,
    #[serde(rename = "ui")]
    application_user_id: i64,
    #[serde(rename = "di")]
    application_user_log_in_token_device_id: String,
    #[serde(rename = "v")]
    obfuscation_value: Cow<'outer_a, str>
}

impl<'outer_a> Common<'outer_a> {
    pub fn new(json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>) -> Self {
        return Self {
            json_access_web_token_id: json_refresh_web_token.get_json_access_web_token_id().get_value().get_value().to_string(),
            application_user_id: json_refresh_web_token.get_application_user_id().get_value(),
            application_user_log_in_token_device_id: json_refresh_web_token.get_application_user_log_in_token_device_id().get_value().get_value().to_string(),
            obfuscation_value: Cow::Borrowed(json_refresh_web_token.get_obfuscation_value().get_value())
        }
    }

    pub fn into_inner(self) -> (String, i64, String, Cow<'outer_a, str>) {
        return (
            self.json_access_web_token_id,
            self.application_user_id,
            self.application_user_log_in_token_device_id,
            self.obfuscation_value
        );
    }
}