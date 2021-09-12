use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "cca")]
    channel_created_at:  Option<String>,
    #[serde(rename = "o")]
    order: i8,
    #[serde(rename = "l")]
    limit: i8
}

impl Base {
    pub fn into_inner(self) -> (Option<String>, i8, i8) {
        return (
            self.channel_created_at,
            self.order,
            self.limit
        );
    }
}