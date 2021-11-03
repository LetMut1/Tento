use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "aui")]
    application_user_id: i64
}

impl Base {
    pub fn get_application_user_id(
        self
    ) -> i64 {
        return self.application_user_id;
    }
}