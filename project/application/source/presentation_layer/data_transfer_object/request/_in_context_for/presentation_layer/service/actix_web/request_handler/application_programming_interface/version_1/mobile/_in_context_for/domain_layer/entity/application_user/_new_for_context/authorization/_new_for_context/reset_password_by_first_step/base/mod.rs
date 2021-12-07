use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "aue")]
    application_user_email: String
}

impl Base {
    pub fn into_inner(
        self
    ) -> String {
        return self.application_user_email;
    }
}