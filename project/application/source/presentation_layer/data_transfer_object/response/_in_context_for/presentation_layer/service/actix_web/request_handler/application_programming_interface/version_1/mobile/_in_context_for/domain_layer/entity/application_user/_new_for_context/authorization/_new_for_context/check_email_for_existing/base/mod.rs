use serde::Serialize;

#[derive(Serialize)]
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