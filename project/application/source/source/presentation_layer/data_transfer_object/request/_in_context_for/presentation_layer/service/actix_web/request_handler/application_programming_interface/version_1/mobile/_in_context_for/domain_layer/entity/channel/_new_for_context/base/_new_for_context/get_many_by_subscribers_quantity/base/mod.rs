use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "csq")]
    channel_subscribers_quantity: Option<i64>,
    #[serde(rename = "o")]
    order: i8,
    #[serde(rename = "l")]
    limit: i16
}

impl Base {
    pub fn into_inner(
        self
    ) -> (Option<i64>, i8, i16) {
        return (
            self.channel_subscribers_quantity,
            self.order,
            self.limit
        );
    }
}