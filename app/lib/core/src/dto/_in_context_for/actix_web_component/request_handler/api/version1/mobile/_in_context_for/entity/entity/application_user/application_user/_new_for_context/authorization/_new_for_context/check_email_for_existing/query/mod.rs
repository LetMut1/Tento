use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    #[serde(rename(deserialize = "e"))]
    pub application_user_email: String
}