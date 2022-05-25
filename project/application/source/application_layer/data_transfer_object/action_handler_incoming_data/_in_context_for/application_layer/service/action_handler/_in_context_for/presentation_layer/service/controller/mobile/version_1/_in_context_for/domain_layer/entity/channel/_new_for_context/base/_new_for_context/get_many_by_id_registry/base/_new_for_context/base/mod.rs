use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Base {
    json_access_web_token: String,
    channel_id_registry: Vec<i64>,
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, Vec<i64>) {
        return (
            self.json_access_web_token,
            self.channel_id_registry,
        );
    }
}