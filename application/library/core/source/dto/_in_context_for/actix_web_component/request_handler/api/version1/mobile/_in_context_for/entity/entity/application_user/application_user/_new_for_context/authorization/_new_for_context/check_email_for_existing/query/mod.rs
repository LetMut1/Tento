use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    #[serde(rename = "e")]
    pub application_user_email: String
}