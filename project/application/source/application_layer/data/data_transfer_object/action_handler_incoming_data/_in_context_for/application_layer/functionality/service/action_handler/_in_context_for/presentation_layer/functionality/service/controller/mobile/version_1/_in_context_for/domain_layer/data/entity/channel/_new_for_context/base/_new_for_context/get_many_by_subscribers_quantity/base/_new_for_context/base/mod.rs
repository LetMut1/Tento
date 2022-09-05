use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    application_user_access_token_web_form: String,
    channel_subscribers_quantity: Option<i64>,
    order: i8,
    limit: i16
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, Option<i64>, i8, i16) {
        return (
            self.application_user_access_token_web_form,
            self.channel_subscribers_quantity,
            self.order,
            self.limit
        );
    }
}