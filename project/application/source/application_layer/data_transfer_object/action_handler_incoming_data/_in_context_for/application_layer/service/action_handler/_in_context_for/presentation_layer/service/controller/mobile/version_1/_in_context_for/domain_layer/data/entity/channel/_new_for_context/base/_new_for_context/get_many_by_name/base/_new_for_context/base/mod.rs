use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    json_access_web_token: String,
    channel_name: String,
    requery_channel_name: Option<String>,
    limit: i8
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, String, Option<String>, i8) {
        return (
            self.json_access_web_token,
            self.channel_name,
            self.requery_channel_name,
            self.limit
        );
    }
}