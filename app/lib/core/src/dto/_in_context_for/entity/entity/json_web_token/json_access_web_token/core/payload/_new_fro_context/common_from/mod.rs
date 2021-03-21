use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommonFrom {
    #[serde(rename(deserialize = "ui"))]
    pub application_user_id: String,
    #[serde(rename(deserialize = "di"))]
    pub application_user_log_in_token_device_id: String,
    #[serde(rename(deserialize = "v"))]
    pub json_refresh_web_token_value: String,
    #[serde(rename(deserialize = "e"))]
    pub exp: String
}