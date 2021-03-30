use serde::Serialize;

#[derive(Serialize)]
pub struct Result {
    #[serde(rename(serialize = "r"))]
    result: bool
}

impl Result {
    pub fn new(result: bool) -> Self {
        return Self {
            result
        };
    }
}