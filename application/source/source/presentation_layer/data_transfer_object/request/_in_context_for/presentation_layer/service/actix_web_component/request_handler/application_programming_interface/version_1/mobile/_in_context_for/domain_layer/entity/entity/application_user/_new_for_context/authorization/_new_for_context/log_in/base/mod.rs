use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "aulitdi")]
    application_user_log_in_token_device_id: String,
    #[serde(rename = "aui")]
    application_user_id: i64,
    #[serde(rename = "aulitv")]
    application_user_log_in_token_value: String
}

impl Base {
    pub fn into_inner(self) -> (String, i64, String) {
        return (
            self.application_user_log_in_token_device_id, 
            self.application_user_id, 
            self.application_user_log_in_token_value
        );
    }
}