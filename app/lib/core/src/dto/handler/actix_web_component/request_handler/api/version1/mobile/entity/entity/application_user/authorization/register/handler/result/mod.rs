use serde::Serialize;

#[derive(Serialize)]
pub struct Result {
    result: bool
}

impl Result {
    pub fn new(result: bool) -> Self {
        return Self {
            result
        };
    }
}