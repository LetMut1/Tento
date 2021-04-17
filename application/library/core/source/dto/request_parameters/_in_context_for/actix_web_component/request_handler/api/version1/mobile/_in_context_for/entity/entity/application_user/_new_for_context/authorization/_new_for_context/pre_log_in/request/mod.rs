use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "i")]
    pub application_user_log_in_token_device_id: String,
    #[serde(rename = "e")]
    pub application_user_email: String,
    #[serde(rename = "p")]
    pub application_user_password: String
}