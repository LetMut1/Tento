use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    #[serde(rename(deserialize = "n"))]
    pub application_user_nickname: String
}