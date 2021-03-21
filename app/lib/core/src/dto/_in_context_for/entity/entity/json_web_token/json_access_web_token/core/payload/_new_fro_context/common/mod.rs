use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'outer> {
    #[serde(rename = "ui")]
    pub application_user_id: String,
    #[serde(rename = "di")]
    pub application_user_log_in_token_device_id: String,
    #[serde(rename = "v")]
    pub json_refresh_web_token_value: Cow<'outer, str>,
    #[serde(rename = "v")]
    pub exp: String
}

impl<'outer> Common<'outer> {
    pub fn new(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> Self {
        return Self {
            application_user_id: json_access_web_token.get_payload().get_application_user_id().get_value().to_string(),
            application_user_log_in_token_device_id: json_access_web_token.get_payload().get_application_user_log_in_token_device_id().get_value().to_string(),
            json_refresh_web_token_value: Cow::Borrowed(json_access_web_token.get_payload().get_json_refresh_web_token_value().get_value()),
            exp: json_access_web_token.get_payload().get_exp().get_value().to_rfc3339()
        };
    }
}