use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'outer_a> {
    #[serde(rename = "aue")]
    application_user_email: Cow<'outer_a, str>,
    #[serde(rename = "v")]
    value: Cow<'outer_a, str>,
    #[serde(rename = "wetq")]
    wrong_enter_tries_quantity: Cow<'outer_a, u8>
}

impl<'outer_a> Common<'outer_a> {
    pub fn new(
        application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Self {
        return Self {
            application_user_email: Cow::Borrowed(application_user_log_in_token.get_application_user_email()),
            value: Cow::Borrowed(application_user_log_in_token.get_value()),
            wrong_enter_tries_quantity: Cow::Borrowed(application_user_log_in_token.get_wrong_enter_tries_quantity())
        };
    }

    pub fn into_inner(
        self
    ) -> (Cow<'outer_a, str>, Cow<'outer_a, str>, Cow<'outer_a, u8>) {
        return (
            self.application_user_email,
            self.value,
            self.wrong_enter_tries_quantity
        );
    }
}