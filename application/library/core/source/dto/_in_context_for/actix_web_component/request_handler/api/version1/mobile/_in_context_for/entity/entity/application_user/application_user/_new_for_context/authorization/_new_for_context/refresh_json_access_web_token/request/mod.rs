use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename(deserialize = "awt"))]
    pub json_access_web_token: String,
    #[serde(rename(serialize = "rwt"))]
    pub json_refresh_web_token: String
}