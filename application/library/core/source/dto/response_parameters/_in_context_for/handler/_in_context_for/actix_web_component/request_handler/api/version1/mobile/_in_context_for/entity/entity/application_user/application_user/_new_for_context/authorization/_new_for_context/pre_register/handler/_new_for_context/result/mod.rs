use serde::Serialize;

#[derive(Serialize)]
pub struct Result {
    #[serde(rename = "i")]
    pre_confirmed_application_user_id: String
}

impl Result {
    pub fn new(pre_confirmed_application_user_id: String) -> Self {
        return Self {
            pre_confirmed_application_user_id: pre_confirmed_application_user_id
        };
    }
}