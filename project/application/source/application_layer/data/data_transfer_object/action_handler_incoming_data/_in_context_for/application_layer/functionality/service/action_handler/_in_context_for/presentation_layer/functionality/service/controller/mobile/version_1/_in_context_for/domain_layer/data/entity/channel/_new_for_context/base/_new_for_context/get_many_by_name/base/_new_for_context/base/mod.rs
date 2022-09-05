use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    application_user_access_token_web_form: String,
    channel_name: String,
    requery_channel_name: Option<String>,
    limit: i8
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, String, Option<String>, i8) {
        return (
            self.application_user_access_token_web_form,
            self.channel_name,
            self.requery_channel_name,
            self.limit
        );
    }
}