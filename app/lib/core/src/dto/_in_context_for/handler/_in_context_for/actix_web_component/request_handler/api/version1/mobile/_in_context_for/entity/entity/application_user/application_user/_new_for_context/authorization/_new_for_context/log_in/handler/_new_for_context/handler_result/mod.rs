use serde::Serialize;

#[derive(Serialize)]
pub struct HandlerResult {
    #[serde(rename(serialize = "jawt"))]
    json_access_web_token: String
}

impl HandlerResult {
    pub fn new(json_access_web_token: String) -> Self {
        return Self {
            json_access_web_token
        };
    }
}