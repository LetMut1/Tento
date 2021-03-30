use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename(deserialize = "i"))]
    pub application_user_log_in_token_device_id: String,
    #[serde(rename(deserialize = "n"))]
    pub application_user_nickname: String,
    #[serde(rename(deserialize = "p"))]
    pub application_user_password: String,
    #[serde(rename(deserialize = "e"))]
    pub application_user_email: String,
    #[serde(rename(deserialize = "v"))]
    pub application_user_registration_confirmation_token_value: String
}