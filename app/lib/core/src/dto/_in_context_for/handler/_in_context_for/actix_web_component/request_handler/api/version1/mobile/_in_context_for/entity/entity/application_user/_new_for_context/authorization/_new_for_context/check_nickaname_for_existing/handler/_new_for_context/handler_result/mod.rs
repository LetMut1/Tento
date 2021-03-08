use serde::Serialize;

#[derive(Serialize)]
pub struct HandlerResult {
    result: bool
}

impl HandlerResult {
    pub fn new(result: bool) -> Self {
        return Self {
            result
        };
    }
}