use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Base {
    json_access_web_token: String,
    json_refresh_web_token: String
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, String) {
        return (
            self.json_access_web_token, 
            self.json_refresh_web_token
        );
    }
}