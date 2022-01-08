use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Base {
    application_user_id: i64
}

impl Base {
    pub fn new(
        application_user_id: i64
    ) -> Self {
        return Self {
            application_user_id
        };
    }
}