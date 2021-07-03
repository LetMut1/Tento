use crate::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common {
    #[serde(rename = "ti")]
    json_access_web_token_id: String,
    #[serde(rename = "ui")]
    application_user_id: i64,
    #[serde(rename = "di")]
    application_user_log_in_token_device_id: String,
    #[serde(rename = "p")]
    exp: String
}

impl Common {
    pub fn new<'outer_a>(json_access_web_token: &'outer_a JsonAccessWebToken<'_>) -> Self {
        return Self {
            json_access_web_token_id: json_access_web_token.get_id().to_string(),
            application_user_id: json_access_web_token.get_application_user_id().get_value(),
            application_user_log_in_token_device_id: json_access_web_token.get_application_user_log_in_token_device_id().to_string(),
            exp: json_access_web_token.get_exp().to_string()
        };
    }

    pub fn into_inner(self) -> (String, i64, String, String) {
        return (
            self.json_access_web_token_id,
            self.application_user_id,
            self.application_user_log_in_token_device_id,
            self.exp
        );
    }
}