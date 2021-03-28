use serde::Serialize;

#[derive(Serialize)]
pub struct Result {
    #[serde(rename(serialize = "t"))]
    json_access_web_token: String
}

impl Result {
    pub fn new(json_access_web_token: String) -> Self {
        return Self {
            json_access_web_token
        };
    }
}