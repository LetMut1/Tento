use serde::Deserialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Serialize;

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
pub struct Base {
    application_user_id: i64,
    application_user_password: String,
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