use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "i")]
    application_user_log_in_token_device_id: String,
    #[serde(rename = "n")]
    application_user_nickname: String,
    #[serde(rename = "p")]
    application_user_password: String,
    #[serde(rename = "e")]
    application_user_email: String,
    #[serde(rename = "v")]
    application_user_registration_confirmation_token_value: String
}

impl Request {
    pub fn into_inner(self) -> (String, String, String, String, String) {
        return (
            self.application_user_log_in_token_device_id, 
            self.application_user_nickname,
            self.application_user_password,
            self.application_user_email,
            self.application_user_registration_confirmation_token_value
        );
    }
}