use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    #[serde(rename = "i")]
    application_user_id: i64
}

impl Response {
    pub fn new(application_user_id: i64) -> Self {
        return Self {
            application_user_id
        };
    }
}