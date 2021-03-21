use serde::Serialize;

#[derive(Serialize)]
pub struct Result {
    #[serde(rename(serialize = "i"))]
    application_user_id: String
}

impl Result {
    pub fn new(application_user_id: String) -> Self {
        return Self {
            application_user_id
        };
    }
}