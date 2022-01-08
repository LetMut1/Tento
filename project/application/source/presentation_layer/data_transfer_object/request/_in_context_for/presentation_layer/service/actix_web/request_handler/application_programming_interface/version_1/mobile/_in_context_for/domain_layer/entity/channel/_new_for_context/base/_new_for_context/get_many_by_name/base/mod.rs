use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Base {
    channel_name: String,
    requery_channel_name: Option<String>,
    limit: i8
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, Option<String>, i8) {
        return (
            self.channel_name,
            self.requery_channel_name,
            self.limit
        );
    }
}