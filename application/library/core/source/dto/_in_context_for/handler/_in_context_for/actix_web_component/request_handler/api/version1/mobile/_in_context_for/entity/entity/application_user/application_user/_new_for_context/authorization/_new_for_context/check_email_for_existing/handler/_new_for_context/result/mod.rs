use serde::Serialize;

#[derive(Serialize)]
pub struct Result {
    #[serde(rename = "r")]
    result: bool
}

impl Result {
    pub fn new(result: bool) -> Self {
        return Self {
            result
        };
    }
}