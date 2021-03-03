use serde::Serialize;

#[derive(Serialize)]
pub struct FailResult {
    success: bool,
    code: &'static str
}

impl FailResult {
    pub fn new(code: &'static str) -> Self {
        return Self {
            success: false,
            code
        };
    }
}