use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    #[serde(rename = "r")]
    result: bool
}

impl Response {
    pub fn new(result: bool) -> Self {
        return Self {
            result
        };
    }
}