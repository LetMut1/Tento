use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    application_user_access_token_web_form: String,
    channel_id_registry: Vec<i64>,
}

impl Base {
    pub fn into_inner(
        self
    ) -> (String, Vec<i64>) {
        return (
            self.application_user_access_token_web_form,
            self.channel_id_registry,
        );
    }
}