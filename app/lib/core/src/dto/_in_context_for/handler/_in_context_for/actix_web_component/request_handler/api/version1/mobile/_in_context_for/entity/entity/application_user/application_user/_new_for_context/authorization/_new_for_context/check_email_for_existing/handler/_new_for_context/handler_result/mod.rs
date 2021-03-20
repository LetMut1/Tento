use serde::Serialize;

#[derive(Serialize)]
pub struct HandlerResult {
    #[serde(rename(serialize = "r"))]
    result: bool
}

impl HandlerResult {
    pub fn new(result: bool) -> Self {
        return Self {
            result
        };
    }
}