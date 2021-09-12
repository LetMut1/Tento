use crate::domain_layer::entity::application_user_log_in_token::application_user_log_in_token::ApplicationUserLogInToken;
use crate::domain_layer::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::application_user_reset_password_token::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::email_sender_trait::EmailSenderTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::email_sender::EmailSender as BaseEmailSender;

pub struct EmailSender;

impl EmailSenderTrait for EmailSender {     // TODO все &'static str в константы? Тогда пройтись по всему приложению и проверить, везде ли так.
    type Error = BaseError;

    fn send_application_user_log_in_token<'outer_a>(
        application_user_log_in_token: &'outer_a ApplicationUserLogInToken<'_>
    ) -> Result<(), Self::Error> {
        BaseEmailSender::send(
            "Log in confirmation", "Your code: ".to_string() + application_user_log_in_token.get_value(),
            application_user_log_in_token.get_application_user_email()
        )?;

        return Ok(());
    }

    fn send_application_user_registration_confirmation_token<'outer_a>(
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), Self::Error> {
        BaseEmailSender::send(
            "Registration confirmation", 
            "Your code: ".to_string() + application_user_registration_confirmation_token.get_value(), 
            application_user_registration_confirmation_token.get_application_user_email()
        )?;

        return Ok(());
    }

    fn send_application_user_reset_password_token<'outer_a>(
        application_user_reset_password_token: &'outer_a ApplicationUserResetPasswordToken<'_>
    ) -> Result<(), Self::Error> {
        BaseEmailSender::send(
            "Reset password confirmation",
            "Your code: ".to_string() + application_user_reset_password_token.get_value(), 
            application_user_reset_password_token.get_application_user_email()
        )?;

        return Ok(());
    }
}