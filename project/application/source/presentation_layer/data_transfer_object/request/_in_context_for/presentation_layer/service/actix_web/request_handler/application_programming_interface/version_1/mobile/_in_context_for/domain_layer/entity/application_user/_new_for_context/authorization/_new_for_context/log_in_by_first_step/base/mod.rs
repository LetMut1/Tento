use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Base {
    application_user_log_in_token_device_id: String,
    application_user_email_or_application_user_nickname: String,
    application_user_password: String
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, String, String) {
        return (
            self.application_user_log_in_token_device_id, 
            self.application_user_email_or_application_user_nickname, 
            self.application_user_password
        );
    }
}