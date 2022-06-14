use crate::domain_layer::data::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize)]
pub struct Common<'a> {
    application_user_registration_confirmation_token_value: Cow<'a, str>,
    application_user_registration_confirmation_token_wrong_enter_tries_quantity: u8
}

impl<'a> Common<'a> {
    pub fn new(
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Self {
        return Self {
            application_user_registration_confirmation_token_value: Cow::Borrowed(application_user_registration_confirmation_token.get_value()),
            application_user_registration_confirmation_token_wrong_enter_tries_quantity: application_user_registration_confirmation_token.get_wrong_enter_tries_quantity()
        };
    }

    pub fn into_inner(
        self
    ) -> (Cow<'a, str>, u8) {
        return (
            self.application_user_registration_confirmation_token_value,
            self.application_user_registration_confirmation_token_wrong_enter_tries_quantity
        );
    }
}