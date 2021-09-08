use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "csq")]
    channel_subscribers_quantity: Option<u64>,
    #[serde(rename = "o")]
    order: u8,
    #[serde(rename = "l")]
    limit: u8
}

impl Base {
    pub fn into_inner(self) -> (Option<u64>, u8, u8) {
        return (
            self.channel_subscribers_quantity,
            self.order,
            self.limit
        );
    }
}