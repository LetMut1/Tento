use crate::entity::entity::json_web_token::json_refresh_web_token::json_refresh_web_token::JsonRefreshWebToken;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common {
    #[serde(rename = "ui")]
    pub application_user_id: String,
    #[serde(rename = "di")]
    pub application_user_log_in_token_device_id: String,
    #[serde(rename = "a")]
    pub expired_at: String
}

impl<'outer> Common {
    pub fn new(json_refresh_web_token: &'outer JsonRefreshWebToken<'outer>) -> Self {
        return Self {
            application_user_id: json_refresh_web_token.get_application_user_id().get_value().to_string(),
            application_user_log_in_token_device_id: json_refresh_web_token.get_application_user_log_in_token_device_id().get_value().to_string(),
            expired_at: json_refresh_web_token.get_expired_at().get_value().to_rfc3339()
        };
    }
}