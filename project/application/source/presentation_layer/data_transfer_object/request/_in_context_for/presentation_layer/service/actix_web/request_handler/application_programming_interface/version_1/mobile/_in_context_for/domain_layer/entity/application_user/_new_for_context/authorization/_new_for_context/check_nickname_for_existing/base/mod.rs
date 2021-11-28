use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "aun")]
    application_user_nickname: String
}

impl Base {
    pub fn into_inner(
        self
    ) -> String {
        return self.application_user_nickname;
    }
}