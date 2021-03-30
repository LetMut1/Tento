use serde::Serialize;

#[derive(Serialize)]
pub struct Result {
    #[serde(rename(serialize = "awt"))]
    json_access_web_token: String,
    #[serde(rename(serialize = "rwt"))]
    json_refresh_web_token: String
}

impl Result {
    pub fn new(json_access_web_token: String, json_refresh_web_token: String) -> Self {
        return Self {
            json_access_web_token,
            json_refresh_web_token
        };
    }
}