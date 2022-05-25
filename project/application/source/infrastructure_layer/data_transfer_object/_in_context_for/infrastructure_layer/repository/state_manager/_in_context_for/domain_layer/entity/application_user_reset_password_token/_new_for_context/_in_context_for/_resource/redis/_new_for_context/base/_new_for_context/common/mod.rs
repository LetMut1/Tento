use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct Common<'a> {
    application_user_reset_password_token_value: Cow<'a, str>,
    application_user_reset_password_token_wrong_enter_tries_quantity: u8
}

impl<'a> Common<'a> {
    pub fn new(
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken
    ) -> Self {
        return Self {
            application_user_reset_password_token_value: Cow::Borrowed(application_user_reset_password_token.get_value()),
            application_user_reset_password_token_wrong_enter_tries_quantity: application_user_reset_password_token.get_wrong_enter_tries_quantity()
        };
    }

    pub fn into_inner(
        self
    ) -> (Cow<'a, str>, u8) {
        return (
            self.application_user_reset_password_token_value,
            self.application_user_reset_password_token_wrong_enter_tries_quantity
        );
    }
}