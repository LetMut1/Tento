use crate::entity::entity::reset_password_token::reset_password_token::ResetPasswordToken;
use crate::entity::entity::application_user::application_user::core::email::Email;
use crate::error::main_error_kind::core::_in_context_for::utility::email_sender::_new_for_context::email_error_kind::EmailErrorKind;
use crate::utility::email_sender::EmailSender;

pub struct BaseSender;

impl<'outer> BaseSender {
    pub fn send_by_email(reset_password_token: &'outer ResetPasswordToken<'outer>, recipient_email: &'outer Email) -> Result<(), EmailErrorKind> {
        EmailSender::send("Reset password token", "Your code: ".to_string() + reset_password_token.get_value().get_value(), recipient_email.get_value())?;

        return Ok(());
    }
}