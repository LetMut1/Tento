use serde::Serialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
pub struct Base {
    application_user_reset_password_token_is_approved: bool
}

impl Base {
    pub fn new(
        application_user_reset_password_token_is_approved: bool
    ) -> Self {
        return Self {
            application_user_reset_password_token_is_approved
        };
    }
}