use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "i")]
    pub application_user_id: String
}