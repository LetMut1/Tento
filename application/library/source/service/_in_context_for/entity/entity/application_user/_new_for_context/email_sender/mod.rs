use crate::entity::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::entity::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::error::main_error_kind::core::run_time_error::run_time_error_kind::RunTimeErrorKind;
use crate::utility::email_sender::EmailSender as BaseEmailSender;

pub struct EmailSender;

impl EmailSender {
    pub fn send_application_user_log_in_token<'outer_a>(
        application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), RunTimeErrorKind> {
        BaseEmailSender::send(
            "Log in confirmation", "Your code: ".to_string() + application_user_log_in_token.get_value().get_value(),
            application_user_log_in_token.get_application_user_email().get_value()
        )?;

        return Ok(());
    }

    pub fn send_application_user_registration_confirmation_token<'outer_a>(
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), RunTimeErrorKind> {
        BaseEmailSender::send(
            "Registration confirmation", 
            "Your code: ".to_string() + application_user_registration_confirmation_token.get_value().get_value(), 
            application_user_registration_confirmation_token.get_application_user_email().get_value()
        )?;

        return Ok(());
    }

    pub fn send_application_user_reset_password_token<'outer_a>(
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), RunTimeErrorKind> {
        BaseEmailSender::send(
            "Reset password confirmation",
            "Your code: ".to_string() + application_user_reset_password_token.get_value().get_value(), 
            application_user_reset_password_token.get_application_user_email().get_value()
        )?;

        return Ok(());
    }
}