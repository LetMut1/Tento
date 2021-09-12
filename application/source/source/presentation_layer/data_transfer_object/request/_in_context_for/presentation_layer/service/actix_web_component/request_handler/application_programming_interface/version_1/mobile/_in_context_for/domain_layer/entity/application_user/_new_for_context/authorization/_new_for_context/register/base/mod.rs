use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "aulitdi")]
    application_user_log_in_token_device_id: String,
    #[serde(rename = "aun")]
    application_user_nickname: String,
    #[serde(rename = "aup")]
    application_user_password: String,
    #[serde(rename = "aue")]
    application_user_email: String,
    #[serde(rename = "aurctv")]
    application_user_registration_confirmation_token_value: String
}

impl Base {
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