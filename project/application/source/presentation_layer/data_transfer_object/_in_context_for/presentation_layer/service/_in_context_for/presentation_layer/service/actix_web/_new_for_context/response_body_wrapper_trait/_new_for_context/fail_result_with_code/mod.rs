use serde::Serialize;

#[derive(Serialize)]
pub struct FailResultWithCode {
    success: bool,
    code: &'static str
}

impl FailResultWithCode {
    pub fn new(
        code: &'static str
    ) -> Self {
        return Self {
            success: false,
            code
        };
    }
}