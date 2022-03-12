use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Base {
    channel_subscribers_quantity: Option<i64>,
    order: i8,
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