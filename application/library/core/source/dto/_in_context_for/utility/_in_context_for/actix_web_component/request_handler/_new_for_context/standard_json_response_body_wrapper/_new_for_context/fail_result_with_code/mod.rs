use serde::Serialize;

#[derive(Serialize)]
pub struct FailResultWithCode {
    #[serde(rename = "s")]
    success: bool,
    #[serde(rename = "c")]
    code: &'static str
}

impl FailResultWithCode {
    pub fn new(code: &'static str) -> Self {
        return Self {
            success: false,
            code
        };
    }
}