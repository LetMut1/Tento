use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'outer_a> {
    #[serde(rename = "jawt")]
    json_access_web_token_id: Cow<'outer_a, str>,
    #[serde(rename = "aui")]
    application_user_id: Cow<'outer_a, i64>,
    #[serde(rename = "aulitdi")]
    application_user_log_in_token_device_id: Cow<'outer_a, str>,
    #[serde(rename = "ov")]
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

    pub fn into_inner(
        self
    ) -> (Cow<'outer_a, str>, Cow<'outer_a, i64>, Cow<'outer_a, str>, Cow<'outer_a, str>) {
        return (
            self.json_access_web_token_id,
            self.application_user_id,
            self.application_user_log_in_token_device_id,
            self.obfuscation_value
        );
    }
}