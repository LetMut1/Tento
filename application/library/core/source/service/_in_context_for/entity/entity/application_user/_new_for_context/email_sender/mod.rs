use crate::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::error::main_error_kind::core::_in_context_for::utility::email_sender::_new_for_context::email_error_kind::EmailErrorKind;
use crate::utility::email_sender::EmailSender as BaseEmailSender;

pub struct EmailSender;

impl<'outer> EmailSender {
    pub fn send_application_user_log_in_token(
        application_user_log_in_token: &'outer ApplicationUserLogInToken<'outer>
    ) -> Result<(), EmailErrorKind> {
        BaseEmailSender::send(
            "Log in confirmation", "Your code: ".to_string() + application_user_log_in_token.get_value().get_value(),
            application_user_log_in_token.get_application_user_email().get_value()
        )?;

        return Ok(());
    }

    pub fn send_application_user_registration_confirmation_token(
        application_user_registration_confirmation_token: &'outer ApplicationUserRegistrationConfirmationToken<'outer>
    ) -> Result<(), EmailErrorKind> {
        BaseEmailSender::send(
            "Registration confirmation", 
            "Your code: ".to_string() + application_user_registration_confirmation_token.get_value().get_value(), 
            application_user_registration_confirmation_token.get_application_user_email().get_value()
        )?;

        return Ok(());
    }

    pub fn send_application_user_reset_password_token(
        application_user_reset_password_token: &'outer ApplicationUserResetPasswordToken<'outer>
    ) -> Result<(), EmailErrorKind> {
        BaseEmailSender::send(
            "Reset password confirmation",
            "Your code: ".to_string() + application_user_reset_password_token.get_value().get_value(), 
            application_user_reset_password_token.get_application_user_email().get_value()
        )?;

        return Ok(());
    }
}