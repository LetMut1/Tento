use crate::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'outer> {
    #[serde(rename = "ui")]
    pub application_user_id: String,
    #[serde(rename = "di")]
    pub device_id: String,
    #[serde(rename = "e")]
    pub application_user_email: Cow<'outer, str>,
    #[serde(rename = "v")]
    pub value: Cow<'outer, str>
}

impl<'outer> Common<'outer> {
    pub fn new(application_user_log_in_token: &'outer ApplicationUserLogInToken<'outer>) -> Self {
        return Self {
            application_user_id: application_user_log_in_token.get_application_user_id().get_value().to_string(),
            device_id: application_user_log_in_token.get_device_id().get_value().to_string(),
            application_user_email: Cow::Borrowed(application_user_log_in_token.get_application_user_email().get_value()),
            value: Cow::Borrowed(application_user_log_in_token.get_value().get_value())
        };
    }
}