use serde::Serialize;

#[derive(Serialize)]
pub struct Base {
    #[serde(rename = "aui")]
    application_user_id: i64
}

impl Base {
    pub fn new(application_user_id: i64) -> Self {
        return Self {
            application_user_id
        };
    }
}