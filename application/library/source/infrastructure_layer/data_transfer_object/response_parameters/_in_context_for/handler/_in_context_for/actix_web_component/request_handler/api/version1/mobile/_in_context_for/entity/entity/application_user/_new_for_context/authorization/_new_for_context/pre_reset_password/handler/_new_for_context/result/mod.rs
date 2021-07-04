use serde::Serialize;

#[derive(Serialize)]
pub struct Result {
    #[serde(rename = "i")]
    application_user_id: i64
}

impl Result {
    pub fn new(application_user_id: i64) -> Self {
        return Self {
            application_user_id
        };
    }
}