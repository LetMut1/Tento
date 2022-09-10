use serde::Serialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Deserialize;

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
#[derive(Serialize)]
pub struct Base {
    application_user_access_token_web_form: String,
    application_user_access_refresh_token_web_form: String
}

impl Base {
    pub fn new(
        application_user_access_token_web_form: String,
        application_user_access_refresh_token_web_form: String
    ) -> Self {
        return Self {
            application_user_access_token_web_form,
            application_user_access_refresh_token_web_form
        };
    }
}