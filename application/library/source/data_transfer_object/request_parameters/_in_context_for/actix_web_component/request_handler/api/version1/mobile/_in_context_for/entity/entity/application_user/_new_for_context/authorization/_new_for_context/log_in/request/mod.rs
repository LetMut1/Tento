use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "di")]
    application_user_log_in_token_device_id: String,
    #[serde(rename = "ui")]
    application_user_id: String,
    #[serde(rename = "v")]
    application_user_log_in_token_value: String
}

impl Request {
    pub fn into_inner(self) -> (String, String, String) {
        return (
            self.application_user_log_in_token_device_id, 
            self.application_user_id, 
            self.application_user_log_in_token_value
        );
    }
}