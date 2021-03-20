use serde::Serialize;

#[derive(Serialize)]
pub struct HandlerResult {
    #[serde(rename(serialize = "aui"))]
    application_user_id: String
}

impl HandlerResult {
    pub fn new(application_user_id: String) -> Self {
        return Self {
            application_user_id
        };
    }
}