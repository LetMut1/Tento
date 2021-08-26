use crate::domain_layer::entity::entity::json_access_web_token::json_access_web_token::JsonAccessWebToken;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadCommon {
    #[serde(rename = "jawti")]
    json_access_web_token_id: String,
    #[serde(rename = "aui")]
    application_user_id: i64,
    #[serde(rename = "aulitdi")]
    application_user_log_in_token_device_id: String,
    #[serde(rename = "e")]
    exp: String
}

impl PayloadCommon {
    pub fn new<'outer_a>(json_access_web_token: &'outer_a JsonAccessWebToken<'_>) -> Self {
        return Self {
            json_access_web_token_id: json_access_web_token.get_id().get_value().get_value().to_string(),
            application_user_id: json_access_web_token.get_application_user_id().get_value(),
            application_user_log_in_token_device_id: json_access_web_token.get_application_user_log_in_token_device_id().get_value().get_value().to_string(),
            exp: json_access_web_token.get_exp().get_value().get_value().to_string()
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