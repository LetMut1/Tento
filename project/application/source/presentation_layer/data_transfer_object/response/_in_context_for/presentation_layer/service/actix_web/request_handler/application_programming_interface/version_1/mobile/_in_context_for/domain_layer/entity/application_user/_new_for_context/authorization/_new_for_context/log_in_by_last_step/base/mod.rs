use serde::Serialize;

#[derive(Serialize)]
pub struct Base {
    json_access_web_token: String,
    json_refresh_web_token: String
}

impl Base {
    pub fn new(
        json_access_web_token: String,
        json_refresh_web_token: String
    ) -> Self {
        return Self {
            json_access_web_token,
            json_refresh_web_token
        };
    }
}