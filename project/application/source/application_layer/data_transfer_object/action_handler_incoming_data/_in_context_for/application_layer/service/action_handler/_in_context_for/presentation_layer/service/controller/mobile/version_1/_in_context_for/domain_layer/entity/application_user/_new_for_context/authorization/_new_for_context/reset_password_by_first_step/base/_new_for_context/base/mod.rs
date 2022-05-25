use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
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