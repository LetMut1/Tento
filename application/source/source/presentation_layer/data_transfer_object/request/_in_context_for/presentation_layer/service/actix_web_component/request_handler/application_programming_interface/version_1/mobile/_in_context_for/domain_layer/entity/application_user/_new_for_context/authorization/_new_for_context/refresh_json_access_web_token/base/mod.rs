use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "jawt")]
    json_access_web_token: String,
    #[serde(rename = "jrwt")]
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