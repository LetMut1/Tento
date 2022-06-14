use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    json_access_web_token: String,
    channel_subscribers_quantity: Option<i64>,
    order: i8,
    limit: i16
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, Option<i64>, i8, i16) {
        return (
            self.json_access_web_token,
            self.channel_subscribers_quantity,
            self.order,
            self.limit
        );
    }
}