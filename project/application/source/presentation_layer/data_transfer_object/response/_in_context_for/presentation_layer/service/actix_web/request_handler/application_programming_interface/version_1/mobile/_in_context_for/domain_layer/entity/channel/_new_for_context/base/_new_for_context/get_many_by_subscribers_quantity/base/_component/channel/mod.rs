use serde::Serialize;

#[derive(Serialize)]
pub struct Channel {
    channel_id: i64,
    channel_subscribers_quantity: i64,
}

impl Channel {
    pub fn new(
        channel_id: i64,
        channel_subscribers_quantity: i64,
    ) -> Self {
        return Self {
            channel_id,
            channel_subscribers_quantity
        };
    }
}