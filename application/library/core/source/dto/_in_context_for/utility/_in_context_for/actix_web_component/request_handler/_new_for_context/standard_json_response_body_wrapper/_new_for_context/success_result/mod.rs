use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResult {
    #[serde(rename = "s")]
    success: bool,
}

impl SuccessResult {
    pub fn new() -> Self {
        return Self {
            success: true
        };
    }
}