use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "cca")]
    channel_created_at: String,
    #[serde(rename = "rcca")]
    requery_channel_created_at: Option<String>,
    #[serde(rename = "o")]
    order: u8,
    #[serde(rename = "l")]
    limit: u8
}

impl Base {
    pub fn into_inner(self) -> (String, Option<String>, u8, u8) {
        return (
            self.channel_created_at,
            self.requery_channel_created_at,
            self.order,
            self.limit
        );
    }
}