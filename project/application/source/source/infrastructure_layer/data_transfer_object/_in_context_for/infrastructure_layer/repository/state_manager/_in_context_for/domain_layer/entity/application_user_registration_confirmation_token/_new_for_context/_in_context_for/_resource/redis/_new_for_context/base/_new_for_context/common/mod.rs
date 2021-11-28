use crate::domain_layer::entity::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'a> {
    #[serde(rename = "aue")]
    application_user_email: Cow<'a, str>,
    #[serde(rename = "v")]
    value: Cow<'a, str>,
    #[serde(rename = "wetq")]
    wrong_enter_tries_quantity: Cow<'a, u8>
}

impl<'a> Common<'a> {
    pub fn new(
        application_user_registration_confirmation_token: &'a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Self {
        return Self {
            application_user_email: Cow::Borrowed(application_user_registration_confirmation_token.get_application_user_email()),
            value: Cow::Borrowed(application_user_registration_confirmation_token.get_value()),
            wrong_enter_tries_quantity: Cow::Borrowed(application_user_registration_confirmation_token.get_wrong_enter_tries_quantity())
        };
    }

    pub fn into_inner(
        self
    ) -> (Cow<'a, str>, Cow<'a, str>, Cow<'a, u8>) {
        return (
            self.application_user_email,
            self.value,
            self.wrong_enter_tries_quantity
        );
    }
}