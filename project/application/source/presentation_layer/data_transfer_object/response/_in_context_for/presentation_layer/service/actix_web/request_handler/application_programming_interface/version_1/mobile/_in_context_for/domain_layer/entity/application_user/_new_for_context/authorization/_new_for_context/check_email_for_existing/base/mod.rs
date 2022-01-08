use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Base {
    result: bool
}

impl Base {
    pub fn new(
        result: bool
    ) -> Self {
        return Self {
            result
        };
    }
}