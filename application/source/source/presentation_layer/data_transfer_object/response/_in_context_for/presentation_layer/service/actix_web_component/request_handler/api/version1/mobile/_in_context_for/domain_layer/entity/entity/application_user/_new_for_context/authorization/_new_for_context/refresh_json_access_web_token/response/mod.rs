use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    #[serde(rename = "jawt")]
    json_access_web_token: String,
    #[serde(rename = "jrwt")]
    json_refresh_web_token: String
}

impl Response {
    pub fn new(json_access_web_token: String, json_refresh_web_token: String) -> Self {
        return Self {
            json_access_web_token,
            json_refresh_web_token
        };
    }
}