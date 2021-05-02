use crate::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'outer> {
    #[serde(rename = "ue")]
    pub application_user_email: Cow<'outer, str>,
    #[serde(rename = "v")]
    pub value: Cow<'outer, str>
}

impl<'outer> Common<'outer> {
    pub fn new(application_user_reset_password_token: &'outer ApplicationUserResetPasswordToken<'outer>) -> Self {
        return Self {
            application_user_email: Cow::Borrowed(application_user_reset_password_token.get_application_user_email().get_value()),
            value: Cow::Borrowed(application_user_reset_password_token.get_value().get_value())
        };
    }
}