use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "cn")]
    channel_name: String,
    #[serde(rename = "rcn")]
    requery_channel_name: Option<String>,
    #[serde(rename = "l")]
    limit: i8
}

impl Base {
    pub fn into_inner(self) -> (String, Option<String>, i8) {
        return (
            self.channel_name,
            self.requery_channel_name,
            self.limit
        );
    }
}