use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "e")]
    pub application_user_email: String
}