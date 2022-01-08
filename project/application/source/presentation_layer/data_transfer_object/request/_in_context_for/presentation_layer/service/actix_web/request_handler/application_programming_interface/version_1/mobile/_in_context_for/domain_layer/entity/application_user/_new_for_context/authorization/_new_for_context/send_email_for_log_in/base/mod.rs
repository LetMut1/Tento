use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Base {
    application_user_log_in_token_device_id: String,
    application_user_id: i64
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, i64) {
        return (
            self.application_user_log_in_token_device_id, 
            self.application_user_id
        );
    }
}