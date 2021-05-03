use crate::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common {
    #[serde(rename = "ti")]
    pub json_access_web_token_id: String,
    #[serde(rename = "ui")]
    pub application_user_id: String,
    #[serde(rename = "di")]
    pub application_user_log_in_token_device_id: String,
    #[serde(rename = "p")]
    pub exp: String
}

impl<'outer_a> Common {
    pub fn new(json_access_web_token: &'outer_a JsonAccessWebToken<'outer_a>) -> Self {
        return Self {
            application_user_id: json_access_web_token.get_application_user_id().get_value().get_value().to_string(),
            application_user_log_in_token_device_id: json_access_web_token.get_application_user_log_in_token_device_id().get_value().get_value().to_string(),
            json_access_web_token_id: json_access_web_token.get_id().get_value().to_string(),
            exp: json_access_web_token.get_exp().get_value().to_rfc3339()
        };
    }
}