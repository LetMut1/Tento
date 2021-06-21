use serde::Deserialize;

#[derive(Deserialize)]
pub struct Query {
    #[serde(rename = "n")]
    application_user_nickname: String
}

impl Query {
    pub fn get_application_user_nickname(self) -> String {
        return self.application_user_nickname;
    }
}