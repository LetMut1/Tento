use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Base {
    json_access_web_token: String,
    channel_created_at:  Option<String>,
    order: i8,
    limit: i8
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, Option<String>, i8, i8) {
        return (
            self.json_access_web_token,
            self.channel_created_at,
            self.order,
            self.limit
        );
    }
}