use serde::Serialize;

#[derive(Serialize)]
pub struct Result {
    jawt: String
}

impl Result {
    pub fn new(json_access_web_token: String) -> Self {
        return Self {
            jawt: json_access_web_token
        };
    }
}