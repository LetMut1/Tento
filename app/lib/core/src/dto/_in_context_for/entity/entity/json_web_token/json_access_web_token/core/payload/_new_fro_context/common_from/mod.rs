use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommonFrom {
    pub application_user_id: String,
    pub device_id: String,
    pub json_refresh_web_token_value: String,
    pub exp: String
}