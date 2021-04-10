use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "di")]
    pub application_user_log_in_token_device_id: String,
    #[serde(rename = "ui")]
    pub application_user_id: String,
    #[serde(rename = "v")]
    pub application_user_log_in_token_value: String
}