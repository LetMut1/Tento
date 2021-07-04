use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    #[serde(rename = "i")]
    application_user_id: i64
}

impl Request {
    pub fn get_application_user_id(self) -> i64 {
        return self.application_user_id;
    }
}