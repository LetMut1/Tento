use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "e")]
    application_user_email: String
}

impl Request {
    pub fn get_application_user_email(self) -> String {
        return self.application_user_email;
    }
}