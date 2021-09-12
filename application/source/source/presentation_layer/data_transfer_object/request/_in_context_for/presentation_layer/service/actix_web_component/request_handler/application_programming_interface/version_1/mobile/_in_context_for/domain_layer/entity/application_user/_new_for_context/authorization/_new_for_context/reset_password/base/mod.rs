use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "aui")]
    application_user_id: i64,
    #[serde(rename = "aup")]
    application_user_password: String,
    #[serde(rename = "aurptv")]
    application_user_reset_password_token_value: String
}

impl Base {
    pub fn into_inner(
        self
    ) -> (i64, String, String) {
        return (
            self.application_user_id, 
            self.application_user_password,
            self.application_user_reset_password_token_value
        );
    }
}