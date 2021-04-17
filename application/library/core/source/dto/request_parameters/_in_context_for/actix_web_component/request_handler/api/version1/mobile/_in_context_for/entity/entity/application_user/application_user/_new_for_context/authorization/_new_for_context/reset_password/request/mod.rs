use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "i")]
    pub application_user_id: String,
    #[serde(rename = "p")]
    pub application_user_password: String,
    #[serde(rename = "v")]
    pub reset_password_token_value: String
}