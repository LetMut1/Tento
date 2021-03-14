use serde::Serialize;

#[derive(Serialize)]
pub struct HandlerResult {
    result: bool
}

impl HandlerResult {
    pub fn new() -> Self {
        return Self {
            result: true
        };
    }
}