use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "ci")]
    channel_id: i64,
}

impl Base {
    pub fn into_inner(
        self
    ) -> i64 {
        return self.channel_id;
    }
}