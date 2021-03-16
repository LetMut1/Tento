use serde::Serialize;

#[derive(Serialize)]
pub struct HandlerResult {
    jawt: String
}

impl HandlerResult {
    pub fn new(json_access_web_token: String) -> Self {
        return Self {
            jawt: json_access_web_token
        };
    }
}