use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    #[serde(rename = "aue")]
    application_user_email: String
}

impl Query {
    pub fn get_application_user_email(self) -> String {
        return self.application_user_email;
    }
}