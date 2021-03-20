use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename(deserialize = "di"))]
    pub device_id: String,
    #[serde(rename(deserialize = "aui"))]
    pub application_user_id: String
}