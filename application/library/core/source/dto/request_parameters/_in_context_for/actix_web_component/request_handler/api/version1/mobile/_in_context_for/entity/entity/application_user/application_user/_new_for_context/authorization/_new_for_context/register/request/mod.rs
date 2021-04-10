use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "i")]
    pub application_user_log_in_token_device_id: String,
    #[serde(rename = "n")]
    pub application_user_nickname: String,
    #[serde(rename = "p")]
    pub application_user_password: String,
    #[serde(rename = "e")]
    pub application_user_email: String,
    #[serde(rename = "v")]
    pub application_user_registration_confirmation_token_value: String
}