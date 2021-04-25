use crate::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
pub struct Common<'outer> {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "ui")]
    pub application_user_id: String,
    #[serde(rename = "ue")]
    pub application_user_email: Cow<'outer, str>,
    #[serde(rename = "v")]
    pub value: Cow<'outer, str>,
    #[serde(rename = "a")]
    pub expired_at: String
}

impl<'outer> Common<'outer> {
    pub fn new(application_user_reset_password_token: &'outer ApplicationUserResetPasswordToken<'outer>) -> Self {
        return Self {
            id: application_user_reset_password_token.get_id().get_value().to_string(),
            application_user_id: application_user_reset_password_token.get_application_user_id().get_value().to_string(),
            application_user_email: Cow::Borrowed(application_user_reset_password_token.get_application_user_email().get_value()),
            value: Cow::Borrowed(application_user_reset_password_token.get_value().get_value()),
            expired_at: application_user_reset_password_token.get_expired_at().get_value().to_rfc3339()
        };
    }
}