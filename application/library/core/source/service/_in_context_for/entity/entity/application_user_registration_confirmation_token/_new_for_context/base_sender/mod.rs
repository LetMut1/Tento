use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::error::main_error_kind::core::_in_context_for::utility::email_sender::_new_for_context::email_error_kind::EmailErrorKind;
use crate::utility::email_sender::EmailSender;

pub struct BaseSender;

impl<'outer> BaseSender {
    pub fn send_by_email(
        application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken<'outer>
    ) -> Result<(), EmailErrorKind> {
        EmailSender::send(
            "Registration confirmation", 
            "Your code: ".to_string() + application_user_registration_confirmation_token.get_value().get_value(), 
            application_user_registration_confirmation_token.get_application_user_email().get_value()
        )?;

        return Ok(());
    }
}