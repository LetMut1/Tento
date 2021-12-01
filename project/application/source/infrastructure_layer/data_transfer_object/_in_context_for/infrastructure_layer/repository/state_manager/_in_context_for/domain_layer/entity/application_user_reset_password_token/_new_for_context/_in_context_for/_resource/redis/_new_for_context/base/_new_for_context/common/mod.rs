use crate::domain_layer::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'a> {
    #[serde(rename = "v")]
    value: Cow<'a, str>,
    #[serde(rename = "wetq")]
    wrong_enter_tries_quantity: Cow<'a, u8>
}

impl<'a> Common<'a> {
    pub fn new(
        application_user_reset_password_token: &'a ApplicationUserResetPasswordToken<'_>
    ) -> Self {
        return Self {
            value: Cow::Borrowed(application_user_reset_password_token.get_value()),
            wrong_enter_tries_quantity: Cow::Borrowed(application_user_reset_password_token.get_wrong_enter_tries_quantity())
        };
    }

    pub fn into_inner(
        self
    ) -> (Cow<'a, str>, Cow<'a, u8>) {
        return (
            self.value,
            self.wrong_enter_tries_quantity
        );
    }
}