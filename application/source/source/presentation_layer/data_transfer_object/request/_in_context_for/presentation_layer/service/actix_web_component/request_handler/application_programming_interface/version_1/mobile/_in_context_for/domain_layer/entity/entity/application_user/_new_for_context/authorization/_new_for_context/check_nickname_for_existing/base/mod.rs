use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "aun")]
    application_user_nickname: String
}

impl Base {
    pub fn get_application_user_nickname(self) -> String {
        return self.application_user_nickname;
    }
}