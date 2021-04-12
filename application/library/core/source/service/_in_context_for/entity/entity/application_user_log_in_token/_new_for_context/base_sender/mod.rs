use crate::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::entity::entity::application_user::application_user::core::email::Email;
use crate::error::main_error_kind::core::_in_context_for::utility::email_sender::_new_for_context::email_error_kind::EmailErrorKind;
use crate::utility::email_sender::EmailSender;

pub struct BaseSender;

impl<'outer> BaseSender {
    pub fn send_by_email(
        application_user_log_in_token: &'outer ApplicationUserLogInToken<'outer>, recipient_email: &'outer Email
    ) -> Result<(), EmailErrorKind> {
        EmailSender::send(
            "Log in confirmation", "Your code: ".to_string() + application_user_log_in_token.get_value().get_value(), recipient_email.get_value()
        )?;

        return Ok(());
    }
}