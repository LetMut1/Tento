use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "i")]
    application_user_id: String
}

impl Request {
    pub fn get_application_user_id(self) -> String {
        return self.application_user_id;
    }
}