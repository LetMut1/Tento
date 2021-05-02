use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'outer> {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "ui")]
    pub pre_confirmed_application_user_id: String,
    #[serde(rename = "e")]
    pub application_user_email: Cow<'outer, str>,
    #[serde(rename = "v")]
    pub value: Cow<'outer, str>
}

impl<'outer> Common<'outer> {
    pub fn new(application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken<'outer>) -> Self {
        return Self {
            id: application_user_registration_confirmation_token.get_id().get_value().to_string(),
            pre_confirmed_application_user_id: application_user_registration_confirmation_token.get_pre_confirmed_application_user_id().get_value().to_string(),
            application_user_email: Cow::Borrowed(application_user_registration_confirmation_token.get_application_user_email().get_value()),
            value: Cow::Borrowed(application_user_registration_confirmation_token.get_value().get_value())
        };
    }
}