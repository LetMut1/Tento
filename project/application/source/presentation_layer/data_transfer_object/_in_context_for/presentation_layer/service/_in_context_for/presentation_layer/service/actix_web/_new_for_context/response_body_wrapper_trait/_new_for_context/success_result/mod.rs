use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResult {
    success: bool
}

impl SuccessResult {
    pub const fn new(
    ) -> Self {
        return Self {
            success: true
        };
    }
}