use crate::domain_layer::entity::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Debug)]
pub struct Common<'outer_a> {
    #[serde(rename = "jawti")]
    json_access_web_token_id: Cow<'outer_a, str>,
    #[serde(rename = "aui")]
    application_user_id: Cow<'outer_a, i64>,
    #[serde(rename = "aulitdi")]
    application_user_log_in_token_device_id: Cow<'outer_a, str>,
    #[serde(rename = "v")]
    obfuscation_value: Cow<'outer_a, str>
}

impl<'outer_a> Common<'outer_a> {
    pub fn new(
        json_refresh_web_token: &'outer_a JsonRefreshWebToken<'_>
    ) -> Self {
        return Self {
            json_access_web_token_id: Cow::Borrowed(json_refresh_web_token.get_json_access_web_token_id()),
            application_user_id: Cow::Borrowed(json_refresh_web_token.get_application_user_id()),
            application_user_log_in_token_device_id: Cow::Borrowed(json_refresh_web_token.get_application_user_log_in_token_device_id()),
            obfuscation_value: Cow::Borrowed(json_refresh_web_token.get_obfuscation_value())
        }
    }
}