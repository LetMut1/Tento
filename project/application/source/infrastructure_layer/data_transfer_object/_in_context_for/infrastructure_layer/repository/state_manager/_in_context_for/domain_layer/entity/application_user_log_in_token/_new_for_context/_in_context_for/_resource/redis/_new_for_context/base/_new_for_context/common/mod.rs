use crate::domain_layer::entity::application_user_log_in_token::ApplicationUserLogInToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'a> {
    application_user_log_in_token_value: Cow<'a, str>,
    application_user_log_in_token_wrong_enter_tries_quantity: u8
}

impl<'a> Common<'a> {
    pub fn new(
        application_user_log_in_token: &'a ApplicationUserLogInToken<'_>
    ) -> Self {
        return Self {
            application_user_log_in_token_value: Cow::Borrowed(application_user_log_in_token.get_value()),
            application_user_log_in_token_wrong_enter_tries_quantity: application_user_log_in_token.get_wrong_enter_tries_quantity()
        };
    }

    pub fn into_inner(
        self
    ) -> (Cow<'a, str>, u8) {
        return (
            self.application_user_log_in_token_value,
            self.application_user_log_in_token_wrong_enter_tries_quantity
        );
    }
}