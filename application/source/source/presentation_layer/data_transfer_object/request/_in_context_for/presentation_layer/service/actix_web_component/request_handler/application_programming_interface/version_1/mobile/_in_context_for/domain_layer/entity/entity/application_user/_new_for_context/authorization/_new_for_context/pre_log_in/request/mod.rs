use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "aulitdi")]
    application_user_log_in_token_device_id: String,
    #[serde(rename = "aue")]
    application_user_email: String,
    #[serde(rename = "aup")]
    application_user_password: String
}

impl Request {
    pub fn into_inner(self) -> (String, String, String) {
        return (
            self.application_user_log_in_token_device_id, 
            self.application_user_email, 
            self.application_user_password
        );
    }
}