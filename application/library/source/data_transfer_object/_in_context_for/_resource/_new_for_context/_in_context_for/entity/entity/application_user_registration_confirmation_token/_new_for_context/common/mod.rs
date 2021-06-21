use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'outer_a> {
    #[serde(rename = "e")]
    application_user_email: Cow<'outer_a, str>,
    #[serde(rename = "v")]
    value: Cow<'outer_a, str>,
    #[serde(rename = "q")]
    wrong_enter_tries_quantity: u8
}

impl<'outer_a> Common<'outer_a> {
    pub fn new(application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>) -> Self {
        return Self {
            application_user_email: Cow::Borrowed(application_user_registration_confirmation_token.get_application_user_email().get_value()),
            value: Cow::Borrowed(application_user_registration_confirmation_token.get_value().get_value()),
            wrong_enter_tries_quantity: application_user_registration_confirmation_token.get_wrong_enter_tries_quantity().get_value()
        };
    }

    pub fn into_inner(self) -> (Cow<'outer_a, str>, Cow<'outer_a, str>, u8) {
        return (
            self.application_user_email,
            self.value,
            self.wrong_enter_tries_quantity
        );
    }
}