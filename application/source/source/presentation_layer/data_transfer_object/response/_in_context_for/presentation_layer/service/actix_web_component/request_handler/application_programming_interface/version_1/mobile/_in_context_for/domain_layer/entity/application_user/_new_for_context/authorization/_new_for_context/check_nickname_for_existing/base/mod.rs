use serde::Serialize;

#[derive(Serialize)]
pub struct Base {
    #[serde(rename = "r")]
    result: bool
}

impl Base {
    pub fn new(result: bool) -> Self {
        return Self {
            result
        };
    }
}