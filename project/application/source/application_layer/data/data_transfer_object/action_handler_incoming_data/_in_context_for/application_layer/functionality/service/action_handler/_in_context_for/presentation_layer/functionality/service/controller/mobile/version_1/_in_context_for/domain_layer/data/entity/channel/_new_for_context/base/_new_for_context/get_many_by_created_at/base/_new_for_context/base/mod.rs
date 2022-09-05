use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    application_user_access_token_web_form: String,
    channel_created_at:  Option<String>,
    order: i8,
    limit: i8
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, Option<String>, i8, i8) {
        return (
            self.application_user_access_token_web_form,
            self.channel_created_at,
            self.order,
            self.limit
        );
    }
}