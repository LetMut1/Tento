use serde::Deserialize;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
use serde::Serialize;

#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Serialize))]
#[derive(Deserialize)]
pub struct Base {
    application_user_email: String
}

impl Base {
    pub fn into_inner(
        self
    ) -> String {
        return self.application_user_email;
    }
}