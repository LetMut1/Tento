use crate::entity::entity::json_web_token::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CommonTo<'outer> {
    #[serde(rename(serialize = "ui"))]
    application_user_id: String,
    #[serde(rename(serialize = "di"))]
    application_user_log_in_token_device_id: String,
    #[serde(rename(serialize = "v"))]
    json_refresh_web_token_value: &'outer str,
    #[serde(rename(serialize = "v"))]
    exp: String
}

impl<'outer> CommonTo<'outer> {
    pub fn new(json_access_web_token: &'outer JsonAccessWebToken<'outer>) -> Self {
        return Self {
            application_user_id: json_access_web_token.get_payload().get_application_user_id().get_value().to_string(),
            application_user_log_in_token_device_id: json_access_web_token.get_payload().get_application_user_log_in_token_device_id().get_value().to_string(),
            json_refresh_web_token_value: json_access_web_token.get_payload().get_json_refresh_web_token_value().get_value(),
            exp: json_access_web_token.get_payload().get_exp().get_value().to_rfc3339()
        };
    }
}